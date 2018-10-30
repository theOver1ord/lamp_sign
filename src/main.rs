extern crate rand;
extern crate sha3;

use sha3::{Digest, Sha3_256};

use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::fmt::Binary;
use std::str;

fn main() {

    let mut text = File::open("text.txt").expect("file not found");

    let mut contents_text = String::new();
    text.read_to_string(&mut contents_text).expect("something went wrong reading the file");
    //read text

    fn generate_rand_vec() -> Vec<u8> {
        let mut vector: Vec<u8> = Vec::new();
        for i in 0..256 {
            let mut num: u8 = random();
            vector.push(num);
        }
        return vector;
    }
    //generate rand massive of 256 elements
    
//   fn generate_hash_vect(vect: Vec<u8>) -> Vec<String>{
//       let mut hash_vect: Vec<String> = Vec::new();
//       let mut hasher = Sha3_256::new();
//       for i in 0..vect.len() {
           
//           hasher.input(vect[i]);
  //         hash_vect.push_str(hasher.result_str());
//       }
//       return hash_vect;
//   }
//    fn generate_hash_vect (vec_inp: Vec<u8>) -> Vec<????> бля какой тип, как узнать??

    struct key_struct {
        priv_key_0_part: Vec<u8>,
        priv_key_1_part: Vec<u8>,
        //pub_key_0_part: generate_hash_vect(priv_key_0_part),?,
        //pub_key_1_part: generate_hash_vect(priv_key_1_part),?,
    }

    let a = generate_new_key();

    // ниже буду пробовать создать массив хэшей ключей, вместо того, что бы объявлять массив по маске Vec<T>, где Т - тип, буду объявлять массив через 1-ый элемент

    let h_0_slice = a.priv_key_0_part[0].to_string();
    let mut hasher = Sha3_256::new();
    hasher.input(&h_0_slice);
    let mut h_0_hash = hasher.result();
    let mut hash_v = vec! [h_0_hash];
//    for i in 1..256 {
//        let mut h_i_slice = a.priv_key_0_part[i].to_string();
//        let mut h_i_hash = hasher.result();
//        hash_v.push(h_i_hash);
//    }
    println!("{:x}", hash_v[0]);
    
//    for i in 1..256 {
//        let mut h_var = sha3::Sha3_256::digest(&a.priv_key_0_part[i]);
//        hash_v.push(h_var);
//    }

    fn generate_new_key () -> key_struct {
        key_struct {
            priv_key_0_part: generate_rand_vec(),
            priv_key_1_part: generate_rand_vec(),
        }
    }
    // generate new key

//    let a = generate_new_key();
//    let b = generate_hash_vect(a.priv_key_0_part);

    for i in 0..256 {
        println!("I: {} ||, II: {}", a.priv_key_0_part[i], a.priv_key_1_part[i]);
    }
//private key on your desk    
}
 