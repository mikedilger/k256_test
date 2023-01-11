use k256::schnorr::VerifyingKey;

fn main() {
    let works = "c48b5cced5ada74db078df6b00fa53fc1139d73bf0ed16de325d52220211dbd5";
    let fails = "1bffcc47015d7e0c9422a7083c08d3247b22421c76d328c03724e1ad1cbca731";

    let works_bytes: Vec<u8> = hex::decode(works).unwrap();
    verify(&works_bytes);

    let fails_bytes: Vec<u8> = hex::decode(fails).unwrap();
    verify(&fails_bytes);
}

fn verify(bytes: &[u8]) {
    match VerifyingKey::from_bytes(bytes) {
        Ok(_) => println!("Key is a valid schnorr taproot public key"),
        Err(e) => println!("{}", e),
    };
}
