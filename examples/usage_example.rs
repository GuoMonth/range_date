use chrono::NaiveDate;
use range_date::range_type::DatePeriod;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    println!("=== DatePeriod Usage Examples ===\n");

    // 1. Creating periods using constructors
    println!("1. Constructor Examples:");
    let year_2024 = DatePeriod::year(2024);
    let q2_2024 = DatePeriod::quarter(2024, 2)?;
    let may_2024 = DatePeriod::month(2024, 5)?;
    let day_136 = DatePeriod::daily(2024, 136)?; // May 15th, 2024

    println!("  Year: {}", year_2024);
    println!("  Quarter: {}", q2_2024);
    println!("  Month: {}", may_2024);
    println!("  Daily: {}", day_136);

    // 2. Parsing from strings
    println!("\n2. Parsing from Strings:");
    let parsed_quarter = DatePeriod::parse("2024Q3")?;
    let parsed_month = DatePeriod::from_str("2024M12")?;

    println!("  Parsed quarter: {}", parsed_quarter);
    println!("  Parsed month: {}", parsed_month);

    // 3. Converting from dates
    println!("\n3. Converting from NaiveDate:");
    let date = NaiveDate::from_ymd_opt(2024, 8, 15).unwrap();

    let as_year = DatePeriod::from_date_as_year(date);
    let as_quarter = DatePeriod::from_date_as_quarter(date);
    let as_month = DatePeriod::from_date_as_month(date);
    let as_daily = DatePeriod::from_date_as_daily(date);

    println!("  Date {} as year: {}", date, as_year);
    println!("  Date {} as quarter: {}", date, as_quarter);
    println!("  Date {} as month: {}", date, as_month);
    println!("  Date {} as daily: {}", date, as_daily);

    // 4. Getting date ranges
    println!("\n4. Date Range Examples:");
    let q1_2024 = DatePeriod::quarter(2024, 1)?;

    println!("  {} starts on: {}", q1_2024, q1_2024.get_first_day()?);
    println!("  {} ends on: {}", q1_2024, q1_2024.get_last_day()?);

    // Check if a date is contained
    let test_date = NaiveDate::from_ymd_opt(2024, 2, 14).unwrap(); // Valentine's Day
    println!(
        "  Does {} contain {}? {}",
        q1_2024,
        test_date,
        q1_2024.contains_date(test_date)
    );

    // 5. Getting components
    println!("\n5. Component Access:");
    let period = DatePeriod::quarter(2024, 3)?;

    println!("  Period: {}", period);
    println!("  Year: {}", period.get_year());
    println!("  Value: {}", period.value());
    println!("  Type: {}", period.period_name());
    println!("  Short name: {}", period.short_name());

    // 6. Serialization
    println!("\n6. JSON Serialization:");
    let json = serde_json::to_string(&period)?;
    println!("  Serialized: {}", json);

    let deserialized: DatePeriod = serde_json::from_str(&json)?;
    println!("  Deserialized: {}", deserialized);

    // 7. Validation examples
    println!("\n7. Validation Examples:");
    match DatePeriod::quarter(2024, 5) {
        Ok(_) => println!("  Quarter 5 should not be valid!"),
        Err(e) => println!("  Invalid quarter caught: {}", e),
    }

    match DatePeriod::month(2024, 13) {
        Ok(_) => println!("  Month 13 should not be valid!"),
        Err(e) => println!("  Invalid month caught: {}", e),
    }

    match DatePeriod::daily(2023, 366) {
        Ok(_) => println!("  Day 366 in 2023 should not be valid!"),
        Err(e) => println!("  Invalid day for non-leap year caught: {}", e),
    }

    println!("\n=== All examples completed successfully! ===");
    Ok(())
}
