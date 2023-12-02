#[cfg(feature = "hex")]
pub fn pp() {
    println!(
        "{}",
        std::str::from_utf8(&hex::decode("74776f5f6665617475726564").unwrap()).unwrap()
    );
}

#[cfg(not(feature = "hex"))]
pub fn pp() {
    println!("two",);
}
