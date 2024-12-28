//since copied much of this code I'm going to write verbose comments.

//import traits for hashing
use sha2::Digest;
use sha256::digest;

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

fn combine_msg_nonce(message: &str, nonce: u64) -> String {
    nonce.to_string() + message
}

fn main() {
    let message = "Mary was greeted by John.";

    let mut nonce: u64 =  0;
    
    println!("at this stage we have a string and a nonce\n string: {message}\n nonce:{nonce}");
    print!("now let's combine the nonce and the string");

    let nonced_message = combine_msg_nonce(message, nonce);

    println!("combined nonce and string {nonced_message}");

    // Compute the hash
    let result = hash(&nonced_message);
    println!("The hex byte_array of the hash output is {:?}", result);
    
    // Count the leading zeros in the hash
    let leading_0s = leading_bits(&result);
    println!("The number of leading 0s in the hash is {}", leading_0s);

    let readable_hash = digest(&nonced_message);
    
    println!("The actual hash of {nonced_message} is {readable_hash}" );


    println!("-----------------------------------------\n We'll clean this up later, but now let's get started on the loop.\n!-----------------------------------------");
    let min_leading0s: u32 = 27;
    loop {
        let nonced_message = combine_msg_nonce(message, nonce);
        let hash_result = hash(&nonced_message);
        let leading_0s = leading_bits(&hash_result);
        if leading_0s >= min_leading0s {
            break;
        }
        nonce += 1
    }
    println!("After running the loop, the nonce is {nonce} and the nonced_message is {nonced_message}");




}
    

    