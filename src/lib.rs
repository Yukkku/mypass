use serde::Deserialize;
use std::num::NonZero;

mod ascii;
use ascii::{AsciiPrintable, CharGroup};

struct PassSource(sha3::Shake256Reader);
impl PassSource {
    #[must_use]
    fn new(service: &str, masterpass: &str, info: &str) -> Self {
        use sha3::{
            Shake256,
            digest::{ExtendableOutput, Update},
        };
        let mut hasher = Shake256::default();
        hasher.update(masterpass.as_bytes());
        hasher.update(b"\n");
        hasher.update(info.as_bytes());
        hasher.update(b"\n");
        hasher.update(service.as_bytes());
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

    fn choose(&mut self, group: &CharGroup) -> AsciiPrintable {
        let i = self.next_max(group.len().get() - 1);
        unsafe { *group.get_unchecked(i) }
    }
}

fn default_allow_chars() -> CharGroup {
    Box::<str>::from(
        r##"!"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##,
    ).try_into().unwrap()
}

fn default_requires() -> Box<[CharGroup]> {
    [].into()
}

#[derive(Deserialize)]
pub struct Config {
    pub len: NonZero<u8>,
    #[serde(default = "default_allow_chars")]
    pub allow_chars: CharGroup,
    #[serde(default = "default_requires")]
    pub requires: Box<[CharGroup]>,
    #[serde(default)]
    pub info: String,
}

pub fn generate(conf: &Config, service: &str, masterpass: &str) -> Box<str> {
    let mut source = PassSource::new(service, masterpass, &conf.info);
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
