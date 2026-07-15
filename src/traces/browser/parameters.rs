// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use toml_span::{
    DeserError, Deserialize, Error, ErrorKind, Spanned, Value, de_helpers::TableHelper,
};

#[derive(Parser)]
pub(super) struct BrowserParameters {
    #[clap(value_enum, help = "Browser to target")]
    pub(super) browser: Browser,
    #[clap(help = "Path to the browser profile directory")]
    pub(super) profile: PathBuf,
}

impl<'de> Deserialize<'de> for BrowserParameters {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let mut table = TableHelper::new(value)?;
        let browser = table.required("browser");
        let profile = table.required::<String>("profile").map(PathBuf::from);
        table.finalize(None)?;

        Ok(Self {
            browser: browser.expect("required field is missing"),
            profile: profile.expect("required field is missing"),
        })
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(super) enum Browser {
    Firefox,
    Chrome,
    Edge,
    Opera,
    Brave,
}

impl<'de> Deserialize<'de> for Browser {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let browser = Spanned::<String>::deserialize(value)?;

        match browser.value.as_str() {
            "firefox" => Ok(Self::Firefox),
            "chrome" => Ok(Self::Chrome),
            "edge" => Ok(Self::Edge),
            "opera" => Ok(Self::Opera),
            "brave" => Ok(Self::Brave),
            _ => Err(Error {
                kind: ErrorKind::Custom(
                    "expected one of: firefox, chrome, edge, opera, brave".into(),
                ),
                span: browser.span,
                line_info: None,
            }
            .into()),
        }
    }
}
