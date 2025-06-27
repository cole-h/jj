// Copyright 2025 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::cli_util::CommandHelper;
use crate::command_error::CommandError;
use crate::ui::Ui;

/// Create a noop operation
#[derive(clap::Args, Clone, Debug)]
pub struct OperationNoopArgs {
    /// The optional message to associate with the noop operation
    #[arg(long = "message", short, value_name = "MESSAGE")]
    message: Option<String>,
}

pub fn cmd_op_noop(
    ui: &mut Ui,
    command: &CommandHelper,
    args: &OperationNoopArgs,
) -> Result<(), CommandError> {
    let msg = if let Some(msg) = &args.message {
        &format!("noop: {msg}")
    } else {
        "noop"
    };

    let mut workspace_command = command.workspace_helper(ui)?;
    let tx = workspace_command.start_transaction();
    tx.noop(msg)?;

    Ok(())
}
