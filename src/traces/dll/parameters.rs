// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use std::path::PathBuf;
use toml_span::{DeserError, Deserialize, Value, de_helpers::TableHelper};

#[derive(Parser)]
pub(super) struct DllParameters {
    #[clap(help = "Path to the DLL file")]
    pub(super) path: PathBuf,
}

impl<'de> Deserialize<'de> for DllParameters {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let path = table.required::<String>("path").map(PathBuf::from);
        table.finalize(None)?;

        Ok(Self {
            path: path.expect("required field is missing"),
        })
    }
}
