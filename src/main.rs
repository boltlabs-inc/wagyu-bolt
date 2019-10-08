extern crate wagyu_ethereum as ethereum;
extern crate wagyu_zcash as zcash;
extern crate wagyu_model;
extern crate hex;

use zcash::network::Testnet;
use wagyu_bolt::transactions::{Input, TxConfig, Output, createZcashEscrowTx};
use std::str::FromStr;
//use zcash_primitives::merkle_tree::CommitmentTreeWitness;
use pairing::bls12_381::{Bls12, Fr, FrRepr};
use std::fmt;

fn main() {

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

    let signed_escrow_tx = createZcashEscrowTx::<Testnet>(&config, &input, &output);

    println!("signed escrow tx: {}", &signed_escrow_tx);

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

}
