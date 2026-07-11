// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod error;
pub mod types;

pub use crate::metadata::{
    error::{AttackTechniqueError, TraceIdentifierError},
    types::{AttackTechnique, TraceIdentifier},
};
