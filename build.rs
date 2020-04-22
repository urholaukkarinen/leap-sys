use bindgen;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("LeapC.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}