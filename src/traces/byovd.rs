// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod execution;
mod parameters;

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{TraceDefinition, TraceMetadata},
};
use parameters::ByovdParameters;
use std::sync::LazyLock;

pub(crate) const IDENTIFIER: &str = "driver.byovd.load";

pub static DEFINITION: LazyLock<TraceDefinition> = LazyLock::new(|| {
    TraceDefinition::new::<ByovdParameters>(
        TraceMetadata {
            identifier: IDENTIFIER
                .parse::<TraceIdentifier>()
                .expect("trace identifier must be valid"),
            name: "Bring Your Own Vulnerable Driver",
            description: "Loads a kernel driver through the Windows service manager.",
            attack_techniques: vec![
                "T1068"
                    .parse::<AttackTechnique>()
                    .expect("ATT&CK technique must be valid"),
            ],
            sigma_identifiers: vec![
                "b5d44a2e-31c9-4e4f-8a4f-cc18633f2146"
                    .parse::<SigmaIdentifier>()
                    .expect("Sigma identifier must be valid"),
            ],
        },
        execution::execute,
    )
});
