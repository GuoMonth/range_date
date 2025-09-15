use std::str::FromStr;

use chrono::{Datelike, Months, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::{leap_year, range_type::DatePeriod};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RangeDate {
    pub year: i32,
    pub range_type: DatePeriod,
    pub range_index: u32,
}

impl std::fmt::Display for RangeDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.year,
            self.range_type.short_name(),
            self.range_index
        )
    }
}

impl std::str::FromStr for RangeDate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let year = s[0..4].parse::<i32>()?;
        let range_type = DatePeriod::from_str(&s[4..5])?;
        let range_index = s[5..].parse::<u32>()?;

        Ok(RangeDate {
            year,
            range_type,
            range_index,
        })
    }
}

impl Serialize for RangeDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

/// Deserialize a RangeDate from a string representation.
/// # Examples
/// ```
/// use range_date::range_date::RangeDate;
/// let rd: RangeDate = "2024Y1".parse().unwrap();
/// assert_eq!(rd.year, 2024);
/// assert_eq!(rd.range_type, range_date::range_type::DatePeriod::Year);
/// assert_eq!(rd.range_index, 1);
/// ```
impl<'de> Deserialize<'de> for RangeDate {
    fn deserialize<D>(deserializer: D) -> Result<RangeDate, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        // Deserialize the string from the input
        let s = String::deserialize(deserializer)?;
        RangeDate::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl RangeDate {
    /// Create a new RangeDate with validation.
    /// # Examples
    /// ```
    /// use range_date::range_date::RangeDate;
    /// use range_date::range_type::DatePeriod;
    /// let rd = RangeDate::new(2024, DatePeriod::Month, 5).unwrap();
    /// assert_eq!(rd.year, 2024);
    /// assert_eq!(rd.range_type, DatePeriod::Month);
    /// assert_eq!(rd.range_index, 5);
    /// ```
    pub fn new(year: i32, range_type: DatePeriod, range_index: u32) -> anyhow::Result<Self> {
        // validation
        // Year: any i32 is valid
        // Quarter: 1-4
        // Month: 1-12
        // Daily: 1-366 (not validating leap year here)

        if range_index == 0 {
            return Err(anyhow::anyhow!("range_index must be greater than 0"));
        }

        match range_type {
            DatePeriod::Year => {
                // any year is valid
            }
            DatePeriod::Quarter => {
                if range_index < 1 || range_index > 4 {
                    return Err(anyhow::anyhow!(
                        "For Quarter, range_index must be between 1 and 4"
                    ));
                }
            }
            DatePeriod::Month => {
                if range_index < 1 || range_index > 12 {
                    return Err(anyhow::anyhow!(
                        "For Month, range_index must be between 1 and 12"
                    ));
                }
            }
            DatePeriod::Daily => {
                if range_index < 1 || range_index > 366 {
                    return Err(anyhow::anyhow!(
                        "For Daily, range_index must be between 1 and 366"
                    ));
                }

                if leap_year(year) {
                    if range_index > 366 {
                        return Err(anyhow::anyhow!(
                            "For Daily in a leap year, range_index must be between 1 and 366"
                        ));
                    }
                } else {
                    if range_index > 365 {
                        return Err(anyhow::anyhow!(
                            "For Daily in a non-leap year, range_index must be between 1 and 365"
                        ));
                    }
                }
            }
        }

        Ok(RangeDate {
            year,
            range_type,
            range_index,
        })
    }

    /// Create a RangeDate from a NaiveDate and a DatePeriod
    /// # Examples
    /// ```
    /// use chrono::NaiveDate;
    /// use range_date::range_date::RangeDate;
    /// use range_date::range_type::DatePeriod;
    /// let date = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
    /// let rd = RangeDate::from_naive_date(&date, &DatePeriod::Month);
    /// assert_eq!(rd.year, 2024);
    /// assert_eq!(rd.range_type, DatePeriod::Month);
    /// assert_eq!(rd.range_index, 5);
    /// ```
    pub fn from_naive_date(date: &NaiveDate, range_type: &DatePeriod) -> Self {
        let year = date.year();
        let range_index = match range_type {
            DatePeriod::Year => year as u32,
            DatePeriod::Quarter => {
                let month = date.month();
                if month <= 3 {
                    1
                } else if month <= 6 {
                    2
                } else if month <= 9 {
                    3
                } else {
                    4
                }
            }
            DatePeriod::Month => date.month(),
            DatePeriod::Daily => date.ordinal(),
        };
        RangeDate {
            year,
            range_type: range_type.to_owned(),
            range_index,
        }
    }

    /// get the first day of the range
    pub fn get_first_day(&self) -> NaiveDate {
        match self.range_type {
            DatePeriod::Year => NaiveDate::from_ymd_opt(self.year, 1, 1).unwrap_or_default(),
            DatePeriod::Quarter => {
                NaiveDate::from_ymd_opt(self.year, (self.range_index * 3) - 2, 1)
                    .unwrap_or_default()
            }
            DatePeriod::Month => {
                NaiveDate::from_ymd_opt(self.year, self.range_index, 1).unwrap_or_default()
            }
            DatePeriod::Daily => {
                NaiveDate::from_yo_opt(self.year, self.range_index).unwrap_or_default()
            }
        }
    }

    /// get the last day of the range
    pub fn get_last_day(&self) -> NaiveDate {
        match self.range_type {
            DatePeriod::Year => NaiveDate::from_ymd_opt(self.year, 12, 31).unwrap_or_default(),
            DatePeriod::Quarter => self
                .get_first_day()
                .checked_add_months(Months::new(3))
                .unwrap_or_default()
                .pred_opt()
                .unwrap_or_default(),
            DatePeriod::Month => self
                .get_first_day()
                .checked_add_months(Months::new(1))
                .unwrap_or_default()
                .pred_opt()
                .unwrap_or_default(),
            DatePeriod::Daily => self.get_first_day(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use serde_json;

    #[test]
    fn test_range_date_serialization() -> anyhow::Result<()> {
        let range_date = RangeDate::new(2024, DatePeriod::Month, 5)?;
        let serialized = serde_json::to_string(&range_date).unwrap();
        assert_eq!(serialized, "\"2024M5\"");

        let deserialized: RangeDate = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, range_date);

        Ok(())
    }

    #[test]
    fn test_range_date_from_str() {
        let rd = RangeDate::from_str("2024Y1").unwrap();
        assert_eq!(rd.year, 2024);
        assert_eq!(rd.range_type, DatePeriod::Year);
        assert_eq!(rd.range_index, 1);

        let rd = RangeDate::from_str("2024M5").unwrap();
        assert_eq!(rd.year, 2024);
        assert_eq!(rd.range_type, DatePeriod::Month);
        assert_eq!(rd.range_index, 5);

        let rd = RangeDate::from_str("2024Q2").unwrap();
        assert_eq!(rd.year, 2024);
        assert_eq!(rd.range_type, DatePeriod::Quarter);
        assert_eq!(rd.range_index, 2);

        let rd = RangeDate::from_str("2024D150").unwrap();
        assert_eq!(rd.year, 2024);
        assert_eq!(rd.range_type, DatePeriod::Daily);
        assert_eq!(rd.range_index, 150);

        assert!(RangeDate::from_str("2024X1").is_err());
        assert!(RangeDate::from_str("invalid").is_err());
    }

    #[test]
    fn test_range_date_display() -> anyhow::Result<()> {
        let range_date = RangeDate::new(2024, DatePeriod::Daily, 150)?;
        assert_eq!(range_date.to_string(), "2024D150");
        Ok(())
    }

    #[test]
    fn test_range_date_from_naive_date() {
        let date = NaiveDate::from_ymd_opt(2024, 5, 15).unwrap();
        let rd = RangeDate::from_naive_date(&date, &DatePeriod::Month);
        assert_eq!(rd.year, 2024);
        assert_eq!(rd.range_type, DatePeriod::Month);
        assert_eq!(rd.range_index, 5);

        let rd = RangeDate::from_naive_date(&date, &DatePeriod::Quarter);
        assert_eq!(rd.year, 2024);
        assert_eq!(rd.range_type, DatePeriod::Quarter);
        assert_eq!(rd.range_index, 2);

        let rd = RangeDate::from_naive_date(&date, &DatePeriod::Year);
        assert_eq!(rd.year, 2024);
        assert_eq!(rd.range_type, DatePeriod::Year);
        assert_eq!(rd.range_index, 2024);

        let rd = RangeDate::from_naive_date(&date, &DatePeriod::Daily);
        assert_eq!(rd.year, 2024);
        assert_eq!(rd.range_type, DatePeriod::Daily);
        assert_eq!(rd.range_index, 136); // May 15 is the 136th day of the year
    }

    #[test]
    fn test_get_first_and_last_day() -> anyhow::Result<()> {
        let rd = RangeDate::new(2024, DatePeriod::Year, 2024)?;
        assert_eq!(
            rd.get_first_day(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );
        assert_eq!(
            rd.get_last_day(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );

        let rd = RangeDate::new(2024, DatePeriod::Quarter, 2)?;
        assert_eq!(
            rd.get_first_day(),
            NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()
        );
        assert_eq!(
            rd.get_last_day(),
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()
        );

        let rd = RangeDate::new(2024, DatePeriod::Month, 5)?;
        assert_eq!(
            rd.get_first_day(),
            NaiveDate::from_ymd_opt(2024, 5, 1).unwrap()
        );
        assert_eq!(
            rd.get_last_day(),
            NaiveDate::from_ymd_opt(2024, 5, 31).unwrap()
        );

        let rd = RangeDate::new(2024, DatePeriod::Daily, 136)?;
        assert_eq!(
            rd.get_first_day(),
            NaiveDate::from_yo_opt(2024, 136).unwrap()
        );
        assert_eq!(
            rd.get_last_day(),
            NaiveDate::from_yo_opt(2024, 136).unwrap()
        );

        Ok(())
    }

    #[test]
    fn test_leap_year() -> anyhow::Result<()> {
        let rd = RangeDate::new(2024, DatePeriod::Year, 2024)?;
        assert_eq!(
            rd.get_first_day(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
        );
        assert_eq!(
            rd.get_last_day(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );

        let rd = RangeDate::new(2024, DatePeriod::Month, 2)?;
        assert_eq!(
            rd.get_first_day(),
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()
        );
        assert_eq!(
            rd.get_last_day(),
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()
        );

        let rd = RangeDate::new(2023, DatePeriod::Month, 2)?;
        assert_eq!(
            rd.get_first_day(),
            NaiveDate::from_ymd_opt(2023, 2, 1).unwrap()
        );
        assert_eq!(
            rd.get_last_day(),
            NaiveDate::from_ymd_opt(2023, 2, 28).unwrap()
        );

        Ok(())
    }

    #[test]
    fn test_invalid_dates() {
        let rd = RangeDate::new(2024, DatePeriod::Month, 13);
        assert!(rd.is_err()); // Invalid month

        let rd = RangeDate::new(2024, DatePeriod::Daily, 366);
        assert!(rd.is_ok()); // Valid in leap year

        let rd = RangeDate::new(2023, DatePeriod::Daily, 366);
        assert!(rd.is_err()); // Invalid in non-leap year
    }

    #[test]
    fn test_display_trait() -> anyhow::Result<()> {
        let rd = RangeDate::new(2024, DatePeriod::Quarter, 3)?;
        assert_eq!(rd.to_string(), "2024Q3");

        Ok(())
    }
}
