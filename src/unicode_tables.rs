use crate::*;

extern "C" {
    pub static mut toLower: [convertStruct; 172];
    pub static mut toUpper: [convertStruct; 187];
    pub static mut combining: [interval; 280];
    pub static mut foldCase: [convertStruct; 192];
    pub static mut doublewidth: [interval; 113];
    pub static mut ambiguous: [interval; 179];
    pub static mut emoji_all: [interval; 151];
    pub static mut emoji_width: [interval; 39];
}
