use crate::domain::resporitory_interface::resperror::DBERROR;
mod variables;
pub use variables::*;

#[derive(Debug, Clone)]
pub enum PortError {
    Internal(String),
    External(String),
}
/**
Output Example! (acknowledge that this is repo_err):
PortErrorExtender {
    main_type:"DbError"
    sub_type:Mongodb
    message:"Error parsing string from db"
}
*PortException = PortErrorExtender;
**/

//sub-type is dynamic since the coverage is not too broad;
#[derive(Debug, Clone)]
pub struct PortException {
    pub main_type: String,
    pub sub_type: String,
    pub message: String,
    pub interface_type: Option<PortError>,
}

impl BussinessError for PortError {
    fn domain_err(&mut self) -> PortException {
        let msg = self.transfer();
        dbg!(msg);
        // debug
        let mt = "BussinessError".to_owned().to_string();
        let sub_type = String::from("n/a");
        return self.assert_type(mt, sub_type);
    }
}

impl Default for PortException {
    fn default() -> Self {
        let default_exception = PortException {
            main_type: "UNKNOWN".to_string(),
            sub_type: "UNKNOWN-Subtype".to_string(),
            message: String::new(),
            interface_type: None,
        };
        return default_exception;
    }
}
impl PortError {
    // convert internal error to external
    pub fn emit_internal(&self) -> Self {
        if let PortError::Internal(throwable) = self {
            dbg!("Internal Error from Driver entity!");
            dbg!("Details : \n");
            dbg!("{}", throwable);
            return PortError::External("Internal Error!".to_string());
        } else {
            panic!("This error is external! go to transfer!")
        }
    }

    pub fn transfer(&self) -> String {
        if let PortError::External(ex_err) = self {
            return ex_err.to_owned();
        } else {
            panic!("This error is not transferable!")
        }
    }

    // kinda uselesss???
    pub fn bundle_emit(buffer: Vec<Self>) {
        buffer.iter().for_each(|each_err| {
            println!("Emiting from bundles!");
            if let PortError::Internal(_) = each_err {
                each_err.emit_internal();
            } else {
                each_err.transfer();
            }
        })
    }
}

impl OptError for PortError {
    fn operation_err(&mut self) -> PortException {
        let msg = self.transfer();
        let mt = Self::main_type.to_owned().to_string();
        let sub_type = String::from(OperationalSubtype::IO);
        return self.assert_type(mt, sub_type);
    }
    const main_type: &'static str = "Operational";
}
impl Exception for PortError {
    fn extend_input(&self) -> PortException {
        let mut default_exception = PortException::default();
        default_exception.message = self.transfer();
        return default_exception;
    }

    fn assert_type(&mut self, input_type: String, sub_type: String) -> PortException {
        let mut default = self.extend_input();
        default.main_type = input_type;
        default.sub_type = sub_type;
        match self {
            PortError::Internal(x) => {
                dbg!(x);
                default.interface_type = Some(PortError::Internal(String::new()))
            }
            PortError::External(y) => {
                dbg!(y);
                default.interface_type = Some(PortError::External(String::new()))
            }
        }
        default
    }
    fn convert<T>(err: T) -> PortException
    where
        T: Into<PortException>,
    {
        err.into()
    }
}

impl Into<PortException> for DBERROR {
    fn into(self) -> PortException {
        let db_type = self.get_type();
        let msg = self.get_string();
        let main_type = "DBERROR".to_string();
        PortException {
            main_type,
            sub_type: db_type,
            message: msg,
            interface_type: Some(PortError::Internal(String::new())),
        }
    }
}
use std::fmt::{Display, Formatter};
impl Display for PortException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "main-type : {}\nsub-type : {}\nmessage:{}\ninterface_type : {:?}\n",
            self.main_type, self.sub_type, self.message, self.interface_type
        )
    }
}
