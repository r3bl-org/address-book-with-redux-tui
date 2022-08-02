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
use r3bl_rs_utils::{fire_and_forget, redux::AsyncMiddlewareSpawns, utils::print_prompt};
use tokio::task::JoinHandle;

use crate::{json_rpc::{fake_contact_data_api::make_request as fake_contact_data_api,
                       FakeContactData},
            Action,
            Mw,
            State,
            Std,
            PROMPT_STR};

#[derive(Default)]
pub struct AddAsyncCmdMw;

#[allow(clippy::all)]
#[async_trait]
impl AsyncMiddlewareSpawns<State, Action> for AddAsyncCmdMw {
  async fn run(&self, action: Action, _state: State) -> JoinHandle<Option<Action>> {
    fire_and_forget![{
      match action {
        Action::Mw(Mw::AsyncAddCmd) => {
          let fake_data = fake_contact_data_api()
            .await
            .unwrap_or_else(|_| FakeContactData {
              name: "Foo Bar".to_string(),
              phone_h: "123-456-7890".to_string(),
              email_u: "foo".to_string(),
              email_d: "bar.com".to_string(),
              ..FakeContactData::default()
            });

          let action = Action::Std(Std::AddContact(
            format!("{}", fake_data.name),
            format!("{}@{}", fake_data.email_u, fake_data.email_d),
            format!("{}", fake_data.phone_h),
          ));

          print_prompt(PROMPT_STR).unwrap();

          return Some(action);
        }
        _ => return None,
      }
    }]
  }
}
