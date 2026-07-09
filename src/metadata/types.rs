// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::metadata::error::TraceIdentifierError;
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

    fn from_str(input: &str) -> Result<Self, Self::Error> {
        let segments: Vec<&str> = input.split('.').collect();

        if segments.len() != 3 {
            return Err(TraceIdentifierError::MalformedIdentifier {
                input: input.to_string(),
            });
        }

        for segment in &segments {
            if segment.is_empty()
                || segment
                    .starts_with(|character: char| character.is_ascii_digit() || character == '_')
                || !segment.chars().all(|character| {
                    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
                })
            {
                return Err(TraceIdentifierError::InvalidSegment {
                    segment: (*segment).to_string(),
                });
            }
        }

        Ok(Self::new(segments[0], segments[1], segments[2]))
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
