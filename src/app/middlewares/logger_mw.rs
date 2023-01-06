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

use async_trait::async_trait;
use r3bl_rs_utils::{print_header, redux::AsyncMiddleware};
use rand::Rng;

use crate::{app::{DELAY_ENABLED, MAX_DELAY, MIN_DELAY},
            Action,
            State};

#[derive(Default)]
pub struct LoggerMw;

#[async_trait]
impl AsyncMiddleware<State, Action> for LoggerMw {
  async fn run(&self, action: Action, _state: State) -> Option<Action> {
    if DELAY_ENABLED {
      // Artificial delay before calling the function.
      let delay_ms = rand::thread_rng().gen_range(MIN_DELAY..MAX_DELAY);
      std::thread::sleep(tokio::time::Duration::from_millis(delay_ms));
    }
    println!();
    print_header("logger_mw");
    println!("action: {action:?}");
    None
  }
}
