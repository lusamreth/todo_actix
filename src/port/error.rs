use crate::domain::resporitory_interface::resperror::DBERROR;
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
pub struct PortException {
    main_type: String,
    pub sub_type: String,
    message: String,
    interface_type: Option<PortError>,
}

pub trait Exception {
    fn extend_input(&self) -> PortException;
    fn convert<T>(err: T) -> PortException
    where
        T: Into<PortException>;
    fn assert_type(&mut self, input_type: String, sub_type: String) -> PortException;
}


pub trait BussinessError{
    fn domain_err(&mut self) -> PortException;
}

impl BussinessError for PortError {
    fn domain_err(&mut self) -> PortException{
        let msg = self.transfer();
        dbg!(msg);
        // debug
        let mt = "BussinessError".to_owned().to_string();
        let sub_type = String::from("IO-Error");
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
pub trait OptError {
    const main_type: &'static str = "Operational";
    fn operation_err(&mut self) -> PortException;
}

impl OptError for PortError {
    fn operation_err(&mut self) -> PortException {
        let msg = self.transfer();
        let mt = Self::main_type.to_owned().to_string();
        let sub_type = String::from("IO-Error");
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
                default.interface_type = Some(PortError::Internal(String::new()))
            }
            PortError::External(y) => {
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

//Error Documentation!
// Error handling diagram of its transformation
/*
                        [propergate]                        [Advanced-to-Exception]
(*)-Driver-Error(database) --------------> PortError  -----GTW-----> Usecase  -----------> |Conversion==Interface|
(Driven adapter)                 (adapter implementation)         (App-layer)           [Http-layer-most_outer-ring]

*Coercsion from porterror to external adapter exception;Throwable errors should be propergate back to consumer in a
Controllable and comprehensible manner;

Error-flow-from
            [Inject]
(?)-RestApi ------> Port ==> Conversion-interface<T:Generic> ----> Foreign-exceptions(json) <---- |ACTIX_WEB_INTERFACE|

Diagram components:
+ ------> : advance to as the nature of application progress
+ GTW : gateway injection into usecase via port(interface)
+ * : mean general flow
+ ? : mean application's component flow
+ |[interface]| : mean adapted to that [interface]

*Usecase and gateway result may varies but the change of gateway should not distrupt the usecase services!

What will Usecase Result conversion will look like if the outer adapter is http?
Attempt: usecase service will still inherit the service from gateway thereforce same output but with additional
layer of successfull message; (But that's a presenter job?)
*/
