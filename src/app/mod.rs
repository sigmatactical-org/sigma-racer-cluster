//! Application lifecycle helpers for the cluster binary.

mod boot;
mod clock;
mod idle;

pub use boot::schedule_hide;
pub use clock::start as start_clock;
pub use idle::push as push_idle;
