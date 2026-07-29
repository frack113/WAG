// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use thiserror::Error;
use toml_span::{DeserError, Deserialize, Error, ErrorKind, span::Spanned, value::Value};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceIdentifier {
    domain: String,
    family: String,
    behavior: String,
}

impl TraceIdentifier {
    pub fn as_str(&self) -> String {
        format!("{}.{}.{}", self.domain, self.family, self.behavior)
    }
}

impl Display for TraceIdentifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
#[error(
    "trace identifier must be `<domain>.<family>.<behavior>` where each segment starts with a lowercase letter and contains only lowercase ASCII, digits, or underscore, got `{input}`"
)]
pub struct ParseTraceIdentifierError {
    input: String,
}

impl FromStr for TraceIdentifier {
    type Err = ParseTraceIdentifierError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = input.split('.').collect();

        let [domain, family, behavior] = segments.as_slice() else {
            return Err(ParseTraceIdentifierError {
                input: input.to_string(),
            });
        };

        for segment in [domain, family, behavior] {
            if segment.is_empty()
                || segment
                    .starts_with(|character: char| character.is_ascii_digit() || character == '_')
                || !segment.chars().all(|character| {
                    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
                })
            {
                return Err(ParseTraceIdentifierError {
                    input: input.to_string(),
                });
            }
        }

        Ok(Self {
            domain: (*domain).to_string(),
            family: (*family).to_string(),
            behavior: (*behavior).to_string(),
        })
    }
}

impl<'de> Deserialize<'de> for TraceIdentifier {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let raw = Spanned::<String>::deserialize(value)?;

        raw.value.parse::<Self>().map_err(|error| {
            Error {
                kind: ErrorKind::Custom(error.to_string().into()),
                span: raw.span,
                line_info: None,
            }
            .into()
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttackTechnique {
    main: String,
    sub: Option<String>,
}

impl AttackTechnique {
    pub fn as_str(&self) -> String {
        match &self.sub {
            Some(sub) => format!("T{}.{}", self.main, sub),
            None => format!("T{}", self.main),
        }
    }
}

impl Display for AttackTechnique {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        format.write_str(&self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
#[error(
    "ATT&CK technique must be `T` followed by 4 digits, optionally `.3 digits` for sub-technique, got `{input}`"
)]
pub struct ParseAttackTechniqueError {
    input: String,
}

impl FromStr for AttackTechnique {
    type Err = ParseAttackTechniqueError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some(after_prefix) = input.strip_prefix('T') else {
            return Err(ParseAttackTechniqueError {
                input: input.to_string(),
            });
        };

        let Some((main, sub)) = after_prefix.split_once('.') else {
            if !(after_prefix.len() == 4
                && after_prefix
                    .chars()
                    .all(|character| character.is_ascii_digit()))
            {
                return Err(ParseAttackTechniqueError {
                    input: input.to_string(),
                });
            }

            return Ok(Self {
                main: after_prefix.to_string(),
                sub: None,
            });
        };

        let main_valid =
            main.len() == 4 && main.chars().all(|character| character.is_ascii_digit());
        let sub_valid = sub.len() == 3 && sub.chars().all(|character| character.is_ascii_digit());

        if !main_valid || !sub_valid {
            return Err(ParseAttackTechniqueError {
                input: input.to_string(),
            });
        }

        Ok(Self {
            main: main.to_string(),
            sub: Some(sub.to_string()),
        })
    }
}

impl<'de> Deserialize<'de> for AttackTechnique {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let raw = Spanned::<String>::deserialize(value)?;

        raw.value.parse::<Self>().map_err(|error| {
            Error {
                kind: ErrorKind::Custom(error.to_string().into()),
                span: raw.span,
                line_info: None,
            }
            .into()
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigmaIdentifier(Uuid);

impl Display for SigmaIdentifier {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        format.write_str(&self.0.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
#[error("Sigma identifier must be a valid UUID v4, got `{input}`")]
pub struct ParseSigmaIdentifierError {
    input: String,
}

impl FromStr for SigmaIdentifier {
    type Err = ParseSigmaIdentifierError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(input)
            .ok()
            .filter(|identifier| identifier.get_version_num() == 4)
            .map(Self)
            .ok_or_else(|| ParseSigmaIdentifierError {
                input: input.to_string(),
            })
    }
}

impl<'de> Deserialize<'de> for SigmaIdentifier {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let raw = Spanned::<String>::deserialize(value)?;

        raw.value.parse::<Self>().map_err(|error| {
            Error {
                kind: ErrorKind::Custom(error.to_string().into()),
                span: raw.span,
                line_info: None,
            }
            .into()
        })
    }
}
