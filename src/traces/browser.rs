// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{Trace, TraceMetadata, TraceParameter, TraceParameterKind},
};
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::{fs, path::PathBuf, process::ExitCode, sync::LazyLock};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, ValueEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BrowserKind {
    Firefox,
    Chrome,
    Edge,
    Opera,
    Brave,
}

impl BrowserKind {
    fn files(&self) -> Vec<&'static str> {
        match self {
            BrowserKind::Firefox => vec![
                "places.sqlite",
                "cookies.sqlite",
                "logins.json",
                "key3.db",
                "key4.db",
                "formhistory.sqlite",
                "favicons.sqlite",
            ],
            BrowserKind::Chrome => vec![
                "History",
                "Cookies",
                "Login Data",
                "Bookmarks",
                "Web Data",
                "Preferences",
                "Top Sites",
                "Favicons",
            ],
            BrowserKind::Edge => vec![
                "History",
                "Cookies",
                "Login Data",
                "Bookmarks",
                "Web Data",
                "Preferences",
            ],
            BrowserKind::Opera => vec!["History", "Cookies", "Login Data", "Bookmarks", "Wand.dat"],
            BrowserKind::Brave => vec!["History", "Cookies", "Login Data", "Bookmarks", "Web Data"],
        }
    }
}

#[derive(Parser, Deserialize)]
pub struct Browser {
    #[clap(value_enum, help = "Browser to target")]
    browser: BrowserKind,
    #[clap(help = "Path to the browser profile directory")]
    profile: PathBuf,
}

pub static METADATA: LazyLock<TraceMetadata> = LazyLock::new(|| TraceMetadata {
    identifier: TraceIdentifier::new("file", "browser", "steal"),
    name: "Browser Stealer",
    requirements_summary: "browser profile directory",
    attack_techniques: vec![AttackTechnique::new("1560", None::<&str>)],
    sigma_identifiers: vec![
        "9796aae8-9ee3-4a26-8b6f-66bf9a0ee864"
            .parse::<SigmaIdentifier>()
            .unwrap(),
    ],
    use_cases: vec!["browser credential access".to_string()],
    decode_parameters: decode_parameters::<Browser>,
});

impl Trace for Browser {
    fn run(&self) -> ExitCode {
        if !self.profile.exists() || !self.profile.is_dir() {
            return ExitCode::FAILURE;
        }

        for file in self.browser.files() {
            let path = self.profile.join(file);

            if path.exists() {
                let _ = fs::read(path);
            }
        }

        ExitCode::SUCCESS
    }
}
