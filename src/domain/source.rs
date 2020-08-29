use sha2::{self, Digest};
use base64::encode;


pub fn create_id(payload:String) -> String{
    let mut hasher = sha2::Sha256::new();
    hasher.update(payload.as_bytes());
    let mut result = hasher.finalize();
    let b64_str = encode(result.as_mut_slice());
    return b64_str;
}

#[test]
fn test(){
    let input = "name";
    let output = create_id(input.to_string());
    dbg!(output);
}