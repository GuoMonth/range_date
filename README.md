# range_date

[![Crates.io](https://img.shields.io/crates/v/range_date.svg)](https://crates.io/crates/range_date)
[![Documentation](https://docs.rs/range_date/badge.svg)](https://docs.rs/range_date)
[![codecov](https://codecov.io/github/GuoMonth/range_date/graph/badge.svg?token=LPXZVA7GSB)](https://codecov.io/github/GuoMonth/range_date)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful Rust crate for handling date periods with embedded data and comprehensive date range operations.

## Features

- 🗓️ **Rich Date Periods**: Enum-based periods with embedded year/index data - Year(u32), Quarter(u32, u32), Month(u32, u32), Daily(u32, u32)
- 🔄 **Type Conversion**: String parsing and serialization support with validation
- 📅 **Date Range Operations**: Calculate first/last days, check date containment
- 🎯 **Date Conversion**: Convert NaiveDate to any period type
- ⚡ **High Performance**: Built on top of the efficient `chrono` library
- 🛡️ **Type Safety**: Complete validation with proper error handling
- 🧪 **Leap Year Support**: Accurate leap year detection and day validation

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
range_date = "0.2.0"
```

## Usage Examples

### Basic Usage

```rust
use range_date::range_type::DatePeriod;
use std::str::FromStr;

// Create date periods with validation
let q1_2024 = DatePeriod::quarter(2024, 1).unwrap();
let may_2024 = DatePeriod::month(2024, 5).unwrap();

// String representation
println!("{}", q1_2024); // Output: 2024Q1
println!("{}", may_2024); // Output: 2024M5

// Parse from string
let parsed = DatePeriod::from_str("2024M03").unwrap();
println!("{:?}", parsed); // DatePeriod::Month(2024, 3)

// Get date ranges
let first_day = q1_2024.get_first_day()?; // 2024-01-01
let last_day = q1_2024.get_last_day()?;   // 2024-03-31
```

### Date Period Construction

```rust
use range_date::range_type::DatePeriod;

// Create with validation
let year_2024 = DatePeriod::year(2024);
let q2_2024 = DatePeriod::quarter(2024, 2).unwrap();   // Q2: April-June
let march_2024 = DatePeriod::month(2024, 3).unwrap();  // March
let day_60 = DatePeriod::daily(2024, 60).unwrap();     // 60th day of 2024

// Validation catches errors
assert!(DatePeriod::quarter(2024, 5).is_err()); // Quarter 5 doesn't exist
assert!(DatePeriod::month(2024, 13).is_err());  // Month 13 doesn't exist
assert!(DatePeriod::daily(2023, 366).is_err()); // Day 366 invalid in non-leap year
```

### Date Conversion

```rust
use range_date::range_type::DatePeriod;
use chrono::NaiveDate;

let date = NaiveDate::from_ymd_opt(2024, 8, 15).unwrap(); // August 15, 2024

// Convert to different period types
let as_year = DatePeriod::from_date_as_year(date);     // 2024Y
let as_quarter = DatePeriod::from_date_as_quarter(date); // 2024Q3
let as_month = DatePeriod::from_date_as_month(date);     // 2024M8
let as_daily = DatePeriod::from_date_as_daily(date);     // 2024D228
```

### Date Range Operations

```rust
use range_date::range_type::DatePeriod;
use chrono::NaiveDate;

let q1_2024 = DatePeriod::quarter(2024, 1).unwrap();

// Get date boundaries
let start = q1_2024.get_first_day()?; // 2024-01-01
let end = q1_2024.get_last_day()?;    // 2024-03-31

// Check date containment
let valentine = NaiveDate::from_ymd_opt(2024, 2, 14).unwrap();
let contains = q1_2024.contains_date(valentine); // true
```

### Serialization Support

```rust
use range_date::range_type::DatePeriod;
use serde_json;

let quarter = DatePeriod::quarter(2024, 2).unwrap();
let json = serde_json::to_string(&quarter).unwrap();
println!("{}", json); // "2024Q2"

let deserialized: DatePeriod = serde_json::from_str(&json).unwrap();
assert_eq!(deserialized, DatePeriod::Quarter(2024, 2));
```

## Date Range Format

The crate uses a compact string format for date periods:

- **Format**: `YYYY[PERIOD][INDEX]`
- **Examples**:
  - `2024Y` - Year 2024
  - `2024Q2` - Q2 2024 (April-June)
  - `2024M03` - March 2024
  - `2024D060` / `2024D60` - 60th day of 2024

## Period Types

| Period | Constructor | String Format | Description |
|--------|-------------|---------------|-------------|
| Year | `DatePeriod::year(2024)` | `2024Y` | Entire year |
| Quarter | `DatePeriod::quarter(2024, 1)` | `2024Q1` | Q1: Jan-Mar, Q2: Apr-Jun, etc. |
| Month | `DatePeriod::month(2024, 3)` | `2024M3` | Specific month (1-12) |
| Daily | `DatePeriod::daily(2024, 60)` | `2024D60` | Specific day of year (1-366) |

## Version Upgrade Guide

### v0.2.0 - Major Testing Infrastructure Improvements

**⚠️ Breaking Changes:**

- **Testing Migration**: All example code has been converted to proper test cases in the `tests/` directory.

**Why This Change?**

1. **Better Development Experience**: Tests can now be run with simple `cargo test`
2. **Improved CI/CD**: Integration with continuous integration pipelines is now seamless
3. **Enhanced Maintainability**: Test code is better organized and easier to maintain
4. **Performance**: Tests run in parallel, significantly faster than sequential example execution


**New Testing Structure:**
```
tests/
├── year_format_tests.rs      # Year format functionality tests
├── integration_tests.rs      # Comprehensive usage examples as tests  
└── comprehensive_tests.rs    # Edge cases and boundary tests
```

**Benefits:**
- ✅ **29 comprehensive tests** covering all functionality
- ✅ **Parallel execution** for faster feedback
- ✅ **Better error reporting** with precise test failure information
- ✅ **Hand-verification support** with `--nocapture` flag to see debug output
- ✅ **CI/CD ready** for automated testing pipelines

## API Documentation

For complete API documentation, visit [docs.rs](https://docs.rs/range_date).

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run specific test files
cargo test --test integration_tests
cargo test --test year_format_tests

# Run tests with output for manual verification
cargo test test_constructor_examples -- --nocapture
cargo test test_year_format -- --nocapture
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

We welcome contributions! Here are several ways you can help:

### 🐛 Found a Bug?
- Open an [issue](https://github.com/GuoMonth/range_date/issues) with a clear description
- Include code examples that reproduce the problem
- Mention your Rust version and operating system

### 💡 Have a Feature Request?
- Check existing [issues](https://github.com/GuoMonth/range_date/issues) to avoid duplicates
- Open a new issue describing your use case and proposed solution
- Explain why this feature would be valuable

### 🔧 Want to Contribute Code?
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes with comprehensive tests
4. Ensure all tests pass (`cargo test`)
5. Run formatting (`cargo fmt`) and linting (`cargo clippy`)
6. Commit your changes (`git commit -am 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### 📖 Documentation Improvements?
- Documentation improvements are always welcome
- Update examples in README.md or code comments
- Help improve API documentation

**Have questions or suggestions? Please [open an issue](https://github.com/GuoMonth/range_date/issues) - we'd love to hear from you!**