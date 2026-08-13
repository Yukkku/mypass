use serde::Deserialize;
use std::num::NonZero;

mod ascii;
use ascii::AsciiPrintable;
pub use ascii::CharGroup;

struct PassSource(sha3::Shake256Reader);
impl PassSource {
    #[must_use]
    fn new(service: &str, phrase: &str, masterpass: &[u8], info: &str) -> Self {
        use sha3::{
            Shake256,
            digest::{ExtendableOutput, Update},
        };
        let mut hasher = Shake256::default();
        hasher.update(phrase.as_bytes());
        hasher.update(b"\0\n");
        hasher.update(service.as_bytes());
        hasher.update(b"\0\n");
        hasher.update(info.as_bytes());
        hasher.update(b"\0\n");
        hasher.update(masterpass);
        Self(hasher.finalize_xof())
    }

    fn next(&mut self) -> u16 {
        use sha3::digest::XofReader;
        let mut bytes = [0; 2];
        self.0.read(&mut bytes);
        u16::from_be_bytes(bytes)
    }

    #[must_use]
    fn next_max(&mut self, n: u8) -> u8 {
        let r = (self.next() as u32 * (n as u32 + 1) >> 16) as u8;
        unsafe { std::hint::assert_unchecked(r <= n) };
        r
    }

    #[must_use]
    fn choose(&mut self, group: &CharGroup) -> AsciiPrintable {
        let i = self.next_max(group.len().get() - 1);
        unsafe { *group.get_unchecked(i) }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub len: NonZero<u8>,
    #[serde(default = "Config::default_allow_chars")]
    pub allow_chars: CharGroup,
    #[serde(default = "Config::default_requires")]
    pub requires: Box<[CharGroup]>,
    #[serde(default)]
    pub info: String,
}

impl Config {
    #[must_use]
    pub fn default_allow_chars() -> CharGroup {
        Box::<str>::from(r##"!"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##).try_into().unwrap()
    }

    #[must_use]
    pub fn default_requires() -> Box<[CharGroup]> {
        [].into()
    }
}

#[must_use]
pub fn generate(conf: &Config, service: &str, phrase: &str, masterpass: &[u8]) -> Box<str> {
    let mut source = PassSource::new(service, phrase, masterpass, &conf.info);
    let mut pass = conf
        .requires
        .iter()
        .chain(std::iter::repeat(&conf.allow_chars))
        .map(|group| source.choose(group) as u8)
        .take(conf.len.get() as usize)
        .collect::<Box<_>>();
    for i in 1..conf.len.get() {
        pass.swap(source.next_max(i) as usize, i as usize);
    }
    unsafe { std::str::from_boxed_utf8_unchecked(pass) }
}
