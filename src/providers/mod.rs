//! Provider implementations for national statistics agencies.

pub mod abs;
pub mod auckland_museum;
pub mod data_gov_au;
pub mod digitalnz;
pub mod stats_nz;
#[cfg(test)]
mod test_support;

pub use abs::AbsProvider;
pub use auckland_museum::AucklandMuseumProvider;
pub use data_gov_au::DataGovAuProvider;
pub use digitalnz::DigitalNzProvider;
pub use stats_nz::StatsNzProvider;
