use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // filename: String,
    #[serde(with = "serde_bytes")]
    code: Vec<u8>,
}

#[napi]
pub fn transform(opts: Object, env: Env) -> napi::Result<()> {
    let config: Config = env.from_js_value(opts)?;
    // println!("filename: {:?}", config.filename);
    println!("code: {:?}", config.code);

    Ok(())
}
