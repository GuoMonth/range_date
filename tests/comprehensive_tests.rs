use chrono::NaiveDate;
use range_date::range_type::DatePeriod;

#[test]
fn test_comprehensive_period_operations() {
    println!("=== Comprehensive Period Operations Test ===");

    // Test all period types creation
    let year = DatePeriod::year(2024);
    let quarter = DatePeriod::quarter(2024, 2).expect("Valid quarter");
    let month = DatePeriod::month(2024, 6).expect("Valid month");
    let daily = DatePeriod::daily(2024, 182).expect("Valid day"); // June 30th

    println!(
        "Created periods: {}, {}, {}, {}",
        year, quarter, month, daily
    );

    // Test all belong to same year
    assert_eq!(year.get_year(), 2024);
    assert_eq!(quarter.get_year(), 2024);
    assert_eq!(month.get_year(), 2024);
    assert_eq!(daily.get_year(), 2024);

    // Test quarter contains month
    let june_1st = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
    assert!(quarter.contains_date(june_1st));
    assert!(month.contains_date(june_1st));
}

#[test]
fn test_leap_year_edge_cases() {
    println!("=== Leap Year Edge Cases ===");

    // 2024 is a leap year - should allow day 366
    let leap_day = DatePeriod::daily(2024, 366).expect("2024 should allow day 366");
    println!("Leap year day 366: {}", leap_day);

    // 2023 is not a leap year - should reject day 366
    assert!(DatePeriod::daily(2023, 366).is_err());
    println!("2023 correctly rejects day 366");

    // But should allow day 365
    let regular_day = DatePeriod::daily(2023, 365).expect("2023 should allow day 365");
    println!("Non-leap year day 365: {}", regular_day);
}

#[test]
fn test_period_boundaries() {
    println!("=== Period Boundaries Test ===");

    // Test Q4 boundaries (Oct-Dec)
    let q4 = DatePeriod::quarter(2024, 4).expect("Valid Q4");
    let q4_start = q4.get_first_day().expect("Should get Q4 start");
    let q4_end = q4.get_last_day().expect("Should get Q4 end");

    println!("Q4 2024: {} to {}", q4_start, q4_end);

    assert_eq!(q4_start, NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
    assert_eq!(q4_end, NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());

    // Test February in leap year
    let feb_2024 = DatePeriod::month(2024, 2).expect("Valid February");
    let feb_end = feb_2024.get_last_day().expect("Should get February end");

    println!("February 2024 ends on: {}", feb_end);
    assert_eq!(feb_end, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap()); // Leap year
}

#[test]
fn test_string_parsing_edge_cases() {
    println!("=== String Parsing Edge Cases ===");

    // Test with leading zeros
    let month_with_zero = DatePeriod::parse("2024M03").expect("Should parse M03");
    println!("Parsed M03: {}", month_with_zero);
    assert_eq!(month_with_zero, DatePeriod::Month(2024, 3));

    // Test daily with leading zeros
    let daily_with_zeros = DatePeriod::parse("2024D001").expect("Should parse D001");
    println!("Parsed D001: {}", daily_with_zeros);
    assert_eq!(daily_with_zeros, DatePeriod::Daily(2024, 1));

    // Test invalid formats
    assert!(DatePeriod::parse("2024").is_err());
    assert!(DatePeriod::parse("2024X1").is_err());
    assert!(DatePeriod::parse("24Q1").is_err()); // Year too short

    println!("Invalid format rejection works correctly");
}
