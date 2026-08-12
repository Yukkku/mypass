use serde::Deserialize;
use std::{
    fmt::{Debug, Display},
    num::NonZero,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum AsciiPrintable {
    Space = b' ',
    ExclamationMark = b'!',
    QuotationMark = b'"',
    NumberSign = b'#',
    DollarSign = b'$',
    PercentSign = b'%',
    Ampersand = b'&',
    Apostrophe = b'\'',
    LeftParenthesis = b'(',
    RightParenthesis = b')',
    Asterisk = b'*',
    PlusSign = b'+',
    Comma = b',',
    HyphenMinus = b'-',
    FullStop = b'.',
    Slash = b'/',
    DigitZero = b'0',
    DigitOne = b'1',
    DigitTwo = b'2',
    DigitThree = b'3',
    DigitFour = b'4',
    DigitFive = b'5',
    DigitSix = b'6',
    DigitSeven = b'7',
    DigitEight = b'8',
    DigitNine = b'9',
    Colon = b':',
    Semicolon = b';',
    LessThanSign = b'<',
    EqualsSign = b'=',
    GreaterThanSign = b'>',
    QuestionMark = b'?',
    CommercialAt = b'@',
    LatinCapitalLetterA = b'A',
    LatinCapitalLetterB = b'B',
    LatinCapitalLetterC = b'C',
    LatinCapitalLetterD = b'D',
    LatinCapitalLetterE = b'E',
    LatinCapitalLetterF = b'F',
    LatinCapitalLetterG = b'G',
    LatinCapitalLetterH = b'H',
    LatinCapitalLetterI = b'I',
    LatinCapitalLetterJ = b'J',
    LatinCapitalLetterK = b'K',
    LatinCapitalLetterL = b'L',
    LatinCapitalLetterM = b'M',
    LatinCapitalLetterN = b'N',
    LatinCapitalLetterO = b'O',
    LatinCapitalLetterP = b'P',
    LatinCapitalLetterQ = b'Q',
    LatinCapitalLetterR = b'R',
    LatinCapitalLetterS = b'S',
    LatinCapitalLetterT = b'T',
    LatinCapitalLetterU = b'U',
    LatinCapitalLetterV = b'V',
    LatinCapitalLetterW = b'W',
    LatinCapitalLetterX = b'X',
    LatinCapitalLetterY = b'Y',
    LatinCapitalLetterZ = b'Z',
    LeftSquareBracket = b'[',
    Backslash = b'\\',
    RightSquareBracket = b']',
    CircumflexAccent = b'^',
    LowLine = b'_',
    GraveAccent = b'`',
    LatinSmallLetterA = b'a',
    LatinSmallLetterB = b'b',
    LatinSmallLetterC = b'c',
    LatinSmallLetterD = b'd',
    LatinSmallLetterE = b'e',
    LatinSmallLetterF = b'f',
    LatinSmallLetterG = b'g',
    LatinSmallLetterH = b'h',
    LatinSmallLetterI = b'i',
    LatinSmallLetterJ = b'j',
    LatinSmallLetterK = b'k',
    LatinSmallLetterL = b'l',
    LatinSmallLetterM = b'm',
    LatinSmallLetterN = b'n',
    LatinSmallLetterO = b'o',
    LatinSmallLetterP = b'p',
    LatinSmallLetterQ = b'q',
    LatinSmallLetterR = b'r',
    LatinSmallLetterS = b's',
    LatinSmallLetterT = b't',
    LatinSmallLetterU = b'u',
    LatinSmallLetterV = b'v',
    LatinSmallLetterW = b'w',
    LatinSmallLetterX = b'x',
    LatinSmallLetterY = b'y',
    LatinSmallLetterZ = b'z',
    LeftCurlyBracket = b'{',
    VerticalLine = b'|',
    RightCurlyBracket = b'}',
    Tilde = b'~',
}

const ASCII_PRINTABLE_RANGE: std::ops::RangeInclusive<u8> = b' '..=b'~';
const ASCII_PRINTABLE_COUNT: usize = ((b'~' - b' ') + 1) as usize;

impl From<AsciiPrintable> for char {
    fn from(value: AsciiPrintable) -> Self {
        value as u8 as char
    }
}

impl Display for AsciiPrintable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&char::from(*self), f)
    }
}
impl Debug for AsciiPrintable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&char::from(*self), f)
    }
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "Box<str>")]
pub struct CharGroup(Box<[AsciiPrintable]>);
impl CharGroup {
    fn check(&self) {
        unsafe {
            std::hint::assert_unchecked(1 <= self.0.len());
            std::hint::assert_unchecked(self.0.len() <= ASCII_PRINTABLE_COUNT);
        }
    }

    #[must_use]
    pub fn len(&self) -> NonZero<u8> {
        self.check();
        unsafe { NonZero::new_unchecked(self.0.len() as u8) }
    }

    #[must_use]
    pub unsafe fn get_unchecked(&self, index: u8) -> &AsciiPrintable {
        self.check();
        unsafe {
            std::hint::assert_unchecked(index < self.0.len() as u8);
            self.0.get_unchecked(index as usize)
        }
    }
    #[must_use]
    pub unsafe fn get_unchecked_mut(&mut self, index: u8) -> &mut AsciiPrintable {
        self.check();
        unsafe {
            std::hint::assert_unchecked(index < self.0.len() as u8);
            self.0.get_unchecked_mut(index as usize)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CharGroupTryFromError {
    Empty,
    Duplicate,
    OutOfRange,
}
impl Display for CharGroupTryFromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => "provided string is empty",
            Self::Duplicate => "duplicate characters found",
            Self::OutOfRange => "string contains characters out of range",
        })
    }
}
impl std::error::Error for CharGroupTryFromError {}

impl TryFrom<Box<[u8]>> for CharGroup {
    type Error = CharGroupTryFromError;
    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        if value.iter().any(|c| !ASCII_PRINTABLE_RANGE.contains(c)) {
            return Err(CharGroupTryFromError::OutOfRange);
        }
        unsafe { Box::from_raw(Box::into_raw(value) as *mut [AsciiPrintable]) }.try_into()
    }
}
impl TryFrom<Box<str>> for CharGroup {
    type Error = CharGroupTryFromError;
    fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
        value.into_boxed_bytes().try_into()
    }
}
impl TryFrom<String> for CharGroup {
    type Error = CharGroupTryFromError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.into_boxed_str().try_into()
    }
}
impl TryFrom<Box<[AsciiPrintable]>> for CharGroup {
    type Error = CharGroupTryFromError;
    fn try_from(mut value: Box<[AsciiPrintable]>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(CharGroupTryFromError::Empty);
        }
        value.sort_unstable();
        for &[a, b] in value.array_windows() {
            if a == b {
                return Err(CharGroupTryFromError::Duplicate);
            }
        }
        Ok(Self(value))
    }
}
