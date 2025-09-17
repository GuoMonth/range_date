# range_date

[![Crates.io](https://img.shields.io/crates/v/range_date.svg)](https://crates.io/crates/range_date)
[![Documentation](https://docs.rs/range_date/badge.svg)](https://docs.rs/range_date)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust crate for handling date ranges with support for years, quarters, months, and days.

## Features

- 🗓️ **Multiple Date Periods**: Support for Year(Y), Quarter(Q), Month(M), and Day(D) time periods
- 🔄 **Type Conversion**: String parsing and serialization support
- 📅 **Date Calculations**: Utilities like leap year detection
- ⚡ **High Performance**: Built on top of the efficient `chrono` library
- 🛡️ **Type Safety**: Complete type system with proper error handling

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
range_date = "0.1.0"
```

## Usage Examples

### Basic Usage

```rust
use range_date::{range_date::RangeDate, range_type::DatePeriod};
use std::str::FromStr;

// Create a date range for Q1 2024
let range = RangeDate {
    year: 2024,
    range_type: DatePeriod::Quarter,
    range_index: 1,
};

// String representation
println!("{}", range); // Output: 2024Q1

// Parse from string
let parsed = RangeDate::from_str("2024M03").unwrap();
println!("{:?}", parsed); // RangeDate { year: 2024, range_type: Month, range_index: 3 }
```

### Date Period Types

```rust
use range_date::range_type::DatePeriod;

// Supported time period types
let year = DatePeriod::new("Y").unwrap();      // Year
let quarter = DatePeriod::new("Q").unwrap();   // Quarter  
let month = DatePeriod::new("M").unwrap();     // Month
let daily = DatePeriod::new("D").unwrap();     // Daily
```

### Serialization Support

```rust
use range_date::range_date::RangeDate;
use std::str::FromStr;
use serde_json;

let range = RangeDate::from_str("2024Q2").unwrap();
let json = serde_json::to_string(&range).unwrap();
println!("{}", json); // "2024Q2"
```

### Leap Year Detection

```rust
use range_date::leap_year;

assert_eq!(leap_year(2024), true);   // Leap year
assert_eq!(leap_year(2023), false);  // Not a leap year
assert_eq!(leap_year(2000), true);   // Leap year (divisible by 400)
assert_eq!(leap_year(1900), false);  // Not a leap year (divisible by 100 but not 400)
```

## Date Range Format

The crate uses a compact string format for date ranges:

- **Format**: `YYYY[PERIOD][INDEX]`
- **Examples**:
  - `2024Y1` - Year 2024
  - `2024Q2` - Q2 2024 (April-June)
  - `2024M03` - March 2024
  - `2024D060` - 60th day of 2024

## API Documentation

For complete API documentation, visit [docs.rs](https://docs.rs/range_date).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.