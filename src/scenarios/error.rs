// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ScenarioIdentifierError {
    #[error(
        "scenario identifier must contain 1 to 128 lowercase ASCII letters or digits separated by single hyphens, got `{input}`"
    )]
    MalformedIdentifier { input: String },
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum TagError {
    #[error(
        "tag must contain 1 to 128 lowercase ASCII letters or digits separated by single hyphens, got `{input}`"
    )]
    MalformedTag { input: String },
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum UrlError {
    #[error("URL must use http or https and include a host, got `{input}`")]
    MalformedUrl { input: String },
}
