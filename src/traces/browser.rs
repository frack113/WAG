// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod execution;
mod parameters;

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{TraceDefinition, TraceMetadata},
};
use parameters::BrowserParameters;
use std::sync::LazyLock;

pub(crate) const IDENTIFIER: &str = "file.browser.steal";

pub static DEFINITION: LazyLock<TraceDefinition> = LazyLock::new(|| {
    TraceDefinition::new::<BrowserParameters>(
        TraceMetadata {
            identifier: IDENTIFIER
                .parse::<TraceIdentifier>()
                .expect("trace identifier must be valid"),
            name: "Browser Stealer",
            description: "Reads credential and history files from a browser profile.",
            attack_techniques: vec![
                "T1555.003"
                    .parse::<AttackTechnique>()
                    .expect("ATT&CK technique must be valid"),
            ],
            sigma_identifiers: vec![
                "9796aae8-9ee3-4a26-8b6f-66bf9a0ee864"
                    .parse::<SigmaIdentifier>()
                    .expect("Sigma identifier must be valid"),
            ],
        },
        execution::execute,
    )
});
