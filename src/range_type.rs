use chrono::{Datelike, Duration, Months, NaiveDate};
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
        serializer.serialize_str(&format!("{}", self))
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
        match self {
            DatePeriod::Year(year) => write!(f, "{}Y", year),
            DatePeriod::Quarter(year, quarter) => write!(f, "{}Q{}", year, quarter),
            DatePeriod::Month(year, month) => write!(f, "{}M{}", year, month),
            DatePeriod::Daily(year, day) => write!(f, "{}D{}", year, day),
        }
    }
}

impl DatePeriod {
    /// Create a new yearly period
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let year = DatePeriod::year(2024);
    /// assert_eq!(year.to_string(), "2024Y");
    /// ```
    pub fn year(year: u32) -> Self {
        DatePeriod::Year(year)
    }

    /// Create a new quarterly period with validation
    /// Quarter must be between 1 and 4
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let quarter = DatePeriod::quarter(2024, 2).unwrap();
    /// assert_eq!(quarter.to_string(), "2024Q2");
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let month = DatePeriod::month(2024, 5).unwrap();
    /// assert_eq!(month.to_string(), "2024M5");
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let daily = DatePeriod::daily(2024, 136).unwrap();
    /// assert_eq!(daily.to_string(), "2024D136");
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let period = DatePeriod::parse("2024Q2").unwrap();
    /// assert_eq!(period.to_string(), "2024Q2");
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
    /// let year = DatePeriod::from_date_as_year(date);
    /// assert_eq!(year.to_string(), "2024Y");
    /// ```
    pub fn from_date_as_year(date: NaiveDate) -> Self {
        Self::year(date.year() as u32)
    }

    /// Convert a NaiveDate to a quarterly DatePeriod
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
    /// let quarter = DatePeriod::from_date_as_quarter(date);
    /// assert_eq!(quarter.to_string(), "2024Q2");
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
    /// let month = DatePeriod::from_date_as_month(date);
    /// assert_eq!(month.to_string(), "2024M5");
    /// ```
    pub fn from_date_as_month(date: NaiveDate) -> Self {
        DatePeriod::Month(date.year() as u32, date.month())
    }

    /// Convert a NaiveDate to a daily DatePeriod
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
    /// let daily = DatePeriod::from_date_as_daily(date);
    /// assert_eq!(daily.to_string(), "2024D136");
    /// ```
    pub fn from_date_as_daily(date: NaiveDate) -> Self {
        DatePeriod::Daily(date.year() as u32, date.ordinal())
    }

