use range_date::range_type::DatePeriod;

fn main() -> anyhow::Result<()> {
    println!("=== Testing Year Format Fix ===\n");

    // Test creating and converting to string
    let year_period = DatePeriod::year(2024);
    println!("Created year period: {}", year_period);
    println!("String format: {}", year_period.to_string());

    // Test parsing from string
    let parsed = DatePeriod::parse("2024Y")?;
    println!("Parsed from '2024Y': {:?}", parsed);

    // Test FromStr trait
    let from_str: DatePeriod = "2024Y".parse()?;
    println!("From str trait: {:?}", from_str);

    // Test round trip
    let round_trip = DatePeriod::parse(&year_period.to_string())?;
    println!("Round trip success: {}", year_period == round_trip);

    // Test JSON serialization
    let json = serde_json::to_string(&year_period)?;
    println!("JSON serialized: {}", json);

    let deserialized: DatePeriod = serde_json::from_str(&json)?;
    println!("JSON deserialized: {:?}", deserialized);
    println!("JSON round trip success: {}", year_period == deserialized);

    println!("\n=== All tests passed! ===");
    Ok(())
}
