use super::super::error::VhdlError;
use super::super::highlight;
use super::super::token::char_set;
use crate::core::lang::vhdl::token::ToColor;
use colored::ColoredString;
use colored::Colorize;
use std::fmt::Display;
use std::str::FromStr;

/// Transforms a VHDL integer `s` into a real unsigned number to be used in rust code.
///
/// Assumes the integer is valid under the following production rule:
/// - integer ::= digit { \[ underline ] digit }
pub fn interpret_integer(s: &str) -> usize {
    let mut chars = s.chars();
    let mut number = String::from(chars.next().expect("must have a lead-off digit"));
    while let Some(c) = chars.next() {
        if c != char_set::UNDERLINE {
            number.push(c);
        }
    }
    number
        .parse::<usize>()
        .expect("integer can only contain 0..=9 or underline '_'")
}

#[derive(Debug, PartialEq, Clone)]
pub struct Character(pub String);

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BitStrLiteral(pub String);

impl Display for BitStrLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AbstLiteral {
    Decimal(String),
    Based(String),
}

impl Display for AbstLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Decimal(val) => val,
                Self::Based(val) => val,
            }
        )
    }
}

impl ToColor for Character {
    fn to_color(&self) -> ColoredString {
        let crayon = highlight::CHARS;
        self.to_string().truecolor(crayon.0, crayon.1, crayon.2)
    }
}

impl ToColor for BitStrLiteral {
    fn to_color(&self) -> ColoredString {
        let crayon = highlight::NUMBERS;
        self.to_string().truecolor(crayon.0, crayon.1, crayon.2)
    }
}

impl ToColor for AbstLiteral {
    fn to_color(&self) -> ColoredString {
        let crayon = highlight::NUMBERS;
        self.to_string().truecolor(crayon.0, crayon.1, crayon.2)
    }
}

pub mod based_integer {
    /// Transforms the base `n` into its character validiation function.
    ///
    /// The output is used to verify extended digits in a VHDL based_literal token.
    pub fn as_fn(n: usize) -> fn(c: &char) -> bool {
        match n {
            2 => is_base_2,
            3 => is_base_3,
            4 => is_base_4,
            5 => is_base_5,
            6 => is_base_6,
            7 => is_base_7,
            8 => is_base_8,
            9 => is_base_9,
            10 => is_base_10,
            11 => is_base_11,
            12 => is_base_12,
            13 => is_base_13,
            14 => is_base_14,
            15 => is_base_15,
            16 => is_base_16,
            _ => panic!("base `n` must be at least 2 and at most 16"),
        }
    }

    pub fn is_base_2(c: &char) -> bool {
        match c {
            '0'..='1' => true,
            _ => false,
        }
    }
    pub fn is_base_3(c: &char) -> bool {
        match c {
            '0'..='2' => true,
            _ => false,
        }
    }
    pub fn is_base_4(c: &char) -> bool {
        match c {
            '0'..='3' => true,
            _ => false,
        }
    }
    pub fn is_base_5(c: &char) -> bool {
        match c {
            '0'..='4' => true,
            _ => false,
        }
    }
    pub fn is_base_6(c: &char) -> bool {
        match c {
            '0'..='5' => true,
            _ => false,
        }
    }
    pub fn is_base_7(c: &char) -> bool {
        match c {
            '0'..='6' => true,
            _ => false,
        }
    }
    pub fn is_base_8(c: &char) -> bool {
        match c {
            '0'..='7' => true,
            _ => false,
        }
    }
    pub fn is_base_9(c: &char) -> bool {
        match c {
            '0'..='8' => true,
            _ => false,
        }
    }
    pub fn is_base_10(c: &char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }
    pub fn is_base_11(c: &char) -> bool {
        match c {
            '0'..='9' | 'a'..='a' | 'A'..='A' => true,
            _ => false,
        }
    }
    pub fn is_base_12(c: &char) -> bool {
        match c {
            '0'..='9' | 'a'..='b' | 'A'..='B' => true,
            _ => false,
        }
    }
    pub fn is_base_13(c: &char) -> bool {
        match c {
            '0'..='9' | 'a'..='c' | 'A'..='C' => true,
            _ => false,
        }
    }
    pub fn is_base_14(c: &char) -> bool {
        match c {
            '0'..='9' | 'a'..='d' | 'A'..='D' => true,
            _ => false,
        }
    }
    pub fn is_base_15(c: &char) -> bool {
        match c {
            '0'..='9' | 'a'..='e' | 'A'..='E' => true,
            _ => false,
        }
    }
    pub fn is_base_16(c: &char) -> bool {
        match c {
            '0'..='9' | 'a'..='f' | 'A'..='F' => true,
            _ => false,
        }
    }
}

/// Set: B | O | X | UB | UO | UX | SB | SO | SX | D
#[derive(Debug, PartialEq)]
pub enum BaseSpec {
    B,
    O,
    X,
    UB,
    UO,
    UX,
    SB,
    SO,
    SX,
    D,
}

impl FromStr for BaseSpec {
    type Err = VhdlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "b" | "B" => Self::B,
            "o" | "O" => Self::O,
            "x" | "X" => Self::X,
            "ub" | "uB" | "Ub" | "UB" => Self::UB,
            "uo" | "uO" | "Uo" | "UO" => Self::UO,
            "ux" | "uX" | "Ux" | "UX" => Self::UX,
            "sb" | "sB" | "Sb" | "SB" => Self::SB,
            "so" | "sO" | "So" | "SO" => Self::SO,
            "sx" | "sX" | "Sx" | "SX" => Self::SX,
            "d" | "D" => Self::D,
            _ => return Err(Self::Err::Any(format!("invalid base specifier '{}'", s))),
        })
    }
}

impl BaseSpec {
    fn as_str(&self) -> &str {
        match self {
            Self::B => "b",
            Self::O => "o",
            Self::X => "x",
            Self::UB => "ub",
            Self::UO => "uo",
            Self::UX => "ux",
            Self::SB => "sb",
            Self::SO => "so",
            Self::SX => "sx",
            Self::D => "d",
        }
    }
}
