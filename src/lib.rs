//! # range_date
//!
//! A Rust crate for handling date ranges with support for years, quarters, months, and days.
//!
//! ## Main Components
//!
//! - [`range_type::DatePeriod`] - Enum defining date periods with embedded data (Year/Quarter/Month/Day)
//! - [`leap_year`] - Utility function to determine if a year is a leap year
//!
//! ## Quick Example
//!
//! ```rust
//! use range_date::range_type::DatePeriod;
//! use std::str::FromStr;
//!
//! // Create Q1 2024
//! let range = DatePeriod::quarter(2024, 1).unwrap();
//!
//! // String representation: "2024Q1"  
//! println!("{}", range);
//!
//! // Parse from string
//! let parsed = DatePeriod::from_str("2024M03").unwrap();
//!
//! // Get date range
//! let first_day = range.get_first_day()?;
//! let last_day = range.get_last_day()?;
//! ```

pub mod range_type;

/// Determines if a given year is a leap year
///
/// # Rules
/// - Years divisible by 4 but not by 100 are leap years
/// - Years divisible by 400 are leap years
///
/// # Examples
///
/// ```rust
/// use range_date::leap_year;
///
/// assert_eq!(leap_year(2024), true);   // Leap year
/// assert_eq!(leap_year(2023), false);  // Not a leap year
/// assert_eq!(leap_year(2000), true);   // Leap year (divisible by 400)  
/// assert_eq!(leap_year(1900), false);  // Not a leap year (divisible by 100 but not 400)
/// ```
pub const fn leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
