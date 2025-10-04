fn main() {
    uniffi::generate_scaffolding("./src/electrum_client.udl").unwrap();
}
