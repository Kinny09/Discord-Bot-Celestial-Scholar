use chrono::format::Item::Error;

// --------------------------------------------
// ---------- Faction Member Counter ----------
// --------------------------------------------
/// A custom struct to handle the custom date mechanics
pub struct CustomDate {
    days_per_month: u32,
    months_per_year: u32,
    day: u32,
    month: u32,
    year: u32,
}

impl CustomDate {
    pub fn new(date_string: String, days_per_month: u32, months_per_year: u32) -> Self {
        let split_date_string: Vec<&str> = date_string.split("/").collect();

        // Checking everything is the correct length for a date
        if split_date_string.len() != 3 {
            panic!(
                "DATE SIZE ERROR: CustomDate must have three items in the format: DD/MM/YYYY, the given custom date has {} items.",
                split_date_string.len()
            );
        }

        // Checking everything can be turned into a usize
        for potential_usize in split_date_string.clone() {
            if potential_usize.parse::<u32>().is_err() {
                panic!(
                    "DATE TYPE ERROR: CustomDate only accepts u32's as inputs for the different date inputs, {} does not match that format",
                    date_string
                );
            }
        }

        // Unwrapping the data from the split date string vector
        let day = split_date_string[0].parse::<u32>().unwrap();
        let month = split_date_string[1].parse::<u32>().unwrap();
        let year = split_date_string[2].parse::<u32>().unwrap();

        // Checking if the day's and months are possible given the number of days/months in a month/year
        if day > days_per_month && month > months_per_year {
            panic!(
                "DATE SIZE ERROR: The max amount of days and months you may have in this CustomDate is: {}/{}/YYYY. The given date {} is either too big on the month or day section.",
                days_per_month, months_per_year, date_string
            );
        }

        Self {
            days_per_month: days_per_month,
            months_per_year: months_per_year,
            day: day,
            month: month,
            year: year,
        }
    }

    /// Puts together and then returns the date in the format of a string
    pub fn get_date_as_string() -> String {
        panic!("Not Implemented")
    }

    /// Allows the addition of days to the date
    pub fn add_days() {
        panic!("Not Implemented")
    }

    /// Allows the addition of months to the date
    pub fn add_months() {
        panic!("Not Implemented")
    }
    /// Allows the addition of years to the date
    pub fn add_years() {
        panic!("Not Implemented")
    }
}
