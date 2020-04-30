#[cfg(feature = "buildtime_bindgen")]
extern crate bindgen;

fn main() {
    #[cfg(feature = "buildtime_bindgen")]
    bindgen::Builder::default()
        .header("LeapC.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}