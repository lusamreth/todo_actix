use dotenv_codegen::dotenv;
use mongodb::Collection;

mod build_db;
mod credential;

pub async fn makedb() -> mongodb::Database {
    let url = dotenv!("DB_HOST");
    dbg!("this is host {}", url);
    let admin = credential::Admin::new(
        dotenv!("USERNAME").to_string(),
        dotenv!("PASSWORD").to_string(),
        dotenv!("DATABASE_NAME").to_string(),
    );
    let new_db = build_db::build_db(&url, admin.init_cred()).await;
    match new_db {
        Ok(client) => return client.database(dotenv!("DATABASE_NAME")),
        Err(cr) => panic!("Client error : {}", cr),
    }
}

#[derive(Clone)]
pub struct Db {
    pub collection: Collection,
}
impl Db {
    pub async fn fetch_collection(name: String) -> Self {
        return Db {
            collection: makedb().await.collection(&name),
        };
    }
}
