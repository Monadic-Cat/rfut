fn main() {
    println!("cargo:rerun-if-changed=src/stuff.fut");
    let mut stuff_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    stuff_path.push("stuff");
    std::process::Command::new("futhark")
        .args(&["opencl", "--library", "src/stuff.fut", "-o", stuff_path.to_str().unwrap()])
        .output()
        .expect("failed to run futhark compiler");
    stuff_path.pop();
    stuff_path.push("stuff.c");
    cc::Build::new()
        .warnings(false)
        .file(stuff_path)
        .compile("stuff");
}
