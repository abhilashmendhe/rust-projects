use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[inline]
pub fn remove_prefix(date_str: &str) -> &str {
    date_str.trim_start_matches("Reviewed ")
}

// fn separate_
// #[derive(Debug, Deserialize, Serialize)]
#[repr(u32)]
enum DirtyMonths {
    Jan = 1,
    Feb = 2,
    March = 3,
    April = 4,
    May = 5, 
    June = 6,
    July = 7,
    Aug = 8,
    Sept = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12,
}

impl std::str::FromStr for DirtyMonths {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Jan." => Ok(DirtyMonths::Jan),
            "Feb." => Ok(DirtyMonths::Feb),
            "March." => Ok(DirtyMonths::March),
            "April." => Ok(DirtyMonths::April),
            "May." => Ok(DirtyMonths::May),
            "June." => Ok(DirtyMonths::June),
            "July." => Ok(DirtyMonths::July),
            "Aug." => Ok(DirtyMonths::Aug),
            "Sept." => Ok(DirtyMonths::Sept),
            "Oct." => Ok(DirtyMonths::Oct),
            "Nov." => Ok(DirtyMonths::Nov),
            "Dec." => Ok(DirtyMonths::Dec),
            _ => Err("Invalid Month")
        }
    }
}

impl Into<u32> for DirtyMonths {
    fn into(self) -> u32 {
        self as u32
    }
}
fn date_str_into_two_parts(date_str: &str) -> Option<(&str, &str, &str)> {
    let mut parts = date_str.split_whitespace();
    let month = parts.next()?;
    let day = parts.next()?;
    let year = parts.next()?;
    Some((month, day, year))
}
pub fn convert_date(date_str: &str) -> Option<NaiveDate> {
    let date_str = remove_prefix(date_str);
    
    let (month_str, day_str, year_str) = date_str_into_two_parts(date_str)?;
    let month = month_str.parse::<DirtyMonths>().ok()?.into();
    let day = day_str.trim_end_matches(",").parse::<i32>().ok()?;
    let year = year_str.parse::<u32>().ok()?;
    

    None
}
#[cfg(test)]
mod tests {
    use super::*;
    const DATE_STR: &str = "Reviewed May 26, 2023";

    #[test]
    fn test_remove_prefix() {
        let date_str = remove_prefix(DATE_STR);
        assert_eq!(date_str, "May 26, 2023");
    }
    #[test]
    fn test_date_convert() {

        let converted_date = convert_date(DATE_STR).unwrap();
        assert_eq!(converted_date, NaiveDate::from_ymd_opt(2023, 05, 26).unwrap());

    }
}