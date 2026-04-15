// CylinderSeal Exchange/Currency Service
// Manages OWC rate feeds and currency conversion.
// All conversions are at the real interbank rate — zero spread, zero fees.

pub mod feed_aggregator;
pub mod spread;

pub use feed_aggregator::*;
