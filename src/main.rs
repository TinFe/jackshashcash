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

fn main() {
    let some_string = "Mary was greeted by John.";
    println!("Let's see if the hash function works.");
    
    // Compute the hash
    let result = hash(&some_string);
    println!("The hex byte_array of the hash output is {:?}", result);
    
    // Count the leading zeros in the hash
    let leading_0s = leading_bits(&result);
    println!("The number of leading 0s in the hash is {}", leading_0s);

    let readable_hash = digest(some_string);
    
    println!("The actual hash is {readable_hash}" );
}
    

    