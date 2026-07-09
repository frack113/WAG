// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum TraceIdentifierError {
    #[error("trace identifier must have the shape `<domain>.<family>.<behavior>`, got `{input}`")]
    MalformedIdentifier { input: String },

    #[error(
        "trace identifier segment `{segment}` must start with a lowercase letter and contain only lowercase ASCII, digits, or underscore"
    )]
    InvalidSegment { segment: String },
}
