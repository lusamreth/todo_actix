// test function builder.factory!
// bad performance

// normal synchronous function
fn fx_factory() -> Box<dyn Fn() -> String> {
    let produced_fx = || String::from("apple");
    return Box::new(produced_fx);
}
use mongodb::bson::to_bson;
use std::future::Future;
use std::pin::Pin;
type ASI = Box<dyn Fn() -> Pin<Box<dyn Future<Output = String>>>>;

pub async fn abitary_awaiter() -> String {
    String::from("from async fx")
}

async fn async_factory() -> ASI {
    return Box::new(|| Box::pin(abitary_awaiter()));
}
#[derive(Serialize, Deserialize, Clone)]
struct temp {
    student: i32,
}

fn input_serialize() {
    let string_obj = String::from(
        r#"{
        name:"apple",
        sauce:"doroti"
    }"#,
    );
    let bson_from_string = to_bson(&string_obj).unwrap();
    println!("{:#?}", bson_from_string)
}
use mongodb::Client;
use mongodb::{bson, bson::doc, options};
async fn init_mong() {
    //USERNAME
    //PASSWORD
    let username = dotenv::var("USERNAME").unwrap();
    let password = dotenv::var("PASSWORD").unwrap();
    let source = dotenv::var("DATABASE_NAME").unwrap();

    let mut cop = options::ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let cred = options::Credential::builder()
        .username("lusamreth".to_string())
        .password(String::from("Somreth012618653"))
        .source("tododb".to_string());
    cop.credential = Some(cred.build());
    let client = Client::with_options(cop).unwrap();
    let testdb = client.database("tododb");

    let shoot = testdb.collection("tasks");
    let ic = shoot
        .insert_one(
            doc! {
                "bbd":1,
                "name":"jeyaaaa"
            },
            None,
        )
        .await
        .unwrap();

    println!("ic {:?}", ic);
    #[allow(unused_variables)]
    let newtmp = temp { student: 8028123 };
    let mut test_vec = Vec::new();
    for _ in 0..10 {
        let update_qu = bson::to_bson(&newtmp).unwrap();
        test_vec.push(update_qu.clone())
    }
    let shooa = shoot
        .update_one(
            doc! {"bbd":1},
            doc! {"$set":{
                "array":Bson::Array(test_vec)
            }},
            None,
        )
        .await;

    // let aggro = shoot.aggregate(vec![doc! {}],None).await;
    println!("laz3er {:?}", shooa.unwrap())
}
use serde::{Deserialize, Serialize};
#[derive(Serialize)]
struct TestSerde {
    #[serde(rename = "puta")]
    partial1: String,
    partial2: i32,
}
use mongodb::bson::Bson;
#[test]
fn test_serde() {
    let tex = TestSerde {
        partial1: "putaaa".to_string(),
        partial2: 223323,
    };
    let ob = r#"{name:822}"#;
    let jsonval = serde_json::to_string(&tex).unwrap();
    let bs = to_bson(&tex).unwrap();
    println!("{}", jsonval);
    println!("{}", bs);
    // Bson::Document();
    let dao = doc! {"name":12};
    let mut empty_suc = bson::document::Document::new();
    let emp_json = json::object! {stuc:392};
    empty_suc.insert("apple", doc! {"a":2});
}
#[cfg(test)]
mod factories {
    use super::*;
    use actix_rt;
    #[test]
    fn test() {
        let str_fx = fx_factory();
        println!("{}", str_fx());
        input_serialize();
    }
    #[actix_rt::test]
    async fn run_async() {
        let new_fx = async_factory().await;
        let from_res = new_fx().await;
        init_mong().await;
        println!("{}", from_res)
    }
}