    /// Generate all yearly periods between two dates (inclusive)
    /// Returns an empty vector if start > end
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let start = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2025, 3, 10).unwrap();
    /// let years = DatePeriod::between_date_as_year(start, end).unwrap();
    /// assert_eq!(years.len(), 3);
    /// assert_eq!(years[0].to_string(), "2023Y");
    /// ```
    pub fn between_date_as_year(
        start: NaiveDate,
        end: NaiveDate,
    ) -> anyhow::Result<Vec<DatePeriod>> {
        if start > end {
            return Ok(vec![]);
        }
        let start_year = start.year() as u32;
        let end_year = end.year() as u32;
        Ok((start_year..=end_year).map(DatePeriod::year).collect())
    }

    /// Generate all quarterly periods between two dates (inclusive)
    /// Returns an empty vector if start > end
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let start = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2024, 9, 30).unwrap();
    /// let quarters = DatePeriod::between_date_as_quarter(start, end).unwrap();
    /// assert_eq!(quarters.len(), 2);
    /// assert_eq!(quarters[0].to_string(), "2024Q2");
    /// ```
    pub fn between_date_as_quarter(
        start: NaiveDate,
        end: NaiveDate,
    ) -> anyhow::Result<Vec<DatePeriod>> {
        if start > end {
            return Ok(vec![]);
        }
        let mut result = vec![];
        let mut current = DatePeriod::from_date_as_quarter(start);
        let end_quarter = DatePeriod::from_date_as_quarter(end);
        while current <= end_quarter {
            result.push(current.clone());
            current = current.succ()?;
        }
        Ok(result)
    }

    /// Generate all monthly periods between two dates (inclusive)
    /// Returns an empty vector if start > end
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let start = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2024, 4, 30).unwrap();
    /// let months = DatePeriod::between_date_as_month(start, end).unwrap();
    /// assert_eq!(months.len(), 3);
    /// assert_eq!(months[0].to_string(), "2024M2");
    /// ```
    pub fn between_date_as_month(
        start: NaiveDate,
        end: NaiveDate,
    ) -> anyhow::Result<Vec<DatePeriod>> {
        if start > end {
            return Ok(vec![]);
        }
        let mut result = vec![];
        let mut current = DatePeriod::from_date_as_month(start);
        let end_month = DatePeriod::from_date_as_month(end);
        while current <= end_month {
            result.push(current.clone());
            current = current.succ()?;
        }
        Ok(result)
    }

    /// Generate all daily periods between two dates (inclusive)
    /// Returns an empty vector if start > end
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let start = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2024, 2, 3).unwrap();
    /// let days = DatePeriod::between_date_as_daily(start, end).unwrap();
    /// assert_eq!(days.len(), 3);
    /// assert_eq!(days[0].to_string(), "2024D32");
    /// ```
    pub fn between_date_as_daily(
        start: NaiveDate,
        end: NaiveDate,
    ) -> anyhow::Result<Vec<DatePeriod>> {
        if start > end {
            return Ok(vec![]);
        }
        let mut result = vec![];
        let mut current = DatePeriod::from_date_as_daily(start);
        let end_daily = DatePeriod::from_date_as_daily(end);
        while current <= end_daily {
            result.push(current.clone());
            current = current.succ()?;
        }
        Ok(result)
    }

    /// Get the first day of this period
    ///
    /// Returns the first date of the period. Since DatePeriod instances should only
    /// be created through validated constructors, this should always succeed.
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let period = DatePeriod::month(2024, 2).unwrap();
    /// let first_day = period.get_first_day().unwrap();
    /// assert_eq!(first_day, NaiveDate::from_ymd_opt(2024, 2, 1).unwrap());
    /// ```
    pub fn get_first_day(&self) -> anyhow::Result<NaiveDate> {
        match self {
            DatePeriod::Year(year) => NaiveDate::from_ymd_opt(*year as i32, 1, 1)
                .ok_or_else(|| anyhow::anyhow!("Invalid year for date creation: {}", year)),
            DatePeriod::Quarter(year, quarter) => {
                let month = (quarter - 1) * 3 + 1;
                NaiveDate::from_ymd_opt(*year as i32, month, 1).ok_or_else(|| {
                    anyhow::anyhow!("Invalid quarter date: year {}, quarter {}", year, quarter)
                })
            }
            DatePeriod::Month(year, month) => NaiveDate::from_ymd_opt(*year as i32, *month, 1)
                .ok_or_else(|| {
                    anyhow::anyhow!("Invalid month date: year {}, month {}", year, month)
                }),
            DatePeriod::Daily(year, day) => NaiveDate::from_yo_opt(*year as i32, *day)
                .ok_or_else(|| anyhow::anyhow!("Invalid daily date: year {}, day {}", year, day)),
        }
    }

    /// Get the last day of this period
    ///
    /// Returns the last date of the period.
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let period = DatePeriod::month(2024, 2).unwrap();
    /// let last_day = period.get_last_day().unwrap();
    /// assert_eq!(last_day, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()); // 2024 is leap year
    /// ```
    pub fn get_last_day(&self) -> anyhow::Result<NaiveDate> {
        match self {
            DatePeriod::Year(year) => NaiveDate::from_ymd_opt(*year as i32, 12, 31)
                .ok_or_else(|| anyhow::anyhow!("Invalid year for last day calculation: {}", year)),
            DatePeriod::Quarter(_, _) => {
                let first_day = self.get_first_day()?;
                let added_months =
                    first_day
                        .checked_add_months(Months::new(3))
                        .ok_or_else(|| {
                            anyhow::anyhow!("Failed to add 3 months to quarter start date")
                        })?;
                let last_day = added_months.pred_opt().ok_or_else(|| {
                    anyhow::anyhow!("Failed to get predecessor date for quarter end")
                })?;
                Ok(last_day)
            }
            DatePeriod::Month(_, _) => {
                let first_day = self.get_first_day()?;
                let added_months = first_day
                    .checked_add_months(Months::new(1))
                    .ok_or_else(|| anyhow::anyhow!("Failed to add 1 month to month start date"))?;
                let last_day = added_months.pred_opt().ok_or_else(|| {
                    anyhow::anyhow!("Failed to get predecessor date for month end")
                })?;
                Ok(last_day)
            }
            DatePeriod::Daily(_, _) => self.get_first_day(), // Same as first day for daily period
        }
    }

    /// Check if this period contains the given date
    ///
    /// Returns false if there's an error calculating the date boundaries.
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    /// use chrono::NaiveDate;
    ///
    /// let period = DatePeriod::month(2024, 2).unwrap();
    /// let date_in_period = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap();
    /// let date_outside_period = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
    /// assert!(period.contains_date(date_in_period));
    /// assert!(!period.contains_date(date_outside_period));
    /// ```
    pub fn contains_date(&self, date: NaiveDate) -> bool {
        match (self.get_first_day(), self.get_last_day()) {
            (Ok(first), Ok(last)) => date >= first && date <= last,
            _ => false, // Return false if we can't calculate the boundaries
        }
    }

    /// Get the year component
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let period = DatePeriod::month(2024, 2).unwrap();
    /// assert_eq!(period.get_year(), 2024);
    /// ```
    pub fn get_year(&self) -> u32 {
        match self {
            DatePeriod::Year(year) => *year,
            DatePeriod::Quarter(year, _) => *year,
            DatePeriod::Month(year, _) => *year,
            DatePeriod::Daily(year, _) => *year,
        }
    }

    /// Get the period value (quarter number, month number, or day number)
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let year_period = DatePeriod::year(2024);
    /// assert_eq!(year_period.value(), 2024);
    ///
    /// let month_period = DatePeriod::month(2024, 2).unwrap();
    /// assert_eq!(month_period.value(), 2);
    /// ```
    pub fn value(&self) -> u32 {
        match self {
            DatePeriod::Year(year) => *year,
            DatePeriod::Quarter(_, quarter) => *quarter,
            DatePeriod::Month(_, month) => *month,
            DatePeriod::Daily(_, day) => *day,
        }
    }

    /// Get the short name of the period type
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let year_period = DatePeriod::year(2024);
    /// assert_eq!(year_period.short_name(), "Y");
    ///
    /// let month_period = DatePeriod::month(2024, 2).unwrap();
    /// assert_eq!(month_period.short_name(), "M");
    /// ```
    pub fn short_name(&self) -> &'static str {
        match self {
            DatePeriod::Year(_) => "Y",
            DatePeriod::Quarter(_, _) => "Q",
            DatePeriod::Month(_, _) => "M",
            DatePeriod::Daily(_, _) => "D",
        }
    }

    /// Get the full name of the period type
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let year_period = DatePeriod::year(2024);
    /// assert_eq!(year_period.period_name(), "YEAR");
    ///
    /// let month_period = DatePeriod::month(2024, 2).unwrap();
    /// assert_eq!(month_period.period_name(), "MONTH");
    /// ```
    pub fn period_name(&self) -> &'static str {
        match self {
            DatePeriod::Year(_) => "YEAR",
            DatePeriod::Quarter(_, _) => "QUARTER",
            DatePeriod::Month(_, _) => "MONTH",
            DatePeriod::Daily(_, _) => "DAILY",
        }
    }

    /// Get the successor (next) period
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let period = DatePeriod::month(2024, 2).unwrap();
    /// let next_period = period.succ().unwrap();
    /// assert_eq!(next_period.to_string(), "2024M3");
    /// ```
    pub fn succ(&self) -> anyhow::Result<DatePeriod> {
        Ok(match self {
            DatePeriod::Year(year) => DatePeriod::Year(year + 1),
            DatePeriod::Quarter(year, quarter) => {
                if *quarter < 4 {
                    DatePeriod::Quarter(*year, quarter + 1)
                } else {
                    DatePeriod::Quarter(year + 1, 1)
                }
            }
            DatePeriod::Month(year, month) => {
                if *month < 12 {
                    DatePeriod::Month(*year, month + 1)
                } else {
                    DatePeriod::Month(year + 1, 1)
                }
            }
            DatePeriod::Daily(year, day) => {
                let max_days = if leap_year(*year as i32) { 366 } else { 365 };
                if *day < max_days {
                    DatePeriod::Daily(*year, day + 1)
                } else {
                    DatePeriod::Daily(year + 1, 1)
                }
            }
        })
    }

    /// Get the predecessor (previous) period
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let period = DatePeriod::month(2024, 2).unwrap();
    /// let prev_period = period.pred().unwrap();
    /// assert_eq!(prev_period.to_string(), "2024M1");
    /// ```
    pub fn pred(&self) -> anyhow::Result<DatePeriod> {
        Ok(match self {
            DatePeriod::Year(year) => {
                if *year > 0 {
                    DatePeriod::Year(year - 1)
                } else {
                    anyhow::bail!("No predecessor for year 0");
                }
            }
            DatePeriod::Quarter(year, quarter) => {
                if *quarter > 1 {
                    DatePeriod::Quarter(*year, quarter - 1)
                } else if *year > 0 {
                    DatePeriod::Quarter(year - 1, 4)
                } else {
                    anyhow::bail!("No predecessor for quarter 1 of year 0");
                }
            }
            DatePeriod::Month(year, month) => {
                if *month > 1 {
                    DatePeriod::Month(*year, month - 1)
                } else if *year > 0 {
                    DatePeriod::Month(year - 1, 12)
                } else {
                    anyhow::bail!("No predecessor for month 1 of year 0");
                }
            }
            DatePeriod::Daily(year, day) => {
                if *day > 1 {
                    DatePeriod::Daily(*year, day - 1)
                } else if *year > 0 {
                    let prev_year = year - 1;
                    let max_days_prev = if leap_year(prev_year as i32) {
                        366
                    } else {
                        365
                    };
                    DatePeriod::Daily(prev_year, max_days_prev)
                } else {
                    anyhow::bail!("No predecessor for day 1 of year 0");
                }
            }
        })
    }

    /// Decompose this period into its direct sub-periods
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let quarter = DatePeriod::quarter(2024, 1).unwrap();
    /// let months = quarter.decompose();
    /// assert_eq!(months.len(), 3);
    /// assert_eq!(months[0].to_string(), "2024M1");
    /// assert_eq!(months[2].to_string(), "2024M3");
    /// ```
    pub fn decompose(&self) -> Vec<DatePeriod> {
        match self {
            DatePeriod::Year(year) => (1..=4)
                .map(|q| match DatePeriod::quarter(*year, q) {
                    Ok(period) => period,
                    Err(_) => unreachable!("quarter should always succeed for valid q"),
                })
                .collect(),
            DatePeriod::Quarter(year, quarter) => {
                let start_month = (quarter - 1) * 3 + 1;
                (0..3)
                    .map(|i| match DatePeriod::month(*year, start_month + i) {
                        Ok(period) => period,
                        Err(_) => unreachable!("month should always succeed for valid month"),
                    })
                    .collect()
            }
            DatePeriod::Month(year, month) => {
                let first_day = match NaiveDate::from_ymd_opt(*year as i32, *month, 1) {
                    Some(date) => date,
                    None => unreachable!("from_ymd_opt should succeed for valid year and month"),
                };
                let last_day = first_day + Months::new(1) - Duration::days(1);
                (1..=last_day.day())
                    .map(|d| match DatePeriod::daily(*year, d) {
                        Ok(period) => period,
                        Err(_) => unreachable!("daily should always succeed for valid day"),
                    })
                    .collect()
            }
            DatePeriod::Daily(_, _) => vec![],
        }
    }

    /// Aggregate this period to its direct parent period
    ///
    /// # Examples
    ///
    /// ```
    /// use range_date::range_type::DatePeriod;
    ///
    /// let daily = DatePeriod::daily(2024, 32).unwrap();
    /// let month = daily.aggregate();
    /// assert_eq!(month, DatePeriod::month(2024, 2).unwrap());
    ///
    /// let quarter = DatePeriod::quarter(2024, 2).unwrap();
    /// let year = quarter.aggregate();
    /// assert_eq!(year, DatePeriod::year(2024));
    ///
    /// let year_period = DatePeriod::year(2024);
    /// let parent = year_period.aggregate();
    /// assert_eq!(parent, year_period); // Year has no parent, remains the same
    /// ```
    pub fn aggregate(&self) -> DatePeriod {
        match self {
            DatePeriod::Year(_) => self.clone(),
            DatePeriod::Quarter(year, _) => DatePeriod::year(*year),
            DatePeriod::Month(year, month) => {
                let quarter = ((month - 1) / 3) + 1;
                match DatePeriod::quarter(*year, quarter) {
                    Ok(period) => period,
                    Err(_) => unreachable!("quarter should always succeed for valid quarter"),
                }
            }
            DatePeriod::Daily(year, day) => {
                let date = match NaiveDate::from_yo_opt(*year as i32, *day) {
                    Some(d) => d,
                    None => unreachable!("from_yo_opt should succeed for valid year and day"),
                };
                match DatePeriod::month(date.year() as u32, date.month()) {
                    Ok(period) => period,
                    Err(_) => unreachable!("month should always succeed for valid month"),
                }
            }
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
    fn test_get_first_and_last_day() -> anyhow::Result<()> {
        // Test year
        let year_period = DatePeriod::year(2024);
        assert_eq!(
            year_period.get_first_day()?,
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );
        assert_eq!(
            year_period.get_last_day()?,
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );

        // Test quarter
        let quarter_period = DatePeriod::quarter(2024, 2).unwrap(); // Q2 = Apr-Jun
        assert_eq!(
            quarter_period.get_first_day()?,
            NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()
        );
        assert_eq!(
            quarter_period.get_last_day()?,
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()
        );

        // Test month
        let month_period = DatePeriod::month(2024, 5).unwrap(); // May
        assert_eq!(
            month_period.get_first_day()?,
            NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
        );
        assert_eq!(
            month_period.get_last_day()?,
            NaiveDate::from_ymd_opt(2024, 5, 31).unwrap()
        );

        // Test daily
        let daily_period = DatePeriod::daily(2024, 136).unwrap(); // May 15
        let expected_date = NaiveDate::from_yo_opt(2024, 136).unwrap();
        assert_eq!(daily_period.get_first_day()?, expected_date);
        assert_eq!(daily_period.get_last_day()?, expected_date);

        Ok(())
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

    #[test]
    fn test_succ() {
        // Test year
        assert_eq!(
            DatePeriod::year(2024).succ().unwrap(),
            DatePeriod::Year(2025)
        );

        // Test quarter
        assert_eq!(
            DatePeriod::quarter(2024, 2).unwrap().succ().unwrap(),
            DatePeriod::Quarter(2024, 3)
        );
        assert_eq!(
            DatePeriod::quarter(2024, 4).unwrap().succ().unwrap(),
            DatePeriod::Quarter(2025, 1)
        );

        // Test month
        assert_eq!(
            DatePeriod::month(2024, 5).unwrap().succ().unwrap(),
            DatePeriod::Month(2024, 6)
        );
        assert_eq!(
            DatePeriod::month(2024, 12).unwrap().succ().unwrap(),
            DatePeriod::Month(2025, 1)
        );

        // Test daily
        assert_eq!(
            DatePeriod::daily(2024, 135).unwrap().succ().unwrap(),
            DatePeriod::Daily(2024, 136)
        );
        assert_eq!(
            DatePeriod::daily(2024, 366).unwrap().succ().unwrap(),
            DatePeriod::Daily(2025, 1)
        ); // Leap year
        assert_eq!(
            DatePeriod::daily(2023, 365).unwrap().succ().unwrap(),
            DatePeriod::Daily(2024, 1)
        ); // Non-leap
    }

    #[test]
    fn test_pred() {
        // Test year
        assert_eq!(
            DatePeriod::year(2024).pred().unwrap(),
            DatePeriod::Year(2023)
        );
        assert_eq!(DatePeriod::year(0).pred().is_err(), true);

        // Test quarter
        assert_eq!(
            DatePeriod::quarter(2024, 2).unwrap().pred().unwrap(),
            DatePeriod::Quarter(2024, 1)
        );
        assert_eq!(
            DatePeriod::quarter(2024, 1).unwrap().pred().unwrap(),
            DatePeriod::Quarter(2023, 4)
        );
        assert_eq!(DatePeriod::quarter(0, 1).unwrap().pred().is_err(), true);

        // Test month
        assert_eq!(
            DatePeriod::month(2024, 5).unwrap().pred().unwrap(),
            DatePeriod::Month(2024, 4)
        );
        assert_eq!(
            DatePeriod::month(2024, 1).unwrap().pred().unwrap(),
            DatePeriod::Month(2023, 12)
        );
        assert_eq!(DatePeriod::month(0, 1).unwrap().pred().is_err(), true);

        // Test daily
        assert_eq!(
            DatePeriod::daily(2024, 135).unwrap().pred().unwrap(),
            DatePeriod::Daily(2024, 134)
        );
        assert_eq!(
            DatePeriod::daily(2024, 1).unwrap().pred().unwrap(),
            DatePeriod::Daily(2023, 365)
        ); // From leap to non-leap
        assert_eq!(DatePeriod::daily(0, 1).unwrap().pred().is_err(), true);
    }

    #[test]
    fn test_decompose() {
        // Test year
        let year_decomposed = DatePeriod::year(2025).decompose();
        assert_eq!(year_decomposed.len(), 4);
        assert_eq!(year_decomposed[0], DatePeriod::Quarter(2025, 1));
        assert_eq!(year_decomposed[3], DatePeriod::Quarter(2025, 4));

        // Test quarter
        let quarter_decomposed = DatePeriod::quarter(2025, 4).unwrap().decompose();
        assert_eq!(quarter_decomposed.len(), 3);
        assert_eq!(quarter_decomposed[0], DatePeriod::Month(2025, 10));
        assert_eq!(quarter_decomposed[2], DatePeriod::Month(2025, 12));

        // Test month (non-leap)
        let month_decomposed = DatePeriod::month(2023, 2).unwrap().decompose();
        assert_eq!(month_decomposed.len(), 28);
        assert_eq!(month_decomposed[0], DatePeriod::Daily(2023, 1));
        assert_eq!(month_decomposed[27], DatePeriod::Daily(2023, 28));

        // Test month (leap)
        let leap_month_decomposed = DatePeriod::month(2024, 2).unwrap().decompose();
        assert_eq!(leap_month_decomposed.len(), 29);
        assert_eq!(leap_month_decomposed[28], DatePeriod::Daily(2024, 29));

        // Test daily
        let daily_decomposed = DatePeriod::daily(2024, 1).unwrap().decompose();
        assert_eq!(daily_decomposed.len(), 0);
    }

    #[test]
    fn test_aggregate() {
        // Test daily
        assert_eq!(
            DatePeriod::daily(2024, 32).unwrap().aggregate(),
            DatePeriod::Month(2024, 2)
        );

        // Test month
        assert_eq!(
            DatePeriod::month(2025, 10).unwrap().aggregate(),
            DatePeriod::Quarter(2025, 4)
        );

        // Test quarter
        assert_eq!(
            DatePeriod::quarter(2025, 4).unwrap().aggregate(),
            DatePeriod::Year(2025)
        );

        // Test year
        assert_eq!(DatePeriod::year(2025).aggregate(), DatePeriod::Year(2025));
    }

    #[test]
    fn test_between_date_as_year() {
        let start = NaiveDate::from_ymd_opt(2023, 6, 15).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 3, 10).unwrap();

        let result = DatePeriod::between_date_as_year(start, end).unwrap();
        assert_eq!(
            result,
            vec![
                DatePeriod::Year(2023),
                DatePeriod::Year(2024),
                DatePeriod::Year(2025)
            ]
        );

        // Same year
        let same = DatePeriod::between_date_as_year(start, start).unwrap();
        assert_eq!(same, vec![DatePeriod::Year(2023)]);

        // Start > end
        let result_empty = DatePeriod::between_date_as_year(end, start).unwrap();
        assert_eq!(result_empty, vec![]);
    }

    #[test]
    fn test_between_date_as_quarter() {
        let start = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(); // Q2 2024
        let end = NaiveDate::from_ymd_opt(2024, 9, 30).unwrap(); // Q3 2024

        let result = DatePeriod::between_date_as_quarter(start, end).unwrap();
        assert_eq!(
            result,
            vec![DatePeriod::Quarter(2024, 2), DatePeriod::Quarter(2024, 3)]
        );

        // Cross year
        let start_cross = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap(); // Q4 2024
        let end_cross = NaiveDate::from_ymd_opt(2025, 3, 31).unwrap(); // Q1 2025
        let result_cross = DatePeriod::between_date_as_quarter(start_cross, end_cross).unwrap();
        assert_eq!(
            result_cross,
            vec![DatePeriod::Quarter(2024, 4), DatePeriod::Quarter(2025, 1)]
        );

        // Start > end
        let result_empty = DatePeriod::between_date_as_quarter(end, start).unwrap();
        assert_eq!(result_empty, vec![]);
    }

    #[test]
    fn test_between_date_as_month() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 4, 30).unwrap();

        let result = DatePeriod::between_date_as_month(start, end).unwrap();
        assert_eq!(
            result,
            vec![
                DatePeriod::Month(2024, 2),
                DatePeriod::Month(2024, 3),
                DatePeriod::Month(2024, 4)
            ]
        );

        // Cross year
        let start_cross = NaiveDate::from_ymd_opt(2024, 11, 1).unwrap();
        let end_cross = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();
        let result_cross = DatePeriod::between_date_as_month(start_cross, end_cross).unwrap();
        assert_eq!(
            result_cross,
            vec![
                DatePeriod::Month(2024, 11),
                DatePeriod::Month(2024, 12),
                DatePeriod::Month(2025, 1)
            ]
        );

        // Start > end
        let result_empty = DatePeriod::between_date_as_month(end, start).unwrap();
        assert_eq!(result_empty, vec![]);
    }

    #[test]
    fn test_between_date_as_daily() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 2).unwrap();

        let result = DatePeriod::between_date_as_daily(start, end).unwrap();
        assert_eq!(
            result,
            vec![
                DatePeriod::Daily(2024, 59), // Feb 28
                DatePeriod::Daily(2024, 60), // Feb 29 (leap)
                DatePeriod::Daily(2024, 61), // Mar 1
                DatePeriod::Daily(2024, 62)  // Mar 2
            ]
        );

        // Same day
        let same = DatePeriod::between_date_as_daily(start, start).unwrap();
        assert_eq!(same, vec![DatePeriod::Daily(2024, 59)]);

        // Start > end
        let result_empty = DatePeriod::between_date_as_daily(end, start).unwrap();
        assert_eq!(result_empty, vec![]);
    }
}
