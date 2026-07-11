// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum TraceIdentifierError {
    #[error(
        "trace identifier must be `<domain>.<family>.<behavior>` where each segment starts with a lowercase letter and contains only lowercase ASCII, digits, or underscore, got `{input}`"
    )]
    MalformedIdentifier { input: String },
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum AttackTechniqueError {
    #[error(
        "ATT&CK technique must be `T` followed by 4 digits, optionally `.3 digits` for sub-technique, got `{input}`"
    )]
    MalformedTechnique { input: String },
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum SigmaIdentifierError {
    #[error("Sigma identifier must be a valid UUID, got `{input}`")]
    MalformedIdentifier { input: String },
}
