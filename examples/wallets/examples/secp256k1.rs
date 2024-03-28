#[cfg(test)]
mod test_indexer {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use ethers::{abi::Address, providers::Middleware, utils::keccak256};
    use ethers_signers::{coins_bip39::English, LocalWallet, MnemonicBuilder};
    use near_sdk::store::key::Keccak256;
    use secp256k1::PublicKey;

    #[test]
    fn to_convert_publickey_to_address() {
        let add1 = [
            3, 139, 73, 66, 35, 67, 61, 249, 133, 171, 164, 3, 0, 15, 75, 13, 50, 165, 181, 232,
            230, 182, 114, 184, 31, 141, 25, 76, 242, 11, 217, 45, 15,
        ];
        let p_uncompreesed =
            PublicKey::from_slice(&add1).unwrap().serialize_uncompressed().to_vec();
        let keccak_encdd = keccak256(&p_uncompreesed[1..]);
        let address = hex::encode(&keccak_encdd[12..32]).to_string();
        println!("{}", format!("0x{}", address));
    }
}
