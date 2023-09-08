use crate::{
    error::{ConversionError, Error},
    timestamp::ParsedTimestamp,
};
use std::str::FromStr;
use time::{Date, Month, OffsetDateTime};

/// A timestamp which will be align with the start of a [`ParsedTimestamp`]. Meaning, it will
/// take the minimum value of all omitted components.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StartTimestamp(pub OffsetDateTime);

impl From<StartTimestamp> for OffsetDateTime {
    fn from(value: StartTimestamp) -> Self {
        value.0
    }
}

impl TryFrom<ParsedTimestamp> for StartTimestamp {
    type Error = ConversionError;

    fn try_from(value: ParsedTimestamp) -> Result<Self, Self::Error> {
        Ok(StartTimestamp(match value {
            ParsedTimestamp::Year(year) => Date::from_calendar_date(year, Month::January, 1)?
                .midnight()
                .assume_utc(),
            ParsedTimestamp::YearMonth(year, month) => Date::from_calendar_date(year, month, 1)?
                .midnight()
                .assume_utc(),
            ParsedTimestamp::YearMonthDay(year, month, day) => {
                Date::from_calendar_date(year, month, day.get())?
                    .midnight()
                    .assume_utc()
            }

            ParsedTimestamp::YearMonthDayHour(year, month, day, hour) => {
                Date::from_calendar_date(year, month, day.get())?
                    .with_hms(hour, 0, 0)?
                    .assume_utc()
            }
            ParsedTimestamp::YearMonthDayHourMinute(year, month, day, hour, minute) => {
                Date::from_calendar_date(year, month, day.get())?
                    .with_hms(hour, minute, 0)?
                    .assume_utc()
            }
            ParsedTimestamp::YearMonthDayHourMinuteSecond(
                year,
                month,
                day,
                hour,
                minute,
                second,
            ) => Date::from_calendar_date(year, month, day.get())?
                .with_hms(hour, minute, second)?
                .assume_utc(),
        }))
    }
}

impl FromStr for StartTimestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StartTimestamp::try_from(ParsedTimestamp::from_str(s)?)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use time::macros::datetime;
    use time::OffsetDateTime;

    macro_rules! assert_start {
        ($name:ident, $s:literal, $dt:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    $dt,
                    OffsetDateTime::from(StartTimestamp::from_str($s).unwrap())
                )
            }
        };
    }

    assert_start!(start_year, "2023", datetime!(2023-01-01 0:00 UTC));
    assert_start!(start_year_month, "2023-02", datetime!(2023-02-01 0:00 UTC));
    assert_start!(
        start_year_month_day,
        "2023-02-03",
        datetime!(2023-02-03 0:00 UTC)
    );

    assert_start!(
        start_year_month_day_hour_single,
        "2023-02-03 4",
        datetime!(2023-02-03 4:00 UTC)
    );
    assert_start!(
        start_year_month_day_hour,
        "2023-02-03 04",
        datetime!(2023-02-03 4:00 UTC)
    );

    assert_start!(
        start_year_month_day_hour_single_minute,
        "2023-02-03 4:05",
        datetime!(2023-02-03 4:05 UTC)
    );
    assert_start!(
        start_year_month_day_hour_minute,
        "2023-02-03 04:05",
        datetime!(2023-02-03 4:05 UTC)
    );

    assert_start!(
        start_year_month_day_hour_minute_second,
        "2023-02-03 04:05:06",
        datetime!(2023-02-03 4:05:06 UTC)
    );
    assert_start!(
        start_year_month_day_hour_single_minute_second,
        "2023-02-03 4:05:06",
        datetime!(2023-02-03 4:05:06 UTC)
    );

    assert_start!(
        start_year_month_day_hour_24h_minute_second,
        "2023-02-03 16:05:06",
        datetime!(2023-02-03 16:05:06 UTC)
    );
}
