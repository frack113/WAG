// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use std::path::PathBuf;
use toml_span::{DeserError, Deserialize, Value, de_helpers::TableHelper};

#[derive(Parser)]
pub(super) struct SpoofingParameters {
    #[clap(required = true, help = "Path to the executable")]
    pub(super) executable: PathBuf,
    #[clap(required = true, help = "Name of the parent executable")]
    pub(super) parent_executable: String,
}

impl<'de> Deserialize<'de> for SpoofingParameters {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let executable = table.required::<String>("executable").map(PathBuf::from);
        let parent_executable = table.required("parent_executable");
        table.finalize(None)?;

        Ok(Self {
            executable: executable.expect("required field is missing"),
            parent_executable: parent_executable.expect("required field is missing"),
        })
    }
}
