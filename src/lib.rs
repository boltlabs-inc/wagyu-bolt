extern crate wagyu_bitcoin as bitcoin;
extern crate wagyu_ethereum as ethereum;
extern crate wagyu_zcash as zcash;
extern crate wagyu_model;
extern crate hex;

pub mod transactions {

    use bitcoin::network::BitcoinNetwork;
    use bitcoin::{BitcoinFormat, BitcoinTransaction, BitcoinTransactionInput, BitcoinTransactionOutput, BitcoinTransactionParameters, BitcoinPrivateKey, BitcoinAmount};
    use bitcoin::address::BitcoinAddress;
    use zcash::network::ZcashNetwork;
    use zcash::address::ZcashAddress;
    use zcash::{ZcashFormat, ZcashTransaction, ZcashTransactionParameters, ZcashPrivateKey, ZcashAmount};
    use wagyu_model::transaction::Transaction;
    pub use bitcoin::SignatureHash as BitcoinSigHash;
    pub use zcash::SignatureHash as ZcashSigHash;
    use wagyu_model::crypto::hash160;
    use crate::wagyu_model::PrivateKey;

    //use zcash_primitives::merkle_tree::CommitmentTreeWitness;
    use pairing::bls12_381::{Bls12, Fr, FrRepr};

    use std::fmt;
    use std::str::FromStr;
    const SATOSHI: i64 = 100000000;

    pub struct Input {
        pub private_key: &'static str,
        pub address_format: &'static str,
        pub transaction_id: &'static str,
        pub index: u32,
        pub redeem_script: Option<&'static str>,
        pub script_pub_key: Option<&'static str>,
        pub utxo_amount: Option<i64>,
        pub sequence: Option<[u8; 4]>
    }

    pub struct Output {
        pub address: &'static str,
        pub address_format: &'static str,
        pub amount: i64
    }

    pub struct SaplingInput {
        pub extended_secret_key: &'static str,
        pub cmu: &'static str,
        pub epk: &'static str,
        pub enc_ciphertext: &'static str,
        pub anchor: Option<&'static str>,
        pub witness: Option<&'static str>,
    }

