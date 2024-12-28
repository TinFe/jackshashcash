//since copied much of this code I'm going to write verbose comments.

//import traits for hashing
use sha2::Digest;
use sha256::digest;
use std::io::{self, Read};

//hash function which takes a string as a parameter, and outputs a hash of the string as an array of 32 unsigned bytes. 
fn hash(input: &str) -> [u8; 32] {
    // create a hasher instance
    let mut hasher = sha2::Sha256::new();
    //hasher instance must be passed bytes
    hasher.update(input.as_bytes());
    
    hasher.finalize().into()
}

// Count leading zero bits in the hash
fn leading_bits(arr: &[u8; 32]) -> u32 {
    let mut count = 0;
    for x in arr {
        let bits = x.leading_zeros(); // Count leading zeros in the byte
        count += bits;
        if bits != 8 { // If a byte is not 00000000, then we should stop. no need to check the next byte.
            break;
        }
    }
    count
}

fn combine_msg_nonce(message: &str, nonce: u32) -> String {
    nonce.to_string() + message
}

fn main() {
    println!("------------Jack's Hashcash------------");
    println!("Type or Paste your input. Ensure your cursor is on a new line, and press Ctrl+D (Linux/macOS) or Ctrl+Z (Windows). Ensure your when you're done:");
    let mut message = String::new();
    let mut nonce = 0;
    io::stdin()
        .read_to_string(&mut message)
        .expect("Failed to read input");

    println!("\n -----------------------------------------------------------------------------\nType desired workload (1-50) and press Enter\n -----------------------------------------------------------------------------\n");
    let mut min_leading0s = String::new();
        io::stdin()
            .read_line(&mut min_leading0s)
            .expect("Failed to read input");
    
    let min_leading0s: u32 = min_leading0s
        .trim()
        .parse()
        .expect("enter valid number");

    
    println!("\n-----------------------------------------------------------------------------\n Please wait. Finding Nonce.\n-----------------------------------------------------------------------------\n");
    loop {
        let nonced_message = combine_msg_nonce(&message, nonce);
        let hash_result = hash(&nonced_message);
        let leading_0s = leading_bits(&hash_result);
        if leading_0s >= min_leading0s {
            break;
        }
        nonce += 1
    }
    let nonced_message = combine_msg_nonce(&message, nonce);
    println!("\nNonce Found!\n-----------------------------------------------------------------------------\nNonce:\n{nonce}\n");

    println!("-------------------- Begin Proof-of-Work Message --------------------\n{nonced_message}\n-------------------- End Proof-of-Work Message --------------------\n");
    let hash_output = digest(&nonced_message);
    println!("\nHash:\n{:?}", hash_output);
}
    

    