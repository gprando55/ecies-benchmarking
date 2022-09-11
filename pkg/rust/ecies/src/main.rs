#![allow(unused)]
use std::time::Instant;

use ecies::{decrypt, encrypt, utils::generate_keypair};

const SIZE_1KB: usize = (1 * 1024 * 1024)/1000;
const SIZE_10KB: usize = (1 * 1024 * 1024)/100;
const SIZE_100KB: usize = (1 * 1024 * 1024)/10;
const SIZE_1MB: usize = 1 * 1024 * 1024;
const SIZE_10MB: usize = 10 * 1024 * 1024;

const MSG_1KB: [u8;SIZE_1KB]= [1u8; SIZE_1KB];
const MSG_10KB: [u8;SIZE_10KB]= [1u8; SIZE_10KB];
const MSG_100KB: [u8;SIZE_100KB]= [1u8; SIZE_100KB];
const MSG_1MB: [u8;SIZE_1MB]= [1u8; SIZE_1MB];
const MSG_10MB: [u8;SIZE_10MB]= [1u8; SIZE_10MB];

fn main() {
    let (sk, pk) = generate_keypair();
    let (sk, pk) = (&sk.serialize(), &pk.serialize());

    let msg = &MSG_10MB;
    
    let before_encrypted = Instant::now(); 
    let msg_encrypted = &encrypt(pk, msg).unwrap();
    println!("encrypt 10mb {:.2?} milliseconds", before_encrypted.elapsed());
   
    let before_decrypt = Instant::now();

    let msg_decrypted = &decrypt(sk, msg_encrypted).unwrap();
    println!("decrypt 10mb {:.2?} milliseconds", before_decrypt.elapsed());
}
