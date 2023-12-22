#![macro_use]
#![allow(missing_docs)] // TODO: Add documentation

#[cfg(can_bxcan)]
mod bxcan;
#[cfg(any(can_fdcan_v1, can_fdcan_h7))]
mod fdcan;

#[cfg(can_bxcan)]
pub use bxcan::*;
#[cfg(any(can_fdcan_v1, can_fdcan_h7))]
pub use fdcan::*;
