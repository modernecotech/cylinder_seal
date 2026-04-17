//! ESC/POS receipt printer driver.
//!
//! Supports three transports:
//!   - USB thermal printer (e.g. Xprinter XP-58III) via /dev/usb/lp*
//!   - Serial (e.g. RS232-attached kiosk printers) via /dev/ttyUSB0
//!   - Network (IP-connected printers) via 9100/TCP
//!
//! The `disabled` kind quietly drops every print — useful for dev and for
//! terminals without a printer attached.

use anyhow::{Context, Result};
use std::io::Write;

use crate::config::PrinterConfig;

pub struct Receipt<'a> {
    pub merchant_name: &'a str,
    pub amount_micro_owc: i64,
    pub currency: &'a str,
    pub transaction_id: &'a str,
    pub memo: &'a str,
    pub timestamp_utc: i64,
}

pub fn print(cfg: &PrinterConfig, receipt: &Receipt) -> Result<()> {
    match cfg.kind.as_str() {
        "disabled" => Ok(()),
        "usb" => print_to_file(&cfg.target, receipt, cfg.width_chars),
        "serial" => print_to_file(&cfg.target, receipt, cfg.width_chars),
        "network" => print_to_tcp(&cfg.target, receipt, cfg.width_chars),
        other => anyhow::bail!("unknown printer kind: {other}"),
    }
}

fn build_receipt(receipt: &Receipt, width_chars: u32) -> Vec<u8> {
    let width = width_chars.max(24) as usize;
    let mut out = Vec::new();
    // ESC @ — initialize
    out.extend_from_slice(&[0x1B, 0x40]);
    // ESC a 1 — center
    out.extend_from_slice(&[0x1B, 0x61, 0x01]);
    let header = pad_center(receipt.merchant_name, width);
    out.extend_from_slice(header.as_bytes());
    out.push(b'\n');
    let div = "-".repeat(width);
    out.extend_from_slice(div.as_bytes());
    out.push(b'\n');

    // ESC a 0 — left
    out.extend_from_slice(&[0x1B, 0x61, 0x00]);

    let amount_str = format_micro_owc(receipt.amount_micro_owc, receipt.currency);
    out.extend_from_slice(format!("Amount: {amount_str}\n").as_bytes());
    out.extend_from_slice(
        format!(
            "Time:   {}\n",
            chrono::DateTime::from_timestamp_micros(receipt.timestamp_utc)
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "?".into())
        )
        .as_bytes(),
    );
    out.extend_from_slice(format!("Tx:     {}\n", receipt.transaction_id).as_bytes());
    if !receipt.memo.is_empty() {
        out.extend_from_slice(format!("Memo:   {}\n", receipt.memo).as_bytes());
    }
    out.extend_from_slice(div.as_bytes());
    out.push(b'\n');
    out.extend_from_slice(b"Digital Iraqi Dinar\n");
    out.extend_from_slice(b"Paid via CylinderSeal\n\n\n\n");

    // GS V 1 — partial cut
    out.extend_from_slice(&[0x1D, 0x56, 0x01]);
    out
}

fn print_to_file(path: &str, receipt: &Receipt, width: u32) -> Result<()> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .open(path)
        .with_context(|| format!("open printer device {path}"))?;
    let bytes = build_receipt(receipt, width);
    f.write_all(&bytes).context("write receipt")?;
    Ok(())
}

fn print_to_tcp(endpoint: &str, receipt: &Receipt, width: u32) -> Result<()> {
    let mut stream = std::net::TcpStream::connect(endpoint)
        .with_context(|| format!("connect printer {endpoint}"))?;
    let bytes = build_receipt(receipt, width);
    stream.write_all(&bytes).context("write receipt")?;
    Ok(())
}

fn pad_center(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s[..width].to_string();
    }
    let pad = (width - s.len()) / 2;
    format!("{}{}", " ".repeat(pad), s)
}

fn format_micro_owc(micro: i64, currency: &str) -> String {
    let whole = micro / 1_000_000;
    let frac = (micro % 1_000_000).abs();
    format!("{whole}.{:06} {currency}", frac)
}
