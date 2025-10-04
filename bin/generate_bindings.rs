use camino::Utf8PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the library first
    println!("Building library...");
    let build_status = Command::new("cargo")
        .args(&["build", "--release"])
        .status()
        .expect("Failed to build library");

    if !build_status.success() {
        eprintln!("Build failed!");
        std::process::exit(1);
    }

    let udl_file = Utf8PathBuf::from("./src/electrum_client.udl");
    let out_dir_kotlin = Utf8PathBuf::from("./android/src/main/java");
    let out_dir_swift = Utf8PathBuf::from("./ios");

    // Generate Kotlin bindings
    println!("Generating Kotlin bindings...");
    uniffi_bindgen::bindings::kotlin::generate_bindings(
        &udl_file,
        None,
        &out_dir_kotlin,
        None,
        &uniffi_bindgen::bindings::kotlin::Config::default(),
        false,
    )?;

    // Generate Swift bindings
    println!("Generating Swift bindings...");
    uniffi_bindgen::bindings::swift::generate_bindings(
        &udl_file,
        None,
        &out_dir_swift,
        None,
        &uniffi_bindgen::bindings::swift::Config::default(),
        false,
    )?;

    println!("Bindings generated successfully!");
    Ok(())
}
