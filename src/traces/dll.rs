// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod execution;
mod parameters;

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{TraceDefinition, TraceMetadata},
};
use parameters::DllParameters;
use std::sync::LazyLock;

pub(crate) const IDENTIFIER: &str = "memory.dll.load";

pub static DEFINITION: LazyLock<TraceDefinition> = LazyLock::new(|| {
    TraceDefinition::new::<DllParameters>(
        TraceMetadata {
            identifier: IDENTIFIER
                .parse::<TraceIdentifier>()
                .expect("trace identifier must be valid"),
            name: "DLL Loader",
            description: "Loads a DLL into the current process.",
            attack_techniques: vec![
                "T1129"
                    .parse::<AttackTechnique>()
                    .expect("ATT&CK technique must be valid"),
            ],
            sigma_identifiers: vec![
                "2a4052f7-858e-412e-be8c-60138c8ce031"
                    .parse::<SigmaIdentifier>()
                    .expect("Sigma identifier must be valid"),
            ],
        },
        execution::execute,
    )
});
