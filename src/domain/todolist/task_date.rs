use chrono::{DateTime, Datelike, Local, Timelike, Utc, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Taskdate {
    pub day: Weekday,
    pub num_day: u32,
    pub month: u32,
    pub year: i32,
    pub time: String,
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
type Time = String;
#[allow(dead_code)]
impl Taskdate {
    pub fn make_time(h: i64, m: i64, s: i64) -> Time {
        format!("{}:{}:{}", h, m, s)
    }
    pub fn new(day: Weekday, num_day: u32, month: u32, year: i32, time: &str) -> Self {
        return Taskdate {
            num_day,
            month,
            year,
            time: String::from(time),
            day,
        };
    }
    fn init_date<T: chrono::TimeZone + std::fmt::Debug>(chrono_obj: DateTime<T>) -> Taskdate {
        let now = chrono_obj;
        let day = now.weekday();
        let month = now.month();
        let year = now.year();
        let num_day = now.day();
        let cur_time = format!("{}:{}:{}", now.hour(), now.minute(), now.second());

        dbg!(cur_time.clone());
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
        let spliter = existed_date.split_whitespace().collect::<Vec<&str>>();
        let mut final_form = String::new();
        for (idx, b_var) in spliter.into_iter().enumerate() {
            if idx == 4 {
                let mut slots = Vec::with_capacity(3);
                b_var.split(':').for_each(|var_time| {
                    let rounded = var_time.parse::<f64>().unwrap().round();
                    slots.push(rounded as i64);
                });
                let form = format!("{}:{}:{}", slots[0], slots[1], slots[2]);
                dbg!(&form);
                final_form.push_str(&format!("{} ", form))
            } else {
                final_form.push_str(&format!("{} ", b_var))
            }
        }

        dbg!("{}", &final_form);
        //Fri, 21 Aug 2020 14:41:17.31291231923 GMT

        let dt = DateTime::parse_from_rfc2822(&final_form.trim())
            .expect("Wrong input format in datetime!");
        // DateTime::parse_from_str(&existed_date, UNIVERSAL_FORM).expect("Wrong input format in datetime!");
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
        let date_string = String::from("Fri, 21 Aug 2020 14:41:17.31291231923 GMT");
        let _new_date = Taskdate::new_local();
        let expo = Taskdate::from_string(date_string.clone());
        let back_to_str = expo.to_string();
        println!("{:#?}", (_new_date.to_string(), _new_date));
        println!("{:?}", expo);
        assert_eq!(expo.to_string().clone(), "Fri, 21 Aug 2020 14:41:17 GMT");
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
