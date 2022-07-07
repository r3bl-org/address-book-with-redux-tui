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
use r3bl_rs_utils::{redux::AsyncMiddleware, style_error, utils::print_prompt};

use crate::{json_rpc::get_ip_api::make_request as get_ip_api, Action, Mw, State, PROMPT_STR};

#[derive(Default)]
pub struct IpCmdMw;

#[async_trait]
impl AsyncMiddleware<State, Action> for IpCmdMw {
  async fn run(&self, action: Action, _state: State) -> Option<Action> {
    if let Action::Mw(Mw::IpCmd) = action {
      match get_ip_api().await {
        Ok(resp_data) => {
          println!("{}", resp_data);
          print_prompt(PROMPT_STR).unwrap();
        }
        Err(e) => println!("{}", style_error(&e.to_string())),
      };
    }
    None
  }
}
