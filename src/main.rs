use base64::{engine::general_purpose::STANDARD, Engine};
use ethereum_types::{H160, H256};
use secp256k1::{rand, Secp256k1, SecretKey};
use serde::Serialize;
use sha3::{Digest, Keccak256};

fn main() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let public_key = secret_key.public_key(&secp);
    let priv_key = STANDARD.encode(secret_key.secret_bytes());
    let pub_key = STANDARD.encode(public_key.serialize());
    let mut res = [0u8; 64];
    res.copy_from_slice(&public_key.serialize_uncompressed()[1..65]);
    let addr = format!(
        "{:?}",
        H160::from(H256::from_slice(Keccak256::digest(res).as_slice()))
    );

    let validator_key = PrivValidatorKey {
        address: String::from(&addr[2..]).to_uppercase(),
        pub_key: Key {
            ty: String::from("tendermint/PubKeySecp256k1"),
            value: pub_key,
        },
        priv_key: Key {
            ty: String::from("tendermint/PrivKeySecp256k1"),
            value: priv_key,
        },
    };
    let json_str = serde_json::to_string_pretty(&validator_key).unwrap();
    println!("{}", json_str);
}
#[derive(Serialize)]
struct Key {
    #[serde(rename = "type")]
    pub ty: String,
    pub value: String,
}
#[derive(Serialize)]
struct PrivValidatorKey {
    pub address: String,
    pub priv_key: Key,
    pub pub_key: Key,
}
