fn main() {
    println!("cargo:rustc-link-arg=-T../../../tkey-libs/app.lds");
    println!("cargo:rustc-link-arg=--export-dynamic-symbol=_start");
}
