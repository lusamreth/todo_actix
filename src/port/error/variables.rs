use super::*;
pub enum OperationalSubtype {
    NOTFOUND,
    IO,
    INVALIDINPUT,
}

pub enum BussinessErrorSubtype {
    INVALIDINPUT,
    CONFLICTS,
}

pub trait Exception {
    fn extend_input(&self) -> PortException;
    fn convert<T>(err: T) -> PortException
    where
        T: Into<PortException>;
    fn assert_type(&mut self, input_type: String, sub_type: String) -> PortException;
}

pub trait BussinessError {
    fn domain_err(&mut self) -> PortException;
}
pub trait OptError {
    const main_type: &'static str = "Operational";
    fn operation_err(&mut self) -> PortException;
}
// use std::convert::From

impl From<OperationalSubtype> for String {
    fn from(hs: OperationalSubtype) -> Self {
        match hs {
            OperationalSubtype::NOTFOUND => String::from("not-found"),
            OperationalSubtype::IO => String::from("io-error"),
            OperationalSubtype::INVALIDINPUT => String::from("invalid-input"),
        }
    }
}

impl Into<String> for BussinessErrorSubtype {
    fn into(self) -> String {
        todo!()
    }
}
