use chrono::{
    Datelike, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime,
    TimeZone, Timelike, Utc,
};
use chrono_tz::Asia;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_date() {
    let date = NaiveDate::from_ymd_opt(2025, 01, 15).unwrap();
    println!("Day {}", date.day());
    println!("Month {}", date.month());
    println!("Year {}", date.year());
}

#[test]
fn test_duration() {
    let date = NaiveDate::from_ymd_opt(2025, 01, 15).unwrap();
    let new_date = date + Duration::days(20);

    println!("Old date is {}", date);
    println!("New date is {}", new_date);
}

#[test]
fn test_time() {
    let time = NaiveTime::from_hms_milli_opt(10, 30, 21, 100).unwrap();
    println!("Time is {}", time);
    println!("Nano is {}", time.minute());
}

#[test]
fn test_datetime() {
    let date_time = NaiveDate::from_ymd_opt(2021, 10, 1)
        .unwrap()
        .and_time(NaiveTime::from_hms_opt(10, 12, 10).unwrap());

    println!("Date time is {}", date_time);

    println!("Date format is {}", date_time.format("%d-%m-%Y"));
}

#[test]
fn test_offset() {
    let utc_date_time = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 01, 20).unwrap(),
        NaiveTime::from_hms_opt(10, 20, 10).unwrap(),
    );

    let asia_jakarta = FixedOffset::east_opt(7 * 3600).unwrap();
    let asia_jakarta_date_time = asia_jakarta.from_utc_datetime(&utc_date_time);

    println!("UTC : {}", utc_date_time);
    println!("WIB : {}", asia_jakarta_date_time);
}

#[test]
fn test_time_zone() {
    let utc_date_time = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 01, 20).unwrap(),
        NaiveTime::from_hms_opt(10, 20, 10).unwrap(),
    );

    let asia_jakarta = Asia::Jakarta;
    let asia_jakarta_date_time = asia_jakarta.from_utc_datetime(&utc_date_time);

    println!("UTC : {}", utc_date_time);
    println!("WIB : {}", asia_jakarta_date_time);
}

#[test]
fn test_date_time_with_time_zone() {
    // Ini digunakan untuk mengambil now dalam UTC
    let utc_date_time = Utc::now();
    let asia_jakarta_date_time = Asia::Jakarta.from_utc_datetime(&utc_date_time.naive_utc());

    println!("{}", utc_date_time);
    println!("{}", asia_jakarta_date_time);

    let local_date_time = Local::now();
    let asia_jakarta_date_time = Asia::Jakarta
        .from_local_datetime(&local_date_time.naive_local())
        .unwrap();
    println!("{}", local_date_time);
    println!("{}", asia_jakarta_date_time);
}

#[test]
fn test_parse() {
    let date_string = "2024-10-25 10:09:08".to_string();
    let time = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap();

    println!("{}", time);
}
