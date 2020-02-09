use super::error::CodeGenError;

use openapi::v3_0::Spec;
use openapi::OpenApi;

pub fn extract_v3(oas: OpenApi) -> Result<Spec, CodeGenError> {
    match oas {
        OpenApi::V3_0(s) => Ok(s),
        _ => Err(CodeGenError::new(
            format!("OAS has a version different from V3"),
            None,
        )),
    }
}

pub fn resolve_refs(mut oas: Spec) -> Result<Spec, CodeGenError> {
    let mut paths = oas.paths;
    if paths.is_empty() {
        return Err(CodeGenError::new(
            format!("OAS does not contain any path definition"),
            None,
        ));
    };
    let components = match oas.components {
        Some(components) => components,
        None => {
            return Err(CodeGenError::new(
                format!("OAS does not contain any component definition"),
                None,
            ))
        }
    };
    Ok(Spec::default())
}
