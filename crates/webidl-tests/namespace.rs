use wasm_bindgen_test::*;

include!(concat!(env!("OUT_DIR"), "/namespace.rs"));

#[wasm_bindgen_test]
fn simple_namespace_test() {
    assert_eq!(mathtest::add_one(1), 2);
    assert_eq!(mathtest::powf(1.0, 100.0), 1.0);
    assert_eq!(mathtest::powf(10.0, 2.0), 100.0);
}