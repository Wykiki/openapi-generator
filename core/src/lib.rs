use openapi::error::Error;
use openapi::from_path;
use openapi::OpenApi;

pub fn parse() -> Result<OpenApi, Error> {
    from_path("res/petstore.yml")
}
