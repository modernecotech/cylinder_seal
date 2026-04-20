//! Location coarsening for privacy.
//!
//! Transaction `latitude`/`longitude` are useful for anti-fraud (sudden
//! jumps between geographies indicate a stolen credential or a relay attack)
//! but raw GPS coordinates on every transaction reveal a surveillance-grade
//! movement graph to the super-peer network.
//!
//! The compromise: round to ~1 km buckets (0.01° ≈ 1.11 km at the equator,
//! less near the poles). This preserves fraud utility — detecting a Baghdad
//! wallet suddenly signing in Erbil — without revealing which mosque, market,
//! or street the user is standing on.
//!
//! Accuracy is similarly rounded to the nearest 100 m, which preserves the
//! "is this plausibly a GPS fix vs. a too-broad network fix" signal without
//! fingerprinting specific sensor hardware.

/// Coarsen (lat, lon) to the nearest 0.01° bucket (~1.1 km at the equator,
/// tighter near the poles). Rounds half to even via `f64::round`.
///
/// A sentinel `(0.0, 0.0)` — used by offline transactions with no location —
/// passes through unchanged so downstream code can still distinguish
/// "no location provided" from "location at the equator/prime meridian
/// intersection".
pub fn coarsen_to_1km(lat: f64, lon: f64) -> (f64, f64) {
    if lat == 0.0 && lon == 0.0 {
        return (0.0, 0.0);
    }
    let lat_c = (lat * 100.0).round() / 100.0;
    let lon_c = (lon * 100.0).round() / 100.0;
    (lat_c, lon_c)
}

/// Round GPS accuracy (meters) to the nearest 100 m. Values <= 0 pass
/// through unchanged (they indicate "not available", not a rounded zero).
pub fn coarsen_accuracy(accuracy_meters: i32) -> i32 {
    if accuracy_meters <= 0 {
        return accuracy_meters;
    }
    ((accuracy_meters as f64 / 100.0).round() * 100.0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sentinel_zero_passes_through() {
        assert_eq!(coarsen_to_1km(0.0, 0.0), (0.0, 0.0));
    }

    #[test]
    fn baghdad_rounds_to_0_01_bucket() {
        // Al-Jadriyah, Baghdad — a specific street corner.
        let (lat, lon) = coarsen_to_1km(33.2738, 44.3849);
        assert!((lat - 33.27).abs() < 1e-9);
        assert!((lon - 44.38).abs() < 1e-9);
    }

    #[test]
    fn different_buildings_same_block_bucket_same() {
        // Two coordinates well inside the same 0.01° cell (centred on
        // 33.28, 44.38) round to the same bucket.
        let (a_lat, a_lon) = coarsen_to_1km(33.2761, 44.3829);
        let (b_lat, b_lon) = coarsen_to_1km(33.2788, 44.3840);
        assert_eq!(a_lat, b_lat);
        assert_eq!(a_lon, b_lon);
    }

    #[test]
    fn neighbouring_buckets_remain_distinct() {
        // Baghdad vs. Erbil should clearly differ.
        let (blat, _) = coarsen_to_1km(33.3152, 44.3661);
        let (elat, _) = coarsen_to_1km(36.1911, 44.0090);
        assert!((blat - elat).abs() > 2.0);
    }

    #[test]
    fn negative_coords_round_correctly() {
        let (lat, lon) = coarsen_to_1km(-1.2864, 36.8172);
        assert!((lat - -1.29).abs() < 1e-9);
        assert!((lon - 36.82).abs() < 1e-9);
    }

    #[test]
    fn accuracy_rounds_to_100m() {
        assert_eq!(coarsen_accuracy(17), 0);
        assert_eq!(coarsen_accuracy(49), 0);
        assert_eq!(coarsen_accuracy(50), 100);
        assert_eq!(coarsen_accuracy(149), 100);
        assert_eq!(coarsen_accuracy(150), 200);
        assert_eq!(coarsen_accuracy(2000), 2000);
    }

    #[test]
    fn accuracy_not_available_passes_through() {
        assert_eq!(coarsen_accuracy(0), 0);
        assert_eq!(coarsen_accuracy(-1), -1);
    }
}
