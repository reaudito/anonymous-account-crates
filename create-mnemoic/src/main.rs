use bip39::{Language, Mnemonic};
fn main() {
    let mut rng = bip39::rand::thread_rng();
    let m = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();
    println!("{}", m);
}
