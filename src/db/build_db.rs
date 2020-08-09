use mongodb::{options, Client};
#[allow(dead_code)]
pub type MongoResult<T> = Result<T, mongodb::error::Error>;
// pub type Makedb = dyn ;
#[allow(dead_code)]
pub async fn build_db(url: &'static str, cred: options::Credential) -> MongoResult<Client> {
    let oc = url;
    let mut options = match options::ClientOptions::parse(&oc).await {
        Ok(clientop) => clientop,
        Err(c_err) => {
            eprintln!("Opps database error : ");
            eprintln!("{:#?}", c_err);
            return Err(c_err);
        }
    };
    options.credential = Some(cred);
    options.app_name = Some(String::from("TODO_APP"));
    match Client::with_options(options) {
        Ok(client) => Ok(client),
        Err(cerr) => Err(cerr),
    }
}
