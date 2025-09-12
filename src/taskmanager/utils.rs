pub fn read_number() -> Option<u8> {
    let mut number = String::new();

    std::io::stdin().read_line(&mut number).unwrap();

    match number.trim().parse::<u8>() {
        Ok(number) => Some(number),
        Err(_) => None
    }
}