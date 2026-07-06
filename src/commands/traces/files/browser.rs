// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    commands::traces::{Trace, TraceMetadata},
    displayer::Displayer,
};
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::{fs, path::PathBuf, process::ExitCode};

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

pub const METADATA: TraceMetadata = TraceMetadata {
    identifier: "file.browser.steal",
    name: "Browser Stealer",
    requirements_summary: "browser profile directory",
    attack_techniques: &["T1560"],
    sigma_targets: &["browser"],
    use_cases: &["browser credential access"],
};

impl Trace for Browser {
    fn run(&self) -> ExitCode {
        let mut displayer = Displayer::new();
        displayer.loading("Verifying the browser profile path");

        if !self.profile.exists() {
            displayer.failure(&format!(
                "Browser profile path does not exist: {}",
                self.profile.display()
            ));

            return ExitCode::FAILURE;
        }

        if !self.profile.is_dir() {
            displayer.failure(&format!(
                "Browser profile path is not a directory: {}",
                self.profile.display()
            ));

            return ExitCode::FAILURE;
        }

        displayer.success("The browser profile path is verified");
        displayer.loading("Accessing browser files");

        let files = self.browser.files();

        for file in files {
            let path = self.profile.join(file);

            if !path.exists() {
                continue;
            }

            match fs::read(&path) {
                Ok(_) => {
                    displayer.success(&format!("The file {} is accessed", file));
                }
                Err(error) => {
                    displayer.failure(&format!("Failed to access the file {}: {}", file, error));
                }
            }
        }

        displayer.success("The browser files are accessed");

        ExitCode::SUCCESS
    }
}
