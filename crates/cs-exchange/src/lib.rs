// CylinderSeal Exchange/Currency Service
// Manages OWC rate feeds, currency conversion, and CBI reference data.
// All conversions are at the real interbank rate — zero spread, zero fees.

pub mod cbi;
pub mod feed_aggregator;
pub mod spread;

pub use cbi::*;
pub use feed_aggregator::*;
