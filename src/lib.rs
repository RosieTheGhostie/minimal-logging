#[cfg(feature = "attributes")]
extern crate minimal_logging_attributes;
#[cfg(feature = "macros")]
extern crate minimal_logging_macros;

#[cfg(feature = "attributes")]
pub mod attributes {
    pub use minimal_logging_attributes::*;
}

#[cfg(feature = "macros")]
pub mod macros {
    pub use minimal_logging_macros::*;
}
