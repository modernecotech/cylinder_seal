//! QR generation (merchant presents) + QR decoding from webcam (merchant
//! scans customer's signed payload).

use anyhow::{Context, Result};
use image::{DynamicImage, Luma};
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use qrcode::{EcLevel, QrCode};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Render a payment-request payload as a PNG bitmap.
pub fn render_qr_png(payload: &str, target_px: u32) -> Result<Vec<u8>> {
    let code = QrCode::with_error_correction_level(payload.as_bytes(), EcLevel::M)
        .context("build QR code")?;
    let image = code
        .render::<Luma<u8>>()
        .min_dimensions(target_px, target_px)
        .quiet_zone(true)
        .build();

    let dynamic = DynamicImage::ImageLuma8(image);
    let mut buf = std::io::Cursor::new(Vec::new());
    dynamic
        .write_to(&mut buf, image::ImageFormat::Png)
        .context("encode PNG")?;
    Ok(buf.into_inner())
}

/// Render a QR payload straight into a slint::Image. This is the form the
/// UI layer wants — a buffer of RGBA pixels with known dimensions.
pub fn render_qr_to_slint_image(payload: &str, target_px: u32) -> Result<slint::Image> {
    let code = QrCode::with_error_correction_level(payload.as_bytes(), EcLevel::M)
        .context("build QR code")?;
    let rendered = code
        .render::<Luma<u8>>()
        .min_dimensions(target_px, target_px)
        .quiet_zone(true)
        .build();

    let (w, h) = rendered.dimensions();
    let mut rgba = Vec::with_capacity((w * h * 4) as usize);
    for pixel in rendered.pixels() {
        let v = pixel.0[0];
        rgba.extend_from_slice(&[v, v, v, 255]);
    }

    let pixel_buf = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
        &rgba, w, h,
    );
    Ok(slint::Image::from_rgba8(pixel_buf))
}

/// Background QR-scanner that reads frames from the first available camera
/// and pushes decoded QR payloads through a channel. Returns a receiver
/// end; dropping it stops the thread.
pub fn spawn_scanner(camera_index: u32) -> Result<mpsc::Receiver<String>> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        if let Err(e) = run_scanner(camera_index, tx) {
            tracing::warn!("camera scanner ended: {e:?}");
        }
    });
    Ok(rx)
}

fn run_scanner(camera_index: u32, tx: mpsc::Sender<String>) -> Result<()> {
    let index = CameraIndex::Index(camera_index);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    let mut camera = Camera::new(index, requested).context("open camera")?;
    camera.open_stream().context("open stream")?;

    loop {
        match camera.frame() {
            Ok(frame) => {
                if let Ok(image) = frame.decode_image::<RgbFormat>() {
                    // Convert to grayscale for rqrr.
                    let gray = DynamicImage::ImageRgb8(image).to_luma8();
                    let mut grid = rqrr::PreparedImage::prepare(gray);
                    for g in grid.detect_grids() {
                        if let Ok((_meta, content)) = g.decode() {
                            if tx.send(content).is_err() {
                                return Ok(());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                tracing::debug!("camera frame error: {e}");
                thread::sleep(Duration::from_millis(200));
            }
        }
    }
}
