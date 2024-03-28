use ethers::{
    abi::Address,
    utils::{hex, keccak256},
};
use secp256k1::PublicKey;
use std::{error::Error, str::FromStr};

/// convert secp256k1's public key with slice  to eth address
pub fn secp256k1_slice_to_address(public_key_vec: &[u8; 33]) -> Result<Address, Box<dyn Error>> {
    let p_uncompreesed = PublicKey::from_slice(public_key_vec)?.serialize_uncompressed().to_vec();
    let keccak_encdd = keccak256(&p_uncompreesed[1..]);
    let address = hex::encode(&keccak_encdd[12..32]).to_string();
    let address = Address::from_str(&address)?;
    Ok(address)
}

#[cfg(test)]
mod test_indexer {
    use super::*;

    #[test]
    fn to_convert_publickey_to_address() {
        let add1 = [
            3, 139, 73, 66, 35, 67, 61, 249, 133, 171, 164, 3, 0, 15, 75, 13, 50, 165, 181, 232,
            230, 182, 114, 184, 31, 141, 25, 76, 242, 11, 217, 45, 15,
        ];
        let address = secp256k1_slice_to_address(&add1).unwrap();
        println!("{}", format!("{:?}", address));
    }
}
