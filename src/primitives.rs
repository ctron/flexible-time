//! Parsing primitives

use crate::error::ParsingError;
use crate::timestamp::ParsedTimestamp;
use time::{macros::format_description, parsing::Parsed};

pub fn parse_format_opt<F>(
    input: &[u8],
    items: &[::time::format_description::FormatItem<'_>],
    f: F,
) -> Result<ParsedTimestamp, ParsingError>
where
    F: FnOnce(Parsed) -> Option<ParsedTimestamp>,
{
    let mut parsed = Parsed::new();

    let result = parsed.parse_items(input, items)?;

    if !result.is_empty() {
        return Err(ParsingError::RemainingInformation);
    }

    f(parsed).ok_or(ParsingError::MissingInformation)
}

/// Parse year.
pub fn parse_year(input: &[u8]) -> Result<ParsedTimestamp, ParsingError> {
    let format = format_description!("[year]");

    parse_format_opt(input, format, |parsed| {
        Some(ParsedTimestamp::Year(parsed.year()?))
    })
}

/// Parse year, and month.
pub fn parse_year_month(input: &[u8]) -> Result<ParsedTimestamp, ParsingError> {
    let format = format_description!("[year]-[month]");

    parse_format_opt(input, format, |parsed| {
        Some(ParsedTimestamp::YearMonth(parsed.year()?, parsed.month()?))
    })
}

/// Parse year, month, and day.
pub fn parse_year_month_day(input: &[u8]) -> Result<ParsedTimestamp, ParsingError> {
    let format = format_description!("[year]-[month]-[day]");

    parse_format_opt(input, format, |parsed| {
        Some(ParsedTimestamp::YearMonthDay(
            parsed.year()?,
            parsed.month()?,
            parsed.day()?,
        ))
    })
}

/// Parse year, month, day, and hour.
pub fn parse_year_month_day_hour(input: &[u8]) -> Result<ParsedTimestamp, ParsingError> {
    let format = format_description!("[year]-[month]-[day][first [T] [ ]][hour padding:none]");

    parse_format_opt(input, format, |parsed| {
        Some(ParsedTimestamp::YearMonthDayHour(
            parsed.year()?,
            parsed.month()?,
            parsed.day()?,
            parsed.hour_24()?,
        ))
    })
}

/// Parse year, month, day, hour, and minute.
pub fn parse_year_month_day_hour_minute(input: &[u8]) -> Result<ParsedTimestamp, ParsingError> {
    let format =
        format_description!("[year]-[month]-[day][first [T] [ ]][hour padding:none]:[minute]");

    parse_format_opt(input, format, |parsed| {
        Some(ParsedTimestamp::YearMonthDayHourMinute(
            parsed.year()?,
            parsed.month()?,
            parsed.day()?,
            parsed.hour_24()?,
            parsed.minute()?,
        ))
    })
}

/// Parse year, month, day, hour, minute, and second.
pub fn parse_year_month_day_hour_minute_second(
    input: &[u8],
) -> Result<ParsedTimestamp, ParsingError> {
    let format = format_description!(
        "[year]-[month]-[day][first [T] [ ]][hour padding:none]:[minute]:[second]"
    );

    parse_format_opt(input, format, |parsed| {
        Some(ParsedTimestamp::YearMonthDayHourMinuteSecond(
            parsed.year()?,
            parsed.month()?,
            parsed.day()?,
            parsed.hour_24()?,
            parsed.minute()?,
            parsed.second()?,
        ))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_optional() {
        let format =
            format_description!("[year]-[month]-[day][first [T] [ ]][hour]:[minute]:[second]");
        let mut parsed = Parsed::new();
        parsed.parse_items(b"2023-01-01 12:34:56", &format).unwrap();
        parsed.parse_items(b"2023-01-01T12:34:56", &format).unwrap();
    }
}
