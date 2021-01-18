fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "picolua54")]
    let version = lua_src::PicoLua54;

    let artifacts = lua_src::Build::new().build(version);
    artifacts.print_cargo_metadata();

    println!("owooo");
}
