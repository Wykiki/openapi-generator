mod domain;
mod error;
mod oas;

use std::error::Error;
use std::path::Path;

use openapi::OpenApi;

use domain::CodeGen;
use error::CodeGenError;

pub fn from_path<P: AsRef<Path>>(path: P) -> Result<CodeGen, CodeGenError> {
    let oas_src = match openapi::from_path(path) {
        Ok(oas) => oas,
        Err(e) => {
            return Err(CodeGenError::new(
                format!("Errors in OpenAPI parsing : {}", e),
                None,
            ))
        }
    };
    let spec = oas::extract_v3(oas_src)?;
    let resolved_specs = oas::resolve_refs(spec)?;
    // dbg!(oas_src);
    Ok(CodeGen::default())
}
