// SPDX-License-Identifier: GPL-3.0-or-later
//! Leptos views for PomoTime
//!
//! This crate contains the views for PomoTime.

pub mod app;
mod timer;
mod footer; 
mod main_content;
mod header;

pub use app::*;
use timer::*;
use footer::*;
use main_content::*;
use header::*;