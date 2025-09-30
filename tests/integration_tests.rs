use chrono::NaiveDate;
use range_date::range_type::DatePeriod;
use std::str::FromStr;

#[test]
fn test_constructor_examples() {
    println!("=== Constructor Examples ===");

    let year_2024 = DatePeriod::year(2024);
    let q2_2024 = DatePeriod::quarter(2024, 2).expect("Valid quarter");
    let may_2024 = DatePeriod::month(2024, 5).expect("Valid month");
    let day_136 = DatePeriod::daily(2024, 136).expect("Valid day");

    println!("Year: {}", year_2024);
    println!("Quarter: {}", q2_2024);
    println!("Month: {}", may_2024);
    println!("Daily: {}", day_136);

    assert_eq!(year_2024.to_string(), "2024Y");
    assert_eq!(q2_2024.to_string(), "2024Q2");
    assert_eq!(may_2024.to_string(), "2024M5");
    assert_eq!(day_136.to_string(), "2024D136");
}

#[test]
fn test_parsing_from_strings() {
    println!("=== Parsing from Strings ===");

    let parsed_quarter = DatePeriod::parse("2024Q3").expect("Should parse quarter");
    let parsed_month = DatePeriod::from_str("2024M12").expect("Should parse month");

    println!("Parsed quarter: {}", parsed_quarter);
    println!("Parsed month: {}", parsed_month);

    assert_eq!(parsed_quarter, DatePeriod::Quarter(2024, 3));
    assert_eq!(parsed_month, DatePeriod::Month(2024, 12));
}

#[test]
fn test_date_conversion() {
    println!("=== Converting from NaiveDate ===");

    let date = NaiveDate::from_ymd_opt(2024, 8, 15).unwrap();

    let as_year = DatePeriod::from_date_as_year(date);
    let as_quarter = DatePeriod::from_date_as_quarter(date);
    let as_month = DatePeriod::from_date_as_month(date);
    let as_daily = DatePeriod::from_date_as_daily(date);

    println!("Date {} as year: {}", date, as_year);
    println!("Date {} as quarter: {}", date, as_quarter);
    println!("Date {} as month: {}", date, as_month);
    println!("Date {} as daily: {}", date, as_daily);

    assert_eq!(as_year, DatePeriod::Year(2024));
    assert_eq!(as_quarter, DatePeriod::Quarter(2024, 3)); // August is Q3
    assert_eq!(as_month, DatePeriod::Month(2024, 8));
    assert_eq!(as_daily, DatePeriod::Daily(2024, 228)); // 228th day of 2024
}

#[test]
fn test_date_range_operations() {
    println!("=== Date Range Examples ===");

    let q1_2024 = DatePeriod::quarter(2024, 1).expect("Valid quarter");

    let first_day = q1_2024.get_first_day().expect("Should get first day");
    let last_day = q1_2024.get_last_day().expect("Should get last day");

    println!("{} starts on: {}", q1_2024, first_day);
    println!("{} ends on: {}", q1_2024, last_day);

    // Check if a date is contained
    let test_date = NaiveDate::from_ymd_opt(2024, 2, 14).unwrap(); // Valentine's Day
    let contains = q1_2024.contains_date(test_date);
    println!("Does {} contain {}? {}", q1_2024, test_date, contains);

    assert_eq!(first_day, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    assert_eq!(last_day, NaiveDate::from_ymd_opt(2024, 3, 31).unwrap());
    assert!(contains);
}

#[test]
fn test_component_access() {
    println!("=== Component Access ===");

    let period = DatePeriod::quarter(2024, 3).expect("Valid quarter");

    println!("Period: {}", period);
    println!("Year: {}", period.get_year());
    println!("Value: {}", period.value());
    println!("Type: {}", period.period_name());
    println!("Short name: {}", period.short_name());

    assert_eq!(period.get_year(), 2024);
    assert_eq!(period.value(), 3);
    assert_eq!(period.period_name(), "QUARTER");
    assert_eq!(period.short_name(), "Q");
}

#[test]
fn test_json_serialization() {
    println!("=== JSON Serialization ===");

    let period = DatePeriod::quarter(2024, 3).expect("Valid quarter");

    let json = serde_json::to_string(&period).expect("Should serialize");
    println!("Serialized: {}", json);

    let deserialized: DatePeriod = serde_json::from_str(&json).expect("Should deserialize");
    println!("Deserialized: {}", deserialized);

    assert_eq!(json, "\"2024Q3\"");
    assert_eq!(period, deserialized);
}

#[test]
fn test_validation_examples() {
    println!("=== Validation Examples ===");

    // Test invalid quarter
    match DatePeriod::quarter(2024, 5) {
        Ok(_) => panic!("Quarter 5 should not be valid!"),
        Err(e) => println!("Invalid quarter caught: {}", e),
    }

    // Test invalid month
    match DatePeriod::month(2024, 13) {
        Ok(_) => panic!("Month 13 should not be valid!"),
        Err(e) => println!("Invalid month caught: {}", e),
    }

    // Test invalid day for non-leap year
    match DatePeriod::daily(2023, 366) {
        Ok(_) => panic!("Day 366 in 2023 should not be valid!"),
        Err(e) => println!("Invalid day for non-leap year caught: {}", e),
    }

    // All validation tests passed if we reach here
    println!("All validation tests completed successfully!");
}
