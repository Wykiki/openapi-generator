use core;
use openapi::v3_0::Spec;
use openapi::OpenApi;

fn v3_0(openapi: OpenApi) -> Option<Spec> {
    match openapi {
        OpenApi::V3_0(spec) => Some(spec),
        _ => None,
    }
}

#[test]
fn parse_test() {
    let elem = core::parse();
    assert_eq!(true, elem.is_ok());
    let spec = v3_0(elem.ok().unwrap()).unwrap();
    println!("{:?}", spec.components);
}
