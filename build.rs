fn main() {
    println!("cargo:rustc-link-arg-bin=meos=-Tlink_script.ld");
}
