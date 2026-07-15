// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{TraceDefinition, browser, byovd, dll, spoofing},
};
use std::sync::LazyLock;

pub static REGISTRY: Registry = Registry;

static TRACE_DEFINITIONS: LazyLock<[&'static TraceDefinition; 4]> = LazyLock::new(|| {
    [
        LazyLock::force(&byovd::DEFINITION),
        LazyLock::force(&dll::DEFINITION),
        LazyLock::force(&spoofing::DEFINITION),
        LazyLock::force(&browser::DEFINITION),
    ]
});

pub struct Registry;

impl Registry {
    pub fn all(&self) -> &'static [&'static TraceDefinition] {
        &*TRACE_DEFINITIONS
    }

    pub fn get(&self, identifier: &TraceIdentifier) -> Option<&'static TraceDefinition> {
        self.all()
            .iter()
            .find(|definition| &definition.metadata.identifier == identifier)
            .copied()
    }

    pub fn query(&self, filters: &TraceFilters) -> Vec<&'static TraceDefinition> {
        self.all()
            .iter()
            .copied()
            .filter(|definition| filters.matches(definition))
            .collect()
    }
}

pub struct TraceFilters<'a> {
    pub attack_techniques: &'a [AttackTechnique],
    pub sigma_identifiers: &'a [SigmaIdentifier],
}

impl TraceFilters<'_> {
    fn matches(&self, definition: &TraceDefinition) -> bool {
        let attack_matches = self.attack_techniques.is_empty()
            || self
                .attack_techniques
                .iter()
                .any(|filter| definition.metadata.attack_techniques.contains(filter));

        let sigma_matches = self.sigma_identifiers.is_empty()
            || self
                .sigma_identifiers
                .iter()
                .any(|filter| definition.metadata.sigma_identifiers.contains(filter));

        attack_matches && sigma_matches
    }
}
