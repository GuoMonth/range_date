pub mod range_date;
pub mod range_type;

pub const fn leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
