// --------------------------------------------
// ---------- Faction Member Counter ----------
// --------------------------------------------
/// A custom struct to handle the custom date mechanics
pub struct CustomDate {
    days_per_month: u32,
    days_per_year: u32,
    months_per_year: u32,
    days_since_year_zero: u32,
    // day: u32,
    // month: u32,
    // year: u32,
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

        let days_per_year = months_per_year * days_per_month;
        let days_since_year_zero: u32 = year * days_per_year + month * days_per_month + day;

        Self {
            days_per_month: days_per_month,
            months_per_year: months_per_year,
            days_since_year_zero: days_since_year_zero,
            days_per_year: days_per_year,
        }
    }

    /// Puts together and then returns the date in the format of a string
    pub fn get_date_as_string(&self) -> String {
        let mut days_since_year_zero = self.days_since_year_zero;

        let years = days_since_year_zero / self.days_per_year;
        days_since_year_zero -= years * self.days_per_year;
        let months = days_since_year_zero / self.days_per_month;
        days_since_year_zero -= months * self.days_per_month;
        let days = days_since_year_zero;

        let date_as_string = format!("{:02}/{:02}/{:04}", days, months, years);
        return date_as_string;
    }

    /// Allows the addition of days to the date
    pub fn add_days(&mut self, days_to_add: u32) {
        self.days_since_year_zero += days_to_add;
    }

    // I'm thinking probebly swap it to use "Days since the year 0" and find out the date based on that.
    // This has an error which causes it to still add a year to the date if it's only at 10 months
    /// Allows the addition of months to the date
    pub fn add_months(&mut self, months_to_add: u32) {
        self.days_since_year_zero += months_to_add * self.days_per_month;
    }

    /// Allows the addition of years to the date
    pub fn add_years(&mut self, years_to_add: u32) {
        self.days_since_year_zero += years_to_add * self.days_per_year;
    }
}
