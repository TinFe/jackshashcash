fn main() {
    let text_data = "Mary was greeted by John.";
    let mut nonce = 0;

    println!("so far we have a sentence and a nonce.\n sentence: {text_data}\n the nonce: {nonce}");
    println!("the nonce can be mutated. before it was {nonce}.");
    nonce += 1;
    println!("but now it is {nonce}");
}
