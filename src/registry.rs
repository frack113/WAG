// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{
    metadata::{AttackTechnique, SigmaIdentifier, TraceIdentifier},
    traces::{TraceMetadata, browser, byovd, dll, spoofing},
};
use std::sync::LazyLock;

pub static REGISTRY: Registry = Registry;

static REGISTERED_TRACES: LazyLock<Vec<&'static TraceMetadata>> = LazyLock::new(|| {
    vec![
        &byovd::METADATA,
        &dll::METADATA,
        &spoofing::METADATA,
        &browser::METADATA,
    ]
});

pub struct Registry;

impl Registry {
    pub fn all(&self) -> &'static [&'static TraceMetadata] {
        REGISTERED_TRACES.as_slice()
    }

    pub fn get(&self, identifier: &TraceIdentifier) -> Option<&'static TraceMetadata> {
        self.all()
            .iter()
            .find(|metadata| &metadata.identifier == identifier)
            .copied()
    }

    pub fn query(&self, filters: &TraceFilters) -> Vec<&'static TraceMetadata> {
        self.all()
            .iter()
            .copied()
            .filter(|metadata| filters.matches(metadata))
            .collect()
    }
}

pub struct TraceFilters<'a> {
    pub attack_techniques: &'a [AttackTechnique],
    pub sigma_identifiers: &'a [SigmaIdentifier],
    pub use_cases: &'a [String],
}

impl TraceFilters<'_> {
    fn matches(&self, metadata: &TraceMetadata) -> bool {
        let attack_matches = self.attack_techniques.is_empty()
            || self
                .attack_techniques
                .iter()
                .any(|filter| metadata.attack_techniques.contains(filter));

        let sigma_matches = self.sigma_identifiers.is_empty()
            || self
                .sigma_identifiers
                .iter()
                .any(|filter| metadata.sigma_identifiers.contains(filter));

        let use_case_matches = self.use_cases.is_empty()
            || self
                .use_cases
                .iter()
                .any(|filter| metadata.use_cases.contains(filter));

        attack_matches && sigma_matches && use_case_matches
    }
}
