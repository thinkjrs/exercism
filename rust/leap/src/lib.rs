pub fn is_leap_year(year: u64) -> bool {
    //todo!("true if {year} is a leap year")
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true;
            }
            return false;
        }
        return true;
    } else {
        return false;
    }
}
