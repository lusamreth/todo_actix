use crate::http::error::*;

fn parse_time(time: &str) -> Vec<Result<i64, Fielderror>> {
    let mmhhss_format =
        regex::Regex::new(r"^(?:(?:([01]?\d|2[0-3]):)?([0-5]?\d):)?([0-5]?\d)$").unwrap();
    let captured = mmhhss_format
        .captures(time)
        .unwrap()
        .iter()
        .skip(1)
        .enumerate()
        .map(|(idx, matched)| {
            match matched {
                Some(matched) => {
                    let str = matched.as_str();
                    let parse_index = |s: &str| {
                        let parsed = match s.parse::<i64>() {
                            Ok(parsed) => Ok(parsed),
                            Err(_) => {
                                // let except = PortError::External(String::from("Failed to parsed the time!")).extend_input();
                                let mut name = String::new();
                                match idx {
                                    0 => name.push_str("hour"),   // hh,
                                    1 => name.push_str("minute"), // hh,
                                    2 => name.push_str("second"), // hh
                                    _ => panic!("Failed"),
                                }
                                let except = Fielderror {
                                    field: name.clone(),
                                    detials: format!("The format of the field {} is invalid", name),
                                };
                                Err(except)
                            }
                        };
                        return parsed;
                    };

                    return parse_index(str);
                }
                None => panic!("bad time!"),
            }
        })
        .collect::<Vec<Result<i64, Fielderror>>>();
    return captured;
}

type UsecaseTime = (i64, i64, i64);

pub fn to_usecase_time(time: String) -> Result<UsecaseTime, Vec<Fielderror>> {
    let vec_time = parse_time(time.as_str());
    let mut error_acc = Vec::new();
    let mut converted_time = (0, 0, 0);

    let int_vec = vec_time
        .into_iter()
        .enumerate()
        .map(|(_idx, t_res)| {
            if let Err(parsing_err) = t_res {
                error_acc.push(parsing_err);
                return 0;
            } else {
                return t_res.unwrap();
            };
        })
        .collect::<Vec<i64>>();

    converted_time.0 = *int_vec.get(0).unwrap();
    converted_time.1 = *int_vec.get(1).unwrap();
    converted_time.2 = *int_vec.get(2).unwrap();

    if error_acc.len() > 0 {
        return Err(error_acc);
    } else {
        return Ok(converted_time);
    }
}
#[test]
fn test_time_parser() {
    let time = "12:00:51";
    let mmhhss_format =
        regex::Regex::new(r"^(?:(?:([01]?\d|2[0-3]):)?([0-5]?\d):)?([0-5]?\d)$").unwrap();
    let mut input_time: (i64, i64, i64) = (0, 0, 0);
    let captured = mmhhss_format
        .captures(time)
        .unwrap()
        .iter()
        .skip(1)
        .map(|e| {
            match e {
                Some(matched) => {
                    let str = matched.as_str();
                    let parse_index = |s: &str| {
                        let parsed = s.parse::<i64>().unwrap();
                        return parsed;
                    };

                    return parse_index(str);
                }
                None => panic!("njasns"),
            }
            // e.unwrap().as_str()
        })
        .collect::<Vec<i64>>();
    input_time.0 = *captured.get(0).unwrap();
    input_time.1 = *captured.get(1).unwrap();
    input_time.2 = *captured.get(2).unwrap();

    println!("{:#?}", input_time);
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputJson<T: Serialize, R: Serialize> {
    pub data_type: String,
    pub id: Option<String>,
    pub attributes: T,
    pub relationship: Option<R>,
}

type Uri = String;
type PartialUri = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chainlink {
    by_self: PartialUri,
    to_other: Uri,
}

pub trait DefaultOutput<R: Serialize> {
    fn json_output(self, id: Option<String>) -> OutputJson<Self, R>
    where
        Self: std::marker::Sized,
        Self: Serialize,
    {
        OutputJson {
            data_type: String::from("Task"),
            id,
            attributes: self,
            relationship: None,
        }
    }
}

impl Chainlink {
    pub fn new() -> Self {
        Chainlink {
            by_self: String::new(),
            to_other: String::new(),
        }
    }
    pub fn self_link(mut self, link: &str) -> Self {
        self.by_self = link.to_string();
        self
    }
    pub fn other_link(mut self, link: &str) -> Self {
        self.to_other = link.to_string();
        self
    }
}