    impl fmt::Display for Input {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "<= UTXO Input =>\n");
            write!(f, "private_key => {}\n", &self.private_key);
            write!(f, "address_format => {}\n", &self.address_format);
            write!(f, "tx id => {}\n", &self.transaction_id);
            write!(f, "index => {}\n", &self.index);
            if self.redeem_script.is_some() {
                write!(f, "redeem script => '{}'\n", &self.redeem_script.unwrap());
            }
            if self.script_pub_key.is_some() {
                write!(f, "script pubkey => '{}'\n", &self.script_pub_key.unwrap());
            }
            if self.utxo_amount.is_some() {
                let amount = self.utxo_amount.unwrap() / SATOSHI;
                write!(f, "utxo amount => {}\n", &amount);
            }
            if self.sequence.is_some() {
                write!(f, "sequence => {:?}\n", &self.sequence.unwrap());
            }
            write!(f, "<= UTXO Input =>\n");
            Ok(())
        }
    }

    pub struct TxConfig {
        pub version: u32,
        pub lock_time: u32,
        pub expiry_height: u32
        //coin: String, // replace with enum for chain
    }

    pub fn createBitcoinEscrowTx<N: BitcoinNetwork>(config: &TxConfig, input: &Input, output: &Output) -> String {
        // input
        let private_key = BitcoinPrivateKey::<N>::from_str(input.private_key).unwrap();

        let address_format = match input.address_format {
            "P2PKH" => BitcoinFormat::P2PKH,
            _ => panic!("did not specify supported address format")
        };
        let address = private_key.to_address(&address_format).unwrap();


        let transaction_id = hex::decode(input.transaction_id).unwrap();
        //let redeem_script = input.redeem_script.map(|script| hex::decode(script).unwrap());
        // let address_format = BitcoinFormat::P2PKH;

        let redeem_script = match (input.redeem_script, address_format.clone()) {
            (Some(script), _) => Some(hex::decode(script).unwrap()),
            (None, BitcoinFormat::P2SH_P2WPKH) => {
                let mut redeem_script = vec![0x00, 0x14];
                redeem_script.extend(&hash160(
                    &private_key.to_public_key().to_secp256k1_public_key().serialize(),
                ));
                Some(redeem_script)
            }
            (None, _) => None,
        };

        let script_pub_key = input.script_pub_key.map(|script| hex::decode(script).unwrap());
        let sequence = input.sequence.map(|seq| seq.to_vec());

        let transaction_input = BitcoinTransactionInput::<N>::new(
            &address,
            transaction_id,
            input.index,
            BitcoinAmount(input.utxo_amount.unwrap()),
            redeem_script,
            script_pub_key,
            sequence,
            BitcoinSigHash::SIGHASH_ALL
        )
        .unwrap();

        let mut input_vec = vec![];
        input_vec.push(transaction_input);

        let mut output_vec = vec![];

        let address = BitcoinAddress::<N>::from_str(output.address).unwrap();
        let tx_output = BitcoinTransactionOutput::new(&address, BitcoinAmount(output.amount)).unwrap();
        output_vec.push(tx_output);

        let parameters = BitcoinTransactionParameters::<N> {
            version: config.version,
            inputs: input_vec,
            outputs: output_vec,
            lock_time: config.lock_time,
            segwit_flag: false,
        };
        let mut transaction = BitcoinTransaction::<N>::new(&parameters).unwrap();

        transaction = transaction
            .sign(&BitcoinPrivateKey::from_str(input.private_key).unwrap() )
            .unwrap();

        let signed_transaction = hex::encode(transaction.to_transaction_bytes().unwrap());

        return signed_transaction;
    }

    // single funded transactions
    pub fn createZcashEscrowTx<N: ZcashNetwork>(config: &TxConfig, input: &Input, output: &Output) -> String {
        let version = "sapling";
        let lock_time = config.lock_time;
        let expiry_height = config.expiry_height;

        let parameters = ZcashTransactionParameters::<N>::new(version, lock_time, expiry_height).unwrap();
        let mut transaction = ZcashTransaction::<N>::new(&parameters).unwrap();

        // specify input
        let private_key = ZcashPrivateKey::<N>::from_str(input.private_key).unwrap();
        let address_format = match input.address_format {
            "P2PKH" => ZcashFormat::P2PKH,
            _ => panic!("did not specify supported address format")
        };
        let address = private_key.to_address(&address_format).unwrap();
        //let address = ZcashAddress::<N>::from_str(input.address).unwrap();

        let transaction_id = hex::decode(input.transaction_id).unwrap();
        let redeem_script = input.redeem_script.map(|script| hex::decode(script).unwrap());

        let script_pub_key = input.script_pub_key.map(|script| hex::decode(script).unwrap());
        let sequence = input.sequence.map(|seq| seq.to_vec());

        // add transparent input
        transaction.parameters = transaction.parameters.add_transparent_input(
                address,
                transaction_id,
                input.index,
                Some(ZcashAmount(input.utxo_amount.unwrap())),
                redeem_script,
                script_pub_key,
                sequence,
                ZcashSigHash::SIGHASH_ALL
        ).unwrap();

        let output_address = ZcashAddress::<N>::from_str(output.address).unwrap();
        let output_amount =  output.amount; // in taz
        transaction.parameters = transaction.parameters.add_transparent_output(&output_address,
                                                                               ZcashAmount(output_amount)).unwrap();

        // let's sign the transaction
        //transaction = transaction.sign_raw_transaction(private_key.clone(), 0).unwrap();

        transaction = transaction.sign(&private_key).unwrap();
        let signed_transaction = hex::encode(transaction.to_transaction_bytes().unwrap());

        return signed_transaction;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use transactions::{Input, Output, TxConfig};
    use std::intrinsics::transmute;
    use std::str::FromStr;
    use zcash::Testnet as ZcashTestnet;
    use bitcoin::Testnet as BitcoinTestnet;

    #[test]
    fn test_zcash_escrow_tx_transparent() {

        let input = Input { // has 100 TAZ
            private_key: "cReSytwPmkxKJiFh9HU21kvMVQSXdziiciwbzHrQysLgc8KH57MW",
            address_format: "P2PKH", // "tmMF8BTMfVhjnBZarYhoNLi4UF4gbGkz7uq",
            transaction_id: "bdeeb2648cdf851e096f319e51743ba3dbccdd6eed73ac1e86e7f2dc16f49f79",
            index: 0,
            redeem_script: Some(""),
            script_pub_key: None,
            utxo_amount: Some(10000000000),
            sequence: None
        };

        let config = TxConfig {
            version: 0,
            lock_time: 0,
            expiry_height: 499999999
        };

        let fee = 100000; // 0.001
        let output = Output { address: "tmVyhzSSHdAkpozEkgAPzRy18KfwDDwrQcL", address_format: "P2PKH",
                                amount: 10000000000 - fee };

        let signed_escrow_tx = transactions::createZcashEscrowTx::<ZcashTestnet>(&config, &input, &output);

        println!("signed escrow tx: {}", &signed_escrow_tx);
    }

    #[test]
    #[ignore]
    fn test_bitcoin_escrow_tx() {

        let input = Input {
            private_key: "L1uyy5qTuGrVXrmrsvHWHgVzW9kKdrp27wBC7Vs6nZDTF2BRUVwy",
            address_format: "P2PKH",
            transaction_id: "61d520ccb74288c96bc1a2b20ea1c0d5a704776dd0164a396efec3ea7040349d",
            index: 0,
            redeem_script: Some(""),
            script_pub_key: None,
            utxo_amount: Some(10000000),
            sequence: None // 4294967295
        };

        let config = TxConfig {
            version: 1,
            lock_time: 0,
            expiry_height: 499999999
        };

        let fee = 100; // 0.001
        let output = Output { address: "1Fyxts6r24DpEieygQiNnWxUdb18ANa5p7", address_format: "P2PKH", amount: 199996600 - fee };
        let signed_escrow_tx = transactions::createBitcoinEscrowTx::<BitcoinTestnet>(&config, &input, &output);

        println!("signed escrow tx: {}", &signed_escrow_tx);

    }
}
