pub fn is_leap_year(year: u64) -> bool {
    let four: bool = year % 4 == 0;
    let hundred: bool = year % 100 == 0;
    let fourhundred: bool = year % 400 == 0;

    (four && !hundred) || fourhundred
}
