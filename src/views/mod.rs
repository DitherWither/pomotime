// SPDX-License-Identifier: GPL-3.0-or-later
//! Leptos views for PomoTime
//!
//! This crate contains the views for PomoTime.

pub mod app;
mod footer;
mod header;
mod main_content;
mod timer;

pub use app::*;
use footer::*;
use header::*;
use main_content::*;
use timer::*;
