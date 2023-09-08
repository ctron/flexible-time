use crate::error::ParsingError;
use crate::{error::ParsedError, primitives::*};
use std::num::NonZeroU8;
use std::str::FromStr;
use time::Month;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParsedTimestamp {
    Year(i32),
    YearMonth(i32, Month),
    YearMonthDay(i32, Month, NonZeroU8),

    YearMonthDayHour(i32, Month, NonZeroU8, u8),
    YearMonthDayHourMinute(i32, Month, NonZeroU8, u8, u8),
    YearMonthDayHourMinuteSecond(i32, Month, NonZeroU8, u8, u8, u8),
}

impl FromStr for ParsedTimestamp {
    type Err = ParsedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();

        // we start out with a random error, just to keep the pattern
        Err(ParsingError::MissingInformation)
            .or_else(|_| parse_year_month_day_hour_minute_second(s))
            .or_else(|_| parse_year_month_day_hour_minute(s))
            .or_else(|_| parse_year_month_day_hour(s))
            .or_else(|_| parse_year_month_day(s))
            .or_else(|_| parse_year_month(s))
            .or_else(|_| parse_year(s))
            .or(Err(ParsedError::UnknownFormat))
    }
}
