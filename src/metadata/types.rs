// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::metadata::error::{AttackTechniqueError, TraceIdentifierError};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
use toml_span::{DeserError, Deserialize, Error, ErrorKind, span::Spanned, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub struct TraceIdentifier {
    domain: String,
    family: String,
    behavior: String,
}

impl TraceIdentifier {
    pub fn new(
        domain: impl Into<String>,
        family: impl Into<String>,
        behavior: impl Into<String>,
    ) -> Self {
        Self {
            domain: domain.into(),
            family: family.into(),
            behavior: behavior.into(),
        }
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn behavior(&self) -> &str {
        &self.behavior
    }

    pub fn as_str(&self) -> String {
        format!("{}.{}.{}", self.domain, self.family, self.behavior)
    }
}

impl Display for TraceIdentifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_str())
    }
}

impl FromStr for TraceIdentifier {
    type Err = TraceIdentifierError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = input.split('.').collect();

        let [domain, family, behavior] = segments.as_slice() else {
            return Err(TraceIdentifierError::MalformedIdentifier {
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
                return Err(TraceIdentifierError::MalformedIdentifier {
                    input: input.to_string(),
                });
            }
        }

        Ok(Self::new(*domain, *family, *behavior))
    }
}

impl<'de> Deserialize<'de> for TraceIdentifier {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let raw = <Spanned<String> as Deserialize<'de>>::deserialize(value)?;

        raw.value.parse().map_err(|error| {
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
    pub fn new(main: impl Into<String>, sub: Option<impl Into<String>>) -> Self {
        Self {
            main: main.into(),
            sub: sub.map(Into::into),
        }
    }

    pub fn main(&self) -> &str {
        &self.main
    }

    pub fn sub(&self) -> Option<&str> {
        self.sub.as_deref()
    }

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

impl FromStr for AttackTechnique {
    type Err = AttackTechniqueError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some(after_prefix) = input.strip_prefix('T') else {
            return Err(AttackTechniqueError::MalformedTechnique {
                input: input.to_string(),
            });
        };

        let Some((main, sub)) = after_prefix.split_once('.') else {
            if !(after_prefix.len() == 4
                && after_prefix
                    .chars()
                    .all(|character| character.is_ascii_digit()))
            {
                return Err(AttackTechniqueError::MalformedTechnique {
                    input: input.to_string(),
                });
            }

            return Ok(Self::new(after_prefix, None::<&str>));
        };

        let main_valid =
            main.len() == 4 && main.chars().all(|character| character.is_ascii_digit());
        let sub_valid = sub.len() == 3 && sub.chars().all(|character| character.is_ascii_digit());

        if !main_valid || !sub_valid {
            return Err(AttackTechniqueError::MalformedTechnique {
                input: input.to_string(),
            });
        }

        Ok(Self::new(main, Some(sub)))
    }
}

impl<'de> Deserialize<'de> for AttackTechnique {
    fn deserialize(value: &mut Value<'de>) -> Result<Self, DeserError> {
        let raw = <Spanned<String> as Deserialize<'de>>::deserialize(value)?;

        raw.value.parse().map_err(|error| {
            Error {
                kind: ErrorKind::Custom(error.to_string().into()),
                span: raw.span,
                line_info: None,
            }
            .into()
        })
    }
}
