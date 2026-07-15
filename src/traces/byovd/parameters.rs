// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Parser;
use std::path::PathBuf;
use toml_span::{DeserError, Deserialize, Value, de_helpers::TableHelper};

#[derive(Parser)]
pub(super) struct ByovdParameters {
    #[clap(required = true, help = "Path to the driver")]
    pub(super) driver: PathBuf,
    #[clap(required = true, help = "Name of the driver")]
    pub(super) name: String,
    #[clap(required = true, help = "Display name of the driver service")]
    pub(super) display_name: String,
}

impl<'de> Deserialize<'de> for ByovdParameters {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let driver = table.required::<String>("driver").map(PathBuf::from);
        let name = table.required("name");
        let display_name = table.required("display_name");
        table.finalize(None)?;

        Ok(Self {
            driver: driver.expect("required field is missing"),
            name: name.expect("required field is missing"),
            display_name: display_name.expect("required field is missing"),
        })
    }
}
