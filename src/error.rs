use std::fmt::Formatter;
use time::error::{ComponentRange, ParseFromDescription};

impl std::error::Error for Error {}
impl std::error::Error for ConversionError {}
impl std::error::Error for ParsingError {}
impl std::error::Error for ParsedError {}

#[derive(Debug)]
pub enum ConversionError {
    OutOfRange(ComponentRange),
}

impl From<ComponentRange> for ConversionError {
    fn from(value: ComponentRange) -> Self {
        Self::OutOfRange(value)
    }
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfRange(err) => write!(f, "out of range: {err}"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ParsingError {
    /// After parsing the input, information is missing
    MissingInformation,
    /// The input was not fully parsed
    RemainingInformation,
    /// The input was not fully parsed
    Format(ParseFromDescription),
}

impl From<ParseFromDescription> for ParsingError {
    fn from(value: ParseFromDescription) -> Self {
        Self::Format(value)
    }
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingInformation => write!(f, "missing information"),
            Self::RemainingInformation => write!(f, "remaining input"),
            Self::Format(err) => write!(f, "format error: {err}"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ParsedError {
    /// The format is unknown
    UnknownFormat,
}

impl std::fmt::Display for ParsedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownFormat => write!(f, "unknown format"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Conversion(ConversionError),
    // Parsing(ParsingError),
    Parsed(ParsedError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            //     Self::Parsing(parsing) => write!(f, "parsing error: {parsing}"),
            Self::Conversion(conversion) => write!(f, "conversion error: {conversion}"),
            Self::Parsed(parsed) => write!(f, "parse error: {parsed}"),
        }
    }
}

impl From<ConversionError> for Error {
    fn from(value: ConversionError) -> Self {
        Self::Conversion(value)
    }
}

/*
impl From<ParsingError> for Error {
    fn from(value: ParsingError) -> Self {
        Self::Parsing(value)
    }
}*/

impl From<ParsedError> for Error {
    fn from(value: ParsedError) -> Self {
        Self::Parsed(value)
    }
}
