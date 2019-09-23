extern crate wagyu_ethereum as ethereum;
extern crate wagyu_zcash as zcash;
extern crate wagyu_model;
extern crate hex;

use zcash::network::Testnet;
use zcash::address::ZcashAddress;
use zcash::{ZcashTransaction, ZcashTransactionParameters, ZcashPrivateKey, SignatureHash};
use wagyu_model::transaction::Transaction;

use std::str::FromStr;
//use zcash_primitives::merkle_tree::CommitmentTreeWitness;
use pairing::bls12_381::{Bls12, Fr, FrRepr};
use std::fmt;

const SATOSHI: u64 = 100000000;

pub struct TransparentInput {
    pub private_key: &'static str,
    pub address: &'static str,
    pub transaction_id: &'static str,
    pub index: u32,
    pub redeem_script: Option<&'static str>,
    pub script_pub_key: Option<&'static str>,
    pub utxo_amount: Option<u64>,
    pub sequence: Option<[u8; 4]>,
    pub sig_hash_code: SignatureHash,
}

pub struct SaplingInput {
    pub extended_secret_key: &'static str,
    pub cmu: &'static str,
    pub epk: &'static str,
    pub enc_ciphertext: &'static str,
    pub anchor: Option<&'static str>,
    pub witness: Option<&'static str>,
}

impl fmt::Display for TransparentInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<= Transaprent Input =>\n");
        write!(f, "private_key => {}\n", &self.private_key);
        write!(f, "address => {}\n", &self.address);
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
        write!(f, "sig hash => {}\n", &self.sig_hash_code);
        write!(f, "<= Transaprent Input =>\n");
        Ok(())
    }
}

fn main() {

    // get the inputs
    let sapling_input = SaplingInput {
        extended_secret_key: "",
        cmu: "",
        epk: "",
        enc_ciphertext: "",
        anchor: None,
        witness: None,
    };

    let input = TransparentInput { // has 100 TAZ
        private_key: "cReSytwPmkxKJiFh9HU21kvMVQSXdziiciwbzHrQysLgc8KH57MW",
        address: "tmMF8BTMfVhjnBZarYhoNLi4UF4gbGkz7uq",
        transaction_id: "bdeeb2648cdf851e096f319e51743ba3dbccdd6eed73ac1e86e7f2dc16f49f79",
        index: 0,
        redeem_script: Some(""),
        script_pub_key: None,
        utxo_amount: Some(10000000000),
        sequence: None,
        sig_hash_code: SignatureHash::SIGHASH_ALL,
    };

    println!("{}", &input);

    // default params
    let version = "sapling";
    let lock_time = 0;
    let expiry_height = 499999999;
    let parameters = ZcashTransactionParameters::<Testnet>::new(version, lock_time, expiry_height).unwrap();
    let mut transaction = ZcashTransaction::<Testnet>::new(&parameters).unwrap();

    // let's add shielded spend
//    let mut cmu = [0u8; 32];
//    let mut epk = [0u8; 32];
//    cmu.copy_from_slice(&hex::decode(sapling_input.cmu).unwrap());
//    cmu.reverse();
//    epk.copy_from_slice(&hex::decode(sapling_input.epk).unwrap());
//    epk.reverse();
//
//    let witness_vec = hex::decode(&sapling_input.witness).unwrap();
//    let witness = CommitmentTreeWitness::<Node>::from_slice(&witness_vec[..]).unwrap();
//
//    //let mut f = FrRepr::default();
//    //f.read_le(&hex::decode(sapling_input.anchor.unwrap()).unwrap()[..]).unwrap();
//    let anchor = Fr::from_repr(sapling_input.anchor.unwrap()).unwrap();
//
//    transaction.add_sapling_spend(sapling_input.extended_secret_key, &cmu, &epk, sapling_input.enc_ciphertext, anchor, witness);

    // let's add transaction input
    let private_key = ZcashPrivateKey::<Testnet>::from_str(input.private_key).unwrap();
    let address = ZcashAddress::<Testnet>::from_str(input.address).unwrap();

    let transaction_id = hex::decode(input.transaction_id).unwrap();
    let redeem_script = input.redeem_script.map(|script| hex::decode(script).unwrap());

    let script_pub_key = input.script_pub_key.map(|script| hex::decode(script).unwrap());
    let sequence = input.sequence.map(|seq| seq.to_vec());

    transaction.parameters = transaction.parameters.add_transparent_input(
            address,
            transaction_id,
            input.index,
            input.utxo_amount,
            redeem_script,
            script_pub_key,
            sequence,
            input.sig_hash_code,
    ).unwrap();

    // let's add transparent output
    let output_address = ZcashAddress::<Testnet>::from_str("tmVyhzSSHdAkpozEkgAPzRy18KfwDDwrQcL").unwrap();
    let output_amount = 499960000; // in taz
    transaction.parameters = transaction.parameters.add_transparent_output(&output_address, output_amount).unwrap();

    // show inputs & outputs
    println!("tx params = {:?}", &transaction.parameters);

    // let's sign the transaction
    transaction.sign_raw_transaction(private_key.clone(), 0).unwrap();

    let signed_transaction = hex::encode(transaction.to_transaction_bytes().unwrap());

    // raw signed transaction
    println!("signed tx = {:?}", signed_transaction);
}
