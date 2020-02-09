use core;

#[test]
fn from_path_test() {
    let elem = core::from_path("res/petstore.yml");
    assert_eq!(true, elem.is_ok());
    let spec = elem.ok();
    dbg!(spec);
}
