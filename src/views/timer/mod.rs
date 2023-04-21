//! This module contains the views for the timer.
//!
//! This is a collection of views that are used to display and change the status of a timer.

pub mod duration_changer;
pub mod status_changer;
pub mod status_display;

pub use duration_changer::*;
pub use status_changer::*;
pub use status_display::*;
