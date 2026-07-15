// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod execution;
mod parameters;

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{TraceDefinition, TraceMetadata},
};
use parameters::SpoofingParameters;
use std::sync::LazyLock;

pub(crate) const IDENTIFIER: &str = "process.spoofing.create";

pub static DEFINITION: LazyLock<TraceDefinition> = LazyLock::new(|| {
    TraceDefinition::new::<SpoofingParameters>(
        TraceMetadata {
            identifier: IDENTIFIER
                .parse::<TraceIdentifier>()
                .expect("trace identifier must be valid"),
            name: "Spoofing",
            description: "Creates a process with a spoofed parent process.",
            attack_techniques: vec![
                "T1134.004"
                    .parse::<AttackTechnique>()
                    .expect("ATT&CK technique must be valid"),
            ],
            sigma_identifiers: vec![
                "a05efb77-9c1d-4384-806e-080ca13946fe"
                    .parse::<SigmaIdentifier>()
                    .expect("Sigma identifier must be valid"),
            ],
        },
        execution::execute,
    )
});
