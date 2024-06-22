mod env_settings;
mod trusted_setup_points;

// TODO: remove when we have `portable` feature in `c-kzg`
use blst as _;

#[cfg(not(feature = "kzg-rs"))]
pub use c_kzg::KzgSettings;
#[cfg(feature = "kzg-rs")]
pub use kzg_rs::KzgSettings;

pub use env_settings::EnvKzgSettings;
pub use trusted_setup_points::{
    parse_kzg_trusted_setup, G1Points, G2Points, KzgErrors, BYTES_PER_G1_POINT, BYTES_PER_G2_POINT,
    G1_POINTS, G2_POINTS, NUM_G1_POINTS, NUM_G2_POINTS,
};
