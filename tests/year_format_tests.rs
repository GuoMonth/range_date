use range_date::range_type::DatePeriod;

#[test]
fn test_year_format_creation_and_string_conversion() {
    let year_period = DatePeriod::year(2024);
    println!("Created year period: {}", year_period);

    let string_format = year_period.to_string();
    println!("String format: {}", string_format);

    assert_eq!(string_format, "2024Y");
}

#[test]
fn test_year_format_parsing() {
    let parsed = DatePeriod::parse("2024Y").expect("Should parse successfully");
    println!("Parsed from '2024Y': {:?}", parsed);

    assert_eq!(parsed, DatePeriod::Year(2024));
}

#[test]
fn test_year_format_from_str_trait() {
    let from_str: DatePeriod = "2024Y".parse().expect("Should parse via FromStr");
    println!("From str trait: {:?}", from_str);

    assert_eq!(from_str, DatePeriod::Year(2024));
}

#[test]
fn test_year_format_round_trip() {
    let year_period = DatePeriod::year(2024);
    let round_trip = DatePeriod::parse(&year_period.to_string()).expect("Round trip should work");
    println!("Round trip success: {}", year_period == round_trip);

    assert_eq!(year_period, round_trip);
}

#[test]
fn test_year_format_json_serialization() {
    let year_period = DatePeriod::year(2024);

    let json = serde_json::to_string(&year_period).expect("Should serialize to JSON");
    println!("JSON serialized: {}", json);

    let deserialized: DatePeriod =
        serde_json::from_str(&json).expect("Should deserialize from JSON");
    println!("JSON deserialized: {:?}", deserialized);
    println!("JSON round trip success: {}", year_period == deserialized);

    assert_eq!(json, "\"2024Y\"");
    assert_eq!(year_period, deserialized);
}
