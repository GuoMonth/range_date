use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatePeriod {
    Year,
    Quarter,
    Month,
    Daily,
}

impl Serialize for DatePeriod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.short_name())
    }
}

impl<'de> Deserialize<'de> for DatePeriod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<DatePeriod, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DatePeriod::new(&s).map_err(serde::de::Error::custom)
    }
}

impl std::str::FromStr for DatePeriod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DatePeriod::new(s)
    }
}

impl std::fmt::Display for DatePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatePeriod::Year => write!(f, "YEAR"),
            DatePeriod::Quarter => write!(f, "QUARTER"),
            DatePeriod::Month => write!(f, "MONTH"),
            DatePeriod::Daily => write!(f, "DAILY"),
        }
    }
}

impl DatePeriod {
    /// Create a new RangeType from a string representation.
    /// - Accepts "YEAR", "Y", "QUARTER", "Q", "MONTH", "M", "DAILY", "D" (case insensitive).
    /// - Returns an error for invalid inputs.
    /// # Examples
    /// ```
    /// use range_date::range_type::RangeType;
    ///
    /// let rt = RangeType::new("Y").unwrap();
    /// assert_eq!(rt, RangeType::YEAR);
    /// let rt = RangeType::new("MONTH").unwrap();
    /// assert_eq!(rt, RangeType::MONTH);
    /// let rt = RangeType::new("invalid");
    /// assert!(rt.is_err());
    /// ```
    pub fn new(range_type: &str) -> anyhow::Result<Self> {
        match range_type.trim().to_uppercase().as_str() {
            "YEAR" | "Y" => Ok(DatePeriod::Year),
            "QUARTER" | "Q" => Ok(DatePeriod::Quarter),
            "MONTH" | "M" => Ok(DatePeriod::Month),
            "DAILY" | "D" => Ok(DatePeriod::Daily),
            _ => Err(anyhow::anyhow!("Invalid time period type: {}", range_type)),
        }
    }

    pub fn short_name(&self) -> &str {
        match self {
            DatePeriod::Year => "Y",
            DatePeriod::Quarter => "Q",
            DatePeriod::Month => "M",
            DatePeriod::Daily => "D",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use serde_json;

    #[test]
    fn test_range_type_serialization() {
        let range_type = DatePeriod::Month;
        let serialized = serde_json::to_string(&range_type).unwrap();
        assert_eq!(serialized, "\"M\"");

        let deserialized: DatePeriod = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, DatePeriod::Month);
    }

    #[test]
    fn test_range_type_from_str() {
        assert_eq!(DatePeriod::from_str("Y").unwrap(), DatePeriod::Year);
        assert_eq!(DatePeriod::from_str("Q").unwrap(), DatePeriod::Quarter);
        assert_eq!(DatePeriod::from_str("M").unwrap(), DatePeriod::Month);
        assert_eq!(DatePeriod::from_str("D").unwrap(), DatePeriod::Daily);
        assert!(DatePeriod::from_str("INVALID").is_err());
    }

    #[test]
    fn test_range_type_display() {
        let range_type = DatePeriod::Daily;
        assert_eq!(format!("{}", range_type), "DAILY");
    }
}
