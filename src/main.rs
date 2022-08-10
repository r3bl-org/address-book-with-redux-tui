/*
 *   Copyright (c) 2022 R3BL LLC
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

// https://github.com/rust-lang/rust-clippy
// https://rust-lang.github.io/rust-clippy/master/index.html
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

// Connect to source files.
pub mod address_book;
pub mod json_rpc;
pub mod tui;

// Re-exports.
use std::{env::args, process::exit};

pub use address_book::*;
pub use json_rpc::*;
// Imports.
use r3bl_rs_utils::{style_error,
                    style_primary,
                    utils::{call_if_err, with, ArgsToStrings}};
use tui::run_tui_app;
pub use tui::*;

#[tokio::main]
async fn main() {
  with(
    run_tui_app(args().filter_and_convert_to_strings()),
    |result| async {
      call_if_err(&result.await, &|err| {
        eprintln!("{}: {}", style_error("Problem encountered"), err);
        exit(1);
      });
      println!("{}", style_primary("Goodbye."));
      exit(0);
    },
  )
  .await;
}
