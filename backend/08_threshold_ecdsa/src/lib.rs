use ic_cdk::{query, update};
use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use ripemd::{Ripemd160, Digest};
use base_x;

type CanisterId = Principal;

// https://github.com/dfinity/examples/blob/master/rust/threshold-ecdsa/src/ecdsa_example_rust/src/lib.rs
// Import Begin
#[derive(CandidType, Serialize, Debug)]
struct ECDSAPublicKey {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug, Clone)]
struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Serialize, Debug, Clone)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}
// Import End

/// Returns XRP classic address from ECDSA public key (33 bytes)
/// 
/// [Address Encoding](https://xrpl.org/addresses.html#address-encoding)
/// 
#[query]
fn get_xrp_address_from_ecdsa_public_key(public_key: Vec<u8>) -> String {
    // STEP-1: SHA-256
    let mut hasher = sha2::Sha256::new();
    hasher.update(public_key);
    let hashed_sha256:[u8; 32] = hasher.finalize().into();

    // STEP-2: RIPEMD160
    let mut hasher = Ripemd160::new();
    hasher.update(&hashed_sha256);
    let account_id = hasher.finalize();

    // STEP-3: Payload (Starting with 0x00)
    let mut payload = account_id.to_vec();
    payload.insert(0, 0x0);

    // STEP-4: Append checksum (sha256 x 2)
    let mut hasher = sha2::Sha256::new();
    hasher.update(&payload);
    let hash:[u8; 32] = hasher.finalize().into();
    let mut hasher = sha2::Sha256::new();
    hasher.update(hash);
    let checksum:[u8; 32] = hasher.finalize().into();
    payload.append(&mut checksum[0..4].to_vec());

    // STEP-5: Base58 with Ripple Alphabet
    const ALPHABET: &str = "rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";
    base_x::encode(ALPHABET, &payload)
}

#[update]
async fn get_public_key() -> Result<Vec<u8>, String> {
  let request = ECDSAPublicKey {
    canister_id: None,
    derivation_path: vec![],
    key_id: EcdsaKeyId {
      curve: EcdsaCurve::Secp256k1,
      name: "dfx_test_key".to_string(),
    }
  };

  let (res,): (ECDSAPublicKeyReply,) =
    ic_cdk::call(Principal::management_canister(), "ecdsa_public_key", (request,))
      .await
      .map_err(|e| format!("ecdsa_public_key failed {}", e.1))?;

  Ok(res.public_key)
}


#[update]
async fn get_xrp_address() -> Result<String, String> {
  let public_key = get_public_key().await.unwrap();
  Ok(get_xrp_address_from_ecdsa_public_key(public_key))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let public_key: Vec<u8> = vec![
      0x03, 0x40, 0xF6, 0x20, 0x04, 0xD3, 0x69, 0x4A,
      0x11, 0x98, 0x25, 0xAE, 0x20, 0xCE, 0x44, 0x52,
      0x2E, 0xDC, 0x49, 0xDE, 0x0C, 0xF1, 0x10, 0x77,
      0x82, 0x0B, 0x55, 0x8A, 0xA9, 0x0E, 0x4E, 0x5F,
      0xFF
    ];
    let xrp_address = get_xrp_address_from_ecdsa_public_key(public_key);
    assert_eq!(xrp_address, "rpALVWHxZPwGvimmqEcY8brui4xaQRgKXY");
  }
}
