use super::super::super::lexer::TrainCar;
use crate::core::lang::vhdl::token::ToColor;
use crate::core::lang::LangIdentifier;
use crate::core::pkgid::PkgPart;
use crate::util::strcmp;
use colored::ColoredString;
use colored::Colorize;
use serde_derive::Serialize;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use std::hash::Hasher;
use std::str::FromStr;

use crate::core::lang::vhdl::token::char_set;
use crate::core::lang::vhdl::token::VhdlToken;

#[derive(Debug, Clone, PartialOrd, Ord, Serialize)]
#[serde(untagged)]
pub enum Identifier {
    Basic(String),
    Extended(String),
}

impl std::cmp::Eq for Identifier {}

impl Identifier {
    pub fn into_lang_id(self) -> LangIdentifier {
        LangIdentifier::Vhdl(self)
    }

    /// Creates an empty basic identifier.
    pub fn new() -> Self {
        Self::Basic(String::new())
    }

    /// Creates a new basic identifier for the working library: `work`.
    pub fn new_working() -> Self {
        Self::Basic(String::from("work"))
    }

    // Returns the reference to the inner `String` struct.
    fn as_str(&self) -> &str {
        match self {
            Self::Basic(id) => id.as_ref(),
            Self::Extended(id) => id.as_ref(),
        }
    }

    /// Modifies the ending of the identifier with `ext` and writes as a String
    pub fn into_extension(&self, ext: &str) -> Identifier {
        match self {
            Self::Basic(s) => Self::Basic(s.clone() + ext),
            Self::Extended(s) => Self::Extended(s.clone() + ext),
        }
    }

    /// Checks if `self` is an extended identifier or not.
    fn is_extended(&self) -> bool {
        match self {
            Self::Extended(_) => true,
            Self::Basic(_) => false,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Basic(id) => id.len(),
            Self::Extended(id) => id.len() + 2 + (id.chars().filter(|c| c == &'\\').count()),
        }
    }
}

// @todo: test
impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Basic(id) => id.to_lowercase().hash(state),
            Self::Extended(id) => id.hash(state),
        }
    }
}

impl From<&PkgPart> for Identifier {
    fn from(part: &PkgPart) -> Self {
        Identifier::Basic(part.to_normal().to_string())
    }
}

#[derive(Debug, PartialEq)]
pub enum IdentifierError {
    Empty,
    InvalidFirstChar(char),
    CharsAfterDelimiter(String),
}

impl std::error::Error for IdentifierError {}

impl std::fmt::Display for IdentifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "empty identifier"),
            Self::CharsAfterDelimiter(s) => write!(
                f,
                "characters \'{}\' found following closing extended backslash, ",
                s
            ),
            Self::InvalidFirstChar(c) => {
                write!(f, "first character must be letter but found \'{}\'", c)
            }
        }
    }
}

impl FromStr for Identifier {
    type Err = IdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = TrainCar::new(s.chars());
        match chars.consume() {
            // check what type of identifier it is
            Some(c) => Ok(match c {
                '\\' => {
                    let result = Self::Extended(
                        VhdlToken::consume_literal(&mut chars, &char_set::BACKSLASH).unwrap(),
                    );
                    // gather remaining characters
                    let mut rem = String::new();
                    while let Some(c) = chars.consume() {
                        rem.push(c);
                    }
                    match rem.is_empty() {
                        true => result,
                        false => return Err(Self::Err::CharsAfterDelimiter(rem)),
                    }
                }
                _ => {
                    // verify the first character was a letter
                    match char_set::is_letter(&c) {
                        true => Self::Basic(
                            VhdlToken::consume_value_pattern(
                                &mut chars,
                                Some(c),
                                char_set::is_letter_or_digit,
                            )
                            .unwrap(),
                        ),
                        false => return Err(Self::Err::InvalidFirstChar(c)),
                    }
                }
            }),
            None => Err(Self::Err::Empty),
        }
    }
}

impl std::cmp::PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        // instantly not equal if not they are not of same type
        if self.is_extended() != other.is_extended() {
            return false;
        };
        // compare with case sensitivity
        if self.is_extended() == true {
            self.as_str() == other.as_str()
        // compare without case sensitivity
        } else {
            strcmp::cmp_ignore_case(self.as_str(), other.as_str())
        }
    }

    fn ne(&self, other: &Self) -> bool {
        self.eq(other) == false
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic(id) => write!(f, "{}", id),
            Self::Extended(id) => write!(f, "\\{}\\", id.replace('\\', r#"\\"#)),
        }
    }
}

impl ToColor for Identifier {
    fn to_color(&self) -> ColoredString {
        self.to_string().normal()
    }
}
