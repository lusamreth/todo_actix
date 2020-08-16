#[derive(Debug)]
pub enum DBERROR {
    Operation(String),
    Bson(Option<String>),
    Mongodb(mongodb::error::Error),
}

#[allow(dead_code)]
impl DBERROR {
    pub fn get_string(&self) -> String {
        match self {
            DBERROR::Operation(value) => {
                let opt = format!("Operational error occured !");
                let detail = String::from(value);
                return format!("=> {} : \n {}", opt, detail);
            }
            DBERROR::Bson(bson) => match bson {
                Some(msg) => format!("Bson Error : {}", msg),
                None => format!("Cannot convert entities into desirable document!"),
            },
            DBERROR::Mongodb(value) => {
                eprintln!("kind of mdb error {}", value.kind);
                return String::from(format!("Database Error kind : {}", value.kind));
            }
        }
    }
    pub fn get_type(&self) -> String {
        String::from("INTERNAL_ERROR")
    }
}

#[test]
fn test_db_er() {
    let err_array = vec![
        DBERROR::Operation(String::from("lol")),
        DBERROR::Bson(Some("some bson parsing error".to_string())),
    ];
    // cannot test mongodb error cause non_exhaustive struct
    err_array.iter().for_each(|err| {
        let hm: Result<(), &DBERROR> = Err(err);
        println!("raw {:?}", hm);
        let tp = err.get_type();
        let msg = err.get_string();
        println!("type : {} \n msg : {}", tp, msg);
    })
}
