extern crate gigasecond;
extern crate chrono;
use chrono::*;

fn main() {
    let start_date = Utc.ymd(2011, 4, 25).and_hms(0,0,0);
    gigasecond::after(start_date);
}