# range_date

[![Crates.io](https://img.shields.io/crates/v/range_date.svg)](https://crates.io/crates/range_date)
[![Documentation](https://docs.rs/range_date/badge.svg)](https://docs.rs/range_date)
[![codecov](https://codecov.io/github/GuoMonth/range_date/graph/badge.svg?token=LPXZVA7GSB)](https://codecov.io/github/GuoMonth/range_date)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust crate for handling date periods (year / quarter / month / day) with validation, parsing, navigation and range generation, built on top of [`chrono`](https://docs.rs/chrono).

## Installation

```toml
[dependencies]
range_date = "0.2.4"
```

## Period Types

| Period  | Constructor                        | String format | Range                |
| ------- | ---------------------------------- | ------------- | -------------------- |
| Year    | `DatePeriod::year(2024)`           | `2024Y`       | whole year           |
| Quarter | `DatePeriod::quarter(2024, 1)?`    | `2024Q1`      | quarter 1..=4        |
| Month   | `DatePeriod::month(2024, 3)?`      | `2024M3`      | month 1..=12         |
| Daily   | `DatePeriod::daily(2024, 60)?`     | `2024D60`     | ordinal day 1..=366  |

String format: `YYYY<TYPE>[INDEX]`, where `<TYPE>` is one of `Y` / `Q` / `M` / `D`.

## Usage

```rust
use range_date::range_type::DatePeriod;
use chrono::NaiveDate;
use std::str::FromStr;

// Construct & validate
let q1 = DatePeriod::quarter(2024, 1)?;
assert!(DatePeriod::month(2024, 13).is_err());

// Parse & display
let m: DatePeriod = DatePeriod::from_str("2024M03")?;
assert_eq!(q1.to_string(), "2024Q1");

// Date boundaries & containment
let first = q1.get_first_day()?;           // 2024-01-01
let last  = q1.get_last_day()?;            // 2024-03-31
assert!(q1.contains_date(NaiveDate::from_ymd_opt(2024, 2, 14).unwrap()));

// Convert from a date
let day = NaiveDate::from_ymd_opt(2024, 8, 15).unwrap();
let _ = DatePeriod::from_date_as_quarter(day);   // 2024Q3

// Navigate
let q2 = q1.succ()?;              // next   -> 2024Q2
let q4 = q1.succ_n(3)?;           // +3     -> 2024Q4
let _  = q4.pred_n(3)?;           // -3     -> 2024Q1
let _  = q1.offset_n(-1)?;        // signed -> 2023Q4

// Range generation
let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let end   = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();
assert_eq!(DatePeriod::between_date_as_quarter(start, end)?.len(), 2);

// Decompose / aggregate
assert_eq!(DatePeriod::year(2024).decompose().len(), 4);
assert_eq!(DatePeriod::month(2024, 5)?.aggregate(),
           DatePeriod::quarter(2024, 2)?);

// Serde
let json = serde_json::to_string(&q1)?;   // "\"2024Q1\""
let back: DatePeriod = serde_json::from_str(&json)?;
assert_eq!(back, q1);
# Ok::<(), anyhow::Error>(())
```

## Documentation

Full API reference on [docs.rs](https://docs.rs/range_date).

## Contributing

Issues and pull requests are welcome. Before submitting, please run:

```bash
cargo fmt --all
cargo clippy --all-features -- -D warnings
cargo test
```

## License

Licensed under the [MIT License](LICENSE).
