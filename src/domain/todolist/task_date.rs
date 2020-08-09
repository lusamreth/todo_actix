use chrono::{DateTime, Datelike, Local, Utc, Weekday};
use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct Taskdate {
    day: Weekday,
    num_day: u32,
    month: u32,
    year: i32,
    time: String,
}

fn num_to_month(num_month: i16) -> &'static str {
    assert_ne!(num_month, 0);
    assert!(num_month < 12);
    match num_month {
        1 => "Jan",
        2 => "Feb",
        3 => "March",
        4 => "Apr",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "Aug",
        9 => "Sept",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => panic!("Cannot convert number greater than 12 since there are only 12 months!"),
    }
}

#[allow(dead_code)]
const UNIVERSAL_FORM: &'static str = "%a %d %b %y %H:%M:%S%.3f";

#[allow(dead_code)]
impl Taskdate {
    pub fn init_date<T: chrono::TimeZone + std::fmt::Debug>(chrono_obj: DateTime<T>) -> Taskdate {
        let now = chrono_obj;
        let day = now.weekday();
        let month = now.month();
        let year = now.year();
        let num_day = now.day();
        let cur_time = now.time().to_string();
        println!("tmz {:?}", now.timezone());
        return {
            Taskdate {
                day,
                month,
                year,
                num_day,
                time: cur_time,
            }
        };
    }
    pub fn new_local() -> Self {
        let new_lc = Local::now();
        return Taskdate::init_date(new_lc);
    }
    pub fn from_string(existed_date: String) -> Self {
        println!("{}", existed_date);
        let dt =
            DateTime::parse_from_rfc2822(&existed_date).expect("Wrong input format in datetime!");
        Taskdate::init_date(dt)
    }
    pub fn from_timestamp(tms: i64) -> Self {
        let given_time = chrono::NaiveDateTime::from_timestamp(tms, 0);
        let local_con = DateTime::<Utc>::from_utc(given_time, Utc).with_timezone(&chrono::Local);
        Taskdate::init_date(local_con)
    }
    pub fn to_string(&self) -> String {
        let mut new_st = String::new();
        let dnmy = format!(
            "{}, {} {} {}",
            self.day,
            self.num_day,
            num_to_month(self.month as i16),
            self.year
        );
        // always use green mean time zone ;
        let full_format = format!("{} {} GMT", dnmy, self.time);
        new_st.push_str(&full_format);
        return new_st;
    }
}

#[cfg(test)]
mod tts {
    use super::*;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    #[test]
    fn test_taskdate() {
        // 1983 Apr 13 12:09:14.274 +0000
        let date_string = String::from("Wed, 18 Feb 2015 23:16:09 GMT");
        let _new_date = Taskdate::new_local();
        let expo = Taskdate::from_string(date_string.clone());
        println!("{}", _new_date.to_string());
        assert_eq!(expo.to_string().clone(), date_string);
        //480941600 -> float 9 \\ truncate 3 -> 9 - 7;
    }
    #[test]
    fn record_timestamp() {
        let inst = SystemTime::now();
        let epoch_ts = inst
            .duration_since(UNIX_EPOCH)
            .expect("SYSTEM TIME B4 EPOCH! Cannot count!");
        let eps_mil = epoch_ts.as_secs() as i64;
        let recorded = Taskdate::from_timestamp(eps_mil);
        println!("recorded {:?}", recorded);
    }
}
