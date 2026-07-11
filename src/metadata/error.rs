// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum TraceIdentifierError {
    #[error("trace identifier must be `<domain>.<family>.<behavior>` where each segment starts with a lowercase letter and contains only lowercase ASCII, digits, or underscore, got `{input}`")]
    MalformedIdentifier { input: String },
}

}
