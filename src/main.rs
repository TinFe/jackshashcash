use sha256::digest;
fn main() {
    let text_data = "Mary was greeted by John.";
    let mut nonce = 0;

    println!("so far we have a sentence and a nonce.\n sentence: {text_data}\n the nonce: {nonce}");
    println!("the nonce can be mutated. before it was {nonce}.");
    nonce += 1;
    println!("but now it is {nonce}");

    println!("first we need to practice hashing the text_data");
    let test_hash = digest(text_data);
    println!("the hash of text_data is {test_hash}");
    println!("that was successful. now we need to concatenate text_data with nonce. let's try.");
    println!("before we concatenate we must shadow the int nonce with a string.");
    let nonce = nonce.to_string();
    let nonced_message = nonce + text_data;
    println!("the concatenated nonced message is {nonced_message}");
    println!("Alright that was successful to. now.\n Time to hash the nonced message");
    let nonced_message_hash = digest(nonced_message.clone());
    println!("the hash of {nonced_message} is {nonced_message_hash}");
}
