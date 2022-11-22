use forest_key_management::{
    json::KeyInfoJson,
    Key,
};
use fvm_shared::{
    crypto::signature::SignatureType,
    address::Address,
};

pub fn create_wallet(wallet_type: SignatureType) -> (Address, String, Key, KeyInfoJson) {
    let key = forest_key_management::generate_key(wallet_type).unwrap();
    let key_info = KeyInfoJson(key.clone().key_info);
    let encoded_key = serde_json::to_string(&key_info).unwrap();
    let encoded_key = hex::encode(encoded_key);
    (key.address, encoded_key, key, key_info)
}

