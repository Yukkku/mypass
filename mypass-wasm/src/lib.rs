use serde::Deserialize;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
pub struct Config {
    pub len: u8,
    #[tsify(optional)]
    pub allow_chars: Option<String>,
    #[tsify(optional)]
    pub requires: Option<Box<[String]>>,
    #[tsify(optional)]
    pub info: Option<String>,
}

#[wasm_bindgen]
pub fn generate(config: Config, service: &str, masterpass: &[u8]) -> Option<String> {
    let len = config.len.try_into().ok()?;

    let allow_chars = if let Some(allow_chars) = config.allow_chars {
        allow_chars.try_into().ok()?
    } else {
        mypass::Config::default_allow_chars()
    };

    let requires = if let Some(requires) = config.requires {
        let mut temp = Vec::with_capacity(requires.len());
        for req in requires {
            temp.push(req.try_into().ok()?);
        }
        temp.into()
    } else {
        mypass::Config::default_requires()
    };

    let info = config.info.unwrap_or_else(String::new);

    let config = mypass::Config {
        len,
        allow_chars,
        requires,
        info,
    };
    Some(mypass::generate(&config, service, masterpass).into())
}
