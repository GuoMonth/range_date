use chrono::{Datelike, Months, NaiveDate};
use serde::{Deserialize, Deserializer, Serialize};

use crate::leap_year;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatePeriod {
    /// Represents a yearly period with a specific year.
    Year(u32),
    /// Represents a quarterly period with a specific year and quarter (1-4).
    Quarter(u32, u32),
    /// Represents a monthly period with a specific year and month (1-12).
    Month(u32, u32),
    /// Represents a daily period with a specific year and day of the year (1-366).
    Daily(u32, u32),
}

impl Serialize for DatePeriod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DatePeriod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<DatePeriod, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::str::FromStr;
        let s = String::deserialize(deserializer)?;
        DatePeriod::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl std::str::FromStr for DatePeriod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl std::fmt::Display for DatePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl DatePeriod {
    /// Create a new yearly period
    pub fn year(year: u32) -> Self {
        DatePeriod::Year(year)
    }

    /// Create a new quarterly period with validation
    /// Quarter must be between 1 and 4
    pub fn quarter(year: u32, quarter: u32) -> anyhow::Result<Self> {
        if !(1..=4).contains(&quarter) {
            return Err(anyhow::anyhow!(
                "Quarter must be between 1 and 4, got: {}",
                quarter
            ));
        }
        Ok(DatePeriod::Quarter(year, quarter))
    }

    /// Create a new monthly period with validation
    /// Month must be between 1 and 12
    pub fn month(year: u32, month: u32) -> anyhow::Result<Self> {
        if !(1..=12).contains(&month) {
            return Err(anyhow::anyhow!(
                "Month must be between 1 and 12, got: {}",
                month
            ));
        }
        Ok(DatePeriod::Month(year, month))
    }

    /// Create a new daily period with validation
    /// Day must be between 1 and 366 (accounting for leap years)
    pub fn daily(year: u32, day: u32) -> anyhow::Result<Self> {
        if day == 0 {
            return Err(anyhow::anyhow!("Day must be greater than 0"));
        }

        let max_days = if leap_year(year as i32) { 366 } else { 365 };
        if day > max_days {
            return Err(anyhow::anyhow!(
                "Day {} is invalid for year {} (max: {})",
                day,
                year,
                max_days
            ));
        }
        Ok(DatePeriod::Daily(year, day))
    }

    /// Parse a DatePeriod from a string representation like "2024Q2"
    /// Format: YYYYT[#] where T is period type (Y/Q/M/D) and # is the index (optional for Y)
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        let s = s.trim();
        if s.len() < 5 {
            return Err(anyhow::anyhow!("Invalid format, expected YYYYT[#]: {}", s));
        }

        let year: u32 = s[0..4]
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid year in: {}", s))?;
        let period_type = &s[4..5];

        match period_type {
            "Y" => {
                // Year format is just "2024Y" - no index needed
                if s.len() != 5 {
                    return Err(anyhow::anyhow!("Year format should be YYYYY: {}", s));
                }
                Ok(Self::year(year))
            }
            "Q" | "M" | "D" => {
                if s.len() <= 5 {
                    return Err(anyhow::anyhow!("Missing index for {}: {}", period_type, s));
                }
                let index: u32 = s[5..]
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Invalid index in: {}", s))?;

                match period_type {
                    "Q" => Self::quarter(year, index),
                    "M" => Self::month(year, index),
                    "D" => Self::daily(year, index),
                    _ => unreachable!(),
                }
            }
            _ => Err(anyhow::anyhow!(
                "Invalid period type '{}' in: {}",
                period_type,
                s
            )),
        }
    }

    /// Convert a NaiveDate to a yearly DatePeriod
    pub fn from_date_as_year(date: NaiveDate) -> Self {
        Self::year(date.year() as u32)
    }

    /// Convert a NaiveDate to a quarterly DatePeriod
    pub fn from_date_as_quarter(date: NaiveDate) -> Self {
        let year = date.year() as u32;
        let month = date.month();
        let quarter = match month {
            1..=3 => 1,
            4..=6 => 2,
            7..=9 => 3,
            10..=12 => 4,
            _ => unreachable!(),
        };
        DatePeriod::Quarter(year, quarter)
    }

    /// Convert a NaiveDate to a monthly DatePeriod
    pub fn from_date_as_month(date: NaiveDate) -> Self {
        DatePeriod::Month(date.year() as u32, date.month())
    }

    /// Convert a NaiveDate to a daily DatePeriod
    pub fn from_date_as_daily(date: NaiveDate) -> Self {
        DatePeriod::Daily(date.year() as u32, date.ordinal())
    }

    /// Get the first day of this period
    pub fn get_first_day(&self) -> NaiveDate {
        match self {
            DatePeriod::Year(year) => NaiveDate::from_ymd_opt(*year as i32, 1, 1).unwrap(),
            DatePeriod::Quarter(year, quarter) => {
                NaiveDate::from_ymd_opt(*year as i32, (quarter - 1) * 3 + 1, 1).unwrap()
            }
            DatePeriod::Month(year, month) => {
                NaiveDate::from_ymd_opt(*year as i32, *month, 1).unwrap()
            }
            DatePeriod::Daily(year, day) => NaiveDate::from_yo_opt(*year as i32, *day).unwrap(),
        }
    }

    /// Get the last day of this period
    pub fn get_last_day(&self) -> NaiveDate {
        match self {
            DatePeriod::Year(year) => NaiveDate::from_ymd_opt(*year as i32, 12, 31).unwrap(),
            DatePeriod::Quarter(_, _) => {
                let first_day = self.get_first_day();
                first_day
                    .checked_add_months(Months::new(3))
                    .unwrap()
                    .pred_opt()
                    .unwrap()
            }
            DatePeriod::Month(_, _) => {
                let first_day = self.get_first_day();
                first_day
                    .checked_add_months(Months::new(1))
                    .unwrap()
                    .pred_opt()
                    .unwrap()
            }
            DatePeriod::Daily(_, _) => self.get_first_day(),
        }
    }

    /// Check if this period contains the given date
    pub fn contains_date(&self, date: NaiveDate) -> bool {
        date >= self.get_first_day() && date <= self.get_last_day()
    }

    /// Get the year component
    pub fn get_year(&self) -> u32 {
        match self {
            DatePeriod::Year(year) => *year,
            DatePeriod::Quarter(year, _) => *year,
            DatePeriod::Month(year, _) => *year,
            DatePeriod::Daily(year, _) => *year,
        }
    }

    /// Get the period value (quarter number, month number, or day number)
    pub fn value(&self) -> u32 {
        match self {
            DatePeriod::Year(year) => *year,
            DatePeriod::Quarter(_, quarter) => *quarter,
            DatePeriod::Month(_, month) => *month,
            DatePeriod::Daily(_, day) => *day,
        }
    }

    /// Get the short name of the period type
    pub fn short_name(&self) -> &'static str {
        match self {
            DatePeriod::Year(_) => "Y",
            DatePeriod::Quarter(_, _) => "Q",
            DatePeriod::Month(_, _) => "M",
            DatePeriod::Daily(_, _) => "D",
        }
    }

    /// Get the full name of the period type
    pub fn period_name(&self) -> &'static str {
        match self {
            DatePeriod::Year(_) => "YEAR",
            DatePeriod::Quarter(_, _) => "QUARTER",
            DatePeriod::Month(_, _) => "MONTH",
            DatePeriod::Daily(_, _) => "DAILY",
        }
    }

    /// Convert to string representation like "2024Q2"
    pub fn to_string(&self) -> String {
        match self {
            DatePeriod::Year(year) => format!("{}Y", year),
            DatePeriod::Quarter(year, quarter) => format!("{}Q{}", year, quarter),
            DatePeriod::Month(year, month) => format!("{}M{}", year, month),
            DatePeriod::Daily(year, day) => format!("{}D{}", year, day),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use chrono::NaiveDate;
    use serde_json;

    #[test]
    fn test_constructors() {
        // Test year constructor
        let year_period = DatePeriod::year(2024);
        assert_eq!(year_period, DatePeriod::Year(2024));

        // Test quarter constructor with validation
        let quarter_period = DatePeriod::quarter(2024, 2).unwrap();
        assert_eq!(quarter_period, DatePeriod::Quarter(2024, 2));

        // Test invalid quarter
        assert!(DatePeriod::quarter(2024, 5).is_err());
        assert!(DatePeriod::quarter(2024, 0).is_err());

        // Test month constructor with validation
        let month_period = DatePeriod::month(2024, 5).unwrap();
        assert_eq!(month_period, DatePeriod::Month(2024, 5));

        // Test invalid month
        assert!(DatePeriod::month(2024, 13).is_err());
        assert!(DatePeriod::month(2024, 0).is_err());

        // Test daily constructor with validation
        let daily_period = DatePeriod::daily(2024, 136).unwrap();
        assert_eq!(daily_period, DatePeriod::Daily(2024, 136));

        // Test invalid day
        assert!(DatePeriod::daily(2024, 367).is_err()); // Even leap year max is 366
        assert!(DatePeriod::daily(2023, 366).is_err()); // Non-leap year max is 365
        assert!(DatePeriod::daily(2024, 0).is_err());
    }

    #[test]
    fn test_parse_from_string() {
        // Test valid parsing
        assert_eq!(DatePeriod::parse("2024Y").unwrap(), DatePeriod::Year(2024));
        assert_eq!(
            DatePeriod::parse("2024Q2").unwrap(),
            DatePeriod::Quarter(2024, 2)
        );
        assert_eq!(
            DatePeriod::parse("2024M5").unwrap(),
            DatePeriod::Month(2024, 5)
        );
        assert_eq!(
            DatePeriod::parse("2024D136").unwrap(),
            DatePeriod::Daily(2024, 136)
        );

        // Test invalid parsing
        assert!(DatePeriod::parse("invalid").is_err());
        assert!(DatePeriod::parse("202").is_err()); // Too short
        assert!(DatePeriod::parse("2024X1").is_err()); // Invalid period type
        assert!(DatePeriod::parse("2024Q5").is_err()); // Invalid quarter
        assert!(DatePeriod::parse("2024M13").is_err()); // Invalid month
    }

    #[test]
    fn test_from_str_trait() {
        assert_eq!(
            DatePeriod::from_str("2024Y").unwrap(),
            DatePeriod::Year(2024)
        );
        assert_eq!(
            DatePeriod::from_str("2024Q2").unwrap(),
            DatePeriod::Quarter(2024, 2)
        );
        assert_eq!(
            DatePeriod::from_str("2024M5").unwrap(),
            DatePeriod::Month(2024, 5)
        );
        assert_eq!(
            DatePeriod::from_str("2024D136").unwrap(),
            DatePeriod::Daily(2024, 136)
        );
        assert!(DatePeriod::from_str("INVALID").is_err());
    }

    #[test]
    fn test_from_naive_date() {
        let date = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();

        // Test conversion to different period types
        assert_eq!(DatePeriod::from_date_as_year(date), DatePeriod::Year(2024));
        assert_eq!(
            DatePeriod::from_date_as_quarter(date),
            DatePeriod::Quarter(2024, 2)
        );
        assert_eq!(
            DatePeriod::from_date_as_month(date),
            DatePeriod::Month(2024, 5)
        );
        assert_eq!(
            DatePeriod::from_date_as_daily(date),
            DatePeriod::Daily(2024, 136)
        ); // May 15 is 136th day
    }

    #[test]
    fn test_get_first_and_last_day() {
        // Test year
        let year_period = DatePeriod::year(2024);
        assert_eq!(
            year_period.get_first_day(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );
        assert_eq!(
            year_period.get_last_day(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );

        // Test quarter
        let quarter_period = DatePeriod::quarter(2024, 2).unwrap(); // Q2 = Apr-Jun
        assert_eq!(
            quarter_period.get_first_day(),
            NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()
        );
        assert_eq!(
            quarter_period.get_last_day(),
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()
        );

        // Test month
        let month_period = DatePeriod::month(2024, 5).unwrap(); // May
        assert_eq!(
            month_period.get_first_day(),
            NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
        );
        assert_eq!(
            month_period.get_last_day(),
            NaiveDate::from_ymd_opt(2024, 5, 31).unwrap()
        );

        // Test daily
        let daily_period = DatePeriod::daily(2024, 136).unwrap(); // May 15
        let expected_date = NaiveDate::from_yo_opt(2024, 136).unwrap();
        assert_eq!(daily_period.get_first_day(), expected_date);
        assert_eq!(daily_period.get_last_day(), expected_date);
    }

    #[test]
    fn test_contains_date() {
        let quarter_period = DatePeriod::quarter(2024, 2).unwrap(); // Q2 2024

        // Should contain dates in Q2
        assert!(quarter_period.contains_date(NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()));
        assert!(quarter_period.contains_date(NaiveDate::from_ymd_opt(2024, 5, 15).unwrap()));
        assert!(quarter_period.contains_date(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()));

        // Should not contain dates outside Q2
        assert!(!quarter_period.contains_date(NaiveDate::from_ymd_opt(2024, 3, 31).unwrap()));
        assert!(!quarter_period.contains_date(NaiveDate::from_ymd_opt(2024, 7, 1).unwrap()));
    }

    #[test]
    fn test_getters() {
        let quarter_period = DatePeriod::quarter(2024, 2).unwrap();
        assert_eq!(quarter_period.get_year(), 2024);
        assert_eq!(quarter_period.value(), 2);
        assert_eq!(quarter_period.short_name(), "Q");
        assert_eq!(quarter_period.period_name(), "QUARTER");

        let month_period = DatePeriod::month(2024, 5).unwrap();
        assert_eq!(month_period.get_year(), 2024);
        assert_eq!(month_period.value(), 5);
        assert_eq!(month_period.short_name(), "M");
        assert_eq!(month_period.period_name(), "MONTH");
    }

    #[test]
    fn test_to_string() {
        assert_eq!(DatePeriod::year(2024).to_string(), "2024Y");
        assert_eq!(DatePeriod::quarter(2024, 2).unwrap().to_string(), "2024Q2");
        assert_eq!(DatePeriod::month(2024, 5).unwrap().to_string(), "2024M5");
        assert_eq!(
            DatePeriod::daily(2024, 136).unwrap().to_string(),
            "2024D136"
        );
    }

    #[test]
    fn test_display_trait() {
        let quarter_period = DatePeriod::quarter(2024, 2).unwrap();
        assert_eq!(format!("{}", quarter_period), "2024Q2");
    }

    #[test]
    fn test_serialization() {
        let month_period = DatePeriod::month(2024, 5).unwrap();
        let serialized = serde_json::to_string(&month_period).unwrap();
        assert_eq!(serialized, "\"2024M5\"");

        let deserialized: DatePeriod = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, DatePeriod::Month(2024, 5));
    }

    #[test]
    fn test_leap_year_validation() {
        // Leap year - 366 days allowed
        assert!(DatePeriod::daily(2024, 366).is_ok());

        // Non-leap year - only 365 days allowed
        assert!(DatePeriod::daily(2023, 365).is_ok());
        assert!(DatePeriod::daily(2023, 366).is_err());
    }
}
