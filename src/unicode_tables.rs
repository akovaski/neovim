use crate::*;

pub static mut toLower: [convertStruct; 172] =
    [{
         let mut init =
             convertStruct{rangeStart: 0x41 as libc::c_int,
                           rangeEnd: 0x5a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xc0 as libc::c_int,
                           rangeEnd: 0xd6 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xd8 as libc::c_int,
                           rangeEnd: 0xde as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x100 as libc::c_int,
                           rangeEnd: 0x12e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x130 as libc::c_int,
                           rangeEnd: 0x130 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(199 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x132 as libc::c_int,
                           rangeEnd: 0x136 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x139 as libc::c_int,
                           rangeEnd: 0x147 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x14a as libc::c_int,
                           rangeEnd: 0x176 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x178 as libc::c_int,
                           rangeEnd: 0x178 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(121 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x179 as libc::c_int,
                           rangeEnd: 0x17d as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x181 as libc::c_int,
                           rangeEnd: 0x181 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 210 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x182 as libc::c_int,
                           rangeEnd: 0x184 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x186 as libc::c_int,
                           rangeEnd: 0x186 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 206 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x187 as libc::c_int,
                           rangeEnd: 0x187 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x189 as libc::c_int,
                           rangeEnd: 0x18a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 205 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18b as libc::c_int,
                           rangeEnd: 0x18b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18e as libc::c_int,
                           rangeEnd: 0x18e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 79 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18f as libc::c_int,
                           rangeEnd: 0x18f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 202 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x190 as libc::c_int,
                           rangeEnd: 0x190 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 203 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x191 as libc::c_int,
                           rangeEnd: 0x191 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x193 as libc::c_int,
                           rangeEnd: 0x193 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 205 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x194 as libc::c_int,
                           rangeEnd: 0x194 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 207 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x196 as libc::c_int,
                           rangeEnd: 0x196 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 211 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x197 as libc::c_int,
                           rangeEnd: 0x197 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 209 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x198 as libc::c_int,
                           rangeEnd: 0x198 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19c as libc::c_int,
                           rangeEnd: 0x19c as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 211 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19d as libc::c_int,
                           rangeEnd: 0x19d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 213 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19f as libc::c_int,
                           rangeEnd: 0x19f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 214 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a0 as libc::c_int,
                           rangeEnd: 0x1a4 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a6 as libc::c_int,
                           rangeEnd: 0x1a6 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 218 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a7 as libc::c_int,
                           rangeEnd: 0x1a7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a9 as libc::c_int,
                           rangeEnd: 0x1a9 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 218 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ac as libc::c_int,
                           rangeEnd: 0x1ac as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ae as libc::c_int,
                           rangeEnd: 0x1ae as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 218 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1af as libc::c_int,
                           rangeEnd: 0x1af as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b1 as libc::c_int,
                           rangeEnd: 0x1b2 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 217 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b3 as libc::c_int,
                           rangeEnd: 0x1b5 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b7 as libc::c_int,
                           rangeEnd: 0x1b7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 219 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b8 as libc::c_int,
                           rangeEnd: 0x1bc as libc::c_int,
                           step: 4 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c4 as libc::c_int,
                           rangeEnd: 0x1c4 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c5 as libc::c_int,
                           rangeEnd: 0x1c5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c7 as libc::c_int,
                           rangeEnd: 0x1c7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c8 as libc::c_int,
                           rangeEnd: 0x1c8 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ca as libc::c_int,
                           rangeEnd: 0x1ca as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cb as libc::c_int,
                           rangeEnd: 0x1db as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1de as libc::c_int,
                           rangeEnd: 0x1ee as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f1 as libc::c_int,
                           rangeEnd: 0x1f1 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f2 as libc::c_int,
                           rangeEnd: 0x1f4 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f6 as libc::c_int,
                           rangeEnd: 0x1f6 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(97 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f7 as libc::c_int,
                           rangeEnd: 0x1f7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(56 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f8 as libc::c_int,
                           rangeEnd: 0x21e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x220 as libc::c_int,
                           rangeEnd: 0x220 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(130 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x222 as libc::c_int,
                           rangeEnd: 0x232 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23a as libc::c_int,
                           rangeEnd: 0x23a as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10795 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23b as libc::c_int,
                           rangeEnd: 0x23b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23d as libc::c_int,
                           rangeEnd: 0x23d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(163 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23e as libc::c_int,
                           rangeEnd: 0x23e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10792 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x241 as libc::c_int,
                           rangeEnd: 0x241 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x243 as libc::c_int,
                           rangeEnd: 0x243 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(195 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x244 as libc::c_int,
                           rangeEnd: 0x244 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 69 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x245 as libc::c_int,
                           rangeEnd: 0x245 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 71 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x246 as libc::c_int,
                           rangeEnd: 0x24e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x370 as libc::c_int,
                           rangeEnd: 0x372 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x376 as libc::c_int,
                           rangeEnd: 0x376 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x37f as libc::c_int,
                           rangeEnd: 0x37f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 116 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x386 as libc::c_int,
                           rangeEnd: 0x386 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 38 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x388 as libc::c_int,
                           rangeEnd: 0x38a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 37 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x38c as libc::c_int,
                           rangeEnd: 0x38c as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 64 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x38e as libc::c_int,
                           rangeEnd: 0x38f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 63 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x391 as libc::c_int,
                           rangeEnd: 0x3a1 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3a3 as libc::c_int,
                           rangeEnd: 0x3ab as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3cf as libc::c_int,
                           rangeEnd: 0x3cf as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d8 as libc::c_int,
                           rangeEnd: 0x3ee as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f4 as libc::c_int,
                           rangeEnd: 0x3f4 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(60 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f7 as libc::c_int,
                           rangeEnd: 0x3f7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f9 as libc::c_int,
                           rangeEnd: 0x3f9 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3fa as libc::c_int,
                           rangeEnd: 0x3fa as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3fd as libc::c_int,
                           rangeEnd: 0x3ff as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(130 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x400 as libc::c_int,
                           rangeEnd: 0x40f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 80 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x410 as libc::c_int,
                           rangeEnd: 0x42f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x460 as libc::c_int,
                           rangeEnd: 0x480 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x48a as libc::c_int,
                           rangeEnd: 0x4be as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c0 as libc::c_int,
                           rangeEnd: 0x4c0 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 15 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c1 as libc::c_int,
                           rangeEnd: 0x4cd as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4d0 as libc::c_int,
                           rangeEnd: 0x52e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x531 as libc::c_int,
                           rangeEnd: 0x556 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 48 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10a0 as libc::c_int,
                           rangeEnd: 0x10c5 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 7264 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10c7 as libc::c_int,
                           rangeEnd: 0x10cd as libc::c_int,
                           step: 6 as libc::c_int,
                           offset: 7264 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13a0 as libc::c_int,
                           rangeEnd: 0x13ef as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 38864 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13f0 as libc::c_int,
                           rangeEnd: 0x13f5 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c90 as libc::c_int,
                           rangeEnd: 0x1cba as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(3008 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cbd as libc::c_int,
                           rangeEnd: 0x1cbf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(3008 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e00 as libc::c_int,
                           rangeEnd: 0x1e94 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e9e as libc::c_int,
                           rangeEnd: 0x1e9e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7615 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ea0 as libc::c_int,
                           rangeEnd: 0x1efe as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f08 as libc::c_int,
                           rangeEnd: 0x1f0f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f18 as libc::c_int,
                           rangeEnd: 0x1f1d as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f28 as libc::c_int,
                           rangeEnd: 0x1f2f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f38 as libc::c_int,
                           rangeEnd: 0x1f3f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f48 as libc::c_int,
                           rangeEnd: 0x1f4d as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f59 as libc::c_int,
                           rangeEnd: 0x1f5f as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f68 as libc::c_int,
                           rangeEnd: 0x1f6f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f88 as libc::c_int,
                           rangeEnd: 0x1f8f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f98 as libc::c_int,
                           rangeEnd: 0x1f9f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fa8 as libc::c_int,
                           rangeEnd: 0x1faf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb8 as libc::c_int,
                           rangeEnd: 0x1fb9 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fba as libc::c_int,
                           rangeEnd: 0x1fbb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(74 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fbc as libc::c_int,
                           rangeEnd: 0x1fbc as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fc8 as libc::c_int,
                           rangeEnd: 0x1fcb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(86 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fcc as libc::c_int,
                           rangeEnd: 0x1fcc as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fd8 as libc::c_int,
                           rangeEnd: 0x1fd9 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fda as libc::c_int,
                           rangeEnd: 0x1fdb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(100 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fe8 as libc::c_int,
                           rangeEnd: 0x1fe9 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fea as libc::c_int,
                           rangeEnd: 0x1feb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(112 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fec as libc::c_int,
                           rangeEnd: 0x1fec as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ff8 as libc::c_int,
                           rangeEnd: 0x1ff9 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(128 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ffa as libc::c_int,
                           rangeEnd: 0x1ffb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(126 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ffc as libc::c_int,
                           rangeEnd: 0x1ffc as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2126 as libc::c_int,
                           rangeEnd: 0x2126 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7517 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x212a as libc::c_int,
                           rangeEnd: 0x212a as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(8383 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x212b as libc::c_int,
                           rangeEnd: 0x212b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(8262 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2132 as libc::c_int,
                           rangeEnd: 0x2132 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 28 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2160 as libc::c_int,
                           rangeEnd: 0x216f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 16 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2183 as libc::c_int,
                           rangeEnd: 0x2183 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x24b6 as libc::c_int,
                           rangeEnd: 0x24cf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 26 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c00 as libc::c_int,
                           rangeEnd: 0x2c2e as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 48 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c60 as libc::c_int,
                           rangeEnd: 0x2c60 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c62 as libc::c_int,
                           rangeEnd: 0x2c62 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10743 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c63 as libc::c_int,
                           rangeEnd: 0x2c63 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(3814 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c64 as libc::c_int,
                           rangeEnd: 0x2c64 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10727 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c67 as libc::c_int,
                           rangeEnd: 0x2c6b as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6d as libc::c_int,
                           rangeEnd: 0x2c6d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10780 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6e as libc::c_int,
                           rangeEnd: 0x2c6e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10749 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6f as libc::c_int,
                           rangeEnd: 0x2c6f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10783 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c70 as libc::c_int,
                           rangeEnd: 0x2c70 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10782 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c72 as libc::c_int,
                           rangeEnd: 0x2c75 as libc::c_int,
                           step: 3 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c7e as libc::c_int,
                           rangeEnd: 0x2c7f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(10815 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c80 as libc::c_int,
                           rangeEnd: 0x2ce2 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2ceb as libc::c_int,
                           rangeEnd: 0x2ced as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2cf2 as libc::c_int,
                           rangeEnd: 0xa640 as libc::c_int,
                           step: 31054 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa642 as libc::c_int,
                           rangeEnd: 0xa66c as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa680 as libc::c_int,
                           rangeEnd: 0xa69a as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa722 as libc::c_int,
                           rangeEnd: 0xa72e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa732 as libc::c_int,
                           rangeEnd: 0xa76e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa779 as libc::c_int,
                           rangeEnd: 0xa77b as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77d as libc::c_int,
                           rangeEnd: 0xa77d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(35332 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77e as libc::c_int,
                           rangeEnd: 0xa786 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78b as libc::c_int,
                           rangeEnd: 0xa78b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78d as libc::c_int,
                           rangeEnd: 0xa78d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42280 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa790 as libc::c_int,
                           rangeEnd: 0xa792 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa796 as libc::c_int,
                           rangeEnd: 0xa7a8 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7aa as libc::c_int,
                           rangeEnd: 0xa7aa as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42308 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ab as libc::c_int,
                           rangeEnd: 0xa7ab as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42319 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ac as libc::c_int,
                           rangeEnd: 0xa7ac as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42315 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ad as libc::c_int,
                           rangeEnd: 0xa7ad as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42305 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ae as libc::c_int,
                           rangeEnd: 0xa7ae as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42308 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b0 as libc::c_int,
                           rangeEnd: 0xa7b0 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42258 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b1 as libc::c_int,
                           rangeEnd: 0xa7b1 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42282 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b2 as libc::c_int,
                           rangeEnd: 0xa7b2 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42261 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b3 as libc::c_int,
                           rangeEnd: 0xa7b3 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 928 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b4 as libc::c_int,
                           rangeEnd: 0xa7be as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c2 as libc::c_int,
                           rangeEnd: 0xa7c2 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c4 as libc::c_int,
                           rangeEnd: 0xa7c4 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(48 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c5 as libc::c_int,
                           rangeEnd: 0xa7c5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42307 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c6 as libc::c_int,
                           rangeEnd: 0xa7c6 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(35384 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xff21 as libc::c_int,
                           rangeEnd: 0xff3a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10400 as libc::c_int,
                           rangeEnd: 0x10427 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 40 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x104b0 as libc::c_int,
                           rangeEnd: 0x104d3 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 40 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10c80 as libc::c_int,
                           rangeEnd: 0x10cb2 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 64 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x118a0 as libc::c_int,
                           rangeEnd: 0x118bf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x16e40 as libc::c_int,
                           rangeEnd: 0x16e5f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e900 as libc::c_int,
                           rangeEnd: 0x1e921 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 34 as libc::c_int,};
         init
     }];
pub static mut toUpper: [convertStruct; 187] =
    [{
         let mut init =
             convertStruct{rangeStart: 0x61 as libc::c_int,
                           rangeEnd: 0x7a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(32 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xb5 as libc::c_int,
                           rangeEnd: 0xb5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 743 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xe0 as libc::c_int,
                           rangeEnd: 0xf6 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(32 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xf8 as libc::c_int,
                           rangeEnd: 0xfe as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(32 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xff as libc::c_int,
                           rangeEnd: 0xff as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 121 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x101 as libc::c_int,
                           rangeEnd: 0x12f as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x131 as libc::c_int,
                           rangeEnd: 0x131 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(232 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x133 as libc::c_int,
                           rangeEnd: 0x137 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13a as libc::c_int,
                           rangeEnd: 0x148 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x14b as libc::c_int,
                           rangeEnd: 0x177 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x17a as libc::c_int,
                           rangeEnd: 0x17e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x17f as libc::c_int,
                           rangeEnd: 0x17f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(300 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x180 as libc::c_int,
                           rangeEnd: 0x180 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 195 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x183 as libc::c_int,
                           rangeEnd: 0x185 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x188 as libc::c_int,
                           rangeEnd: 0x18c as libc::c_int,
                           step: 4 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x192 as libc::c_int,
                           rangeEnd: 0x192 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x195 as libc::c_int,
                           rangeEnd: 0x195 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 97 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x199 as libc::c_int,
                           rangeEnd: 0x199 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19a as libc::c_int,
                           rangeEnd: 0x19a as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 163 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19e as libc::c_int,
                           rangeEnd: 0x19e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 130 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a1 as libc::c_int,
                           rangeEnd: 0x1a5 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a8 as libc::c_int,
                           rangeEnd: 0x1ad as libc::c_int,
                           step: 5 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b0 as libc::c_int,
                           rangeEnd: 0x1b4 as libc::c_int,
                           step: 4 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b6 as libc::c_int,
                           rangeEnd: 0x1b9 as libc::c_int,
                           step: 3 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1bd as libc::c_int,
                           rangeEnd: 0x1bd as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1bf as libc::c_int,
                           rangeEnd: 0x1bf as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 56 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c5 as libc::c_int,
                           rangeEnd: 0x1c5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c6 as libc::c_int,
                           rangeEnd: 0x1c6 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c8 as libc::c_int,
                           rangeEnd: 0x1c8 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c9 as libc::c_int,
                           rangeEnd: 0x1c9 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cb as libc::c_int,
                           rangeEnd: 0x1cb as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cc as libc::c_int,
                           rangeEnd: 0x1cc as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ce as libc::c_int,
                           rangeEnd: 0x1dc as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1dd as libc::c_int,
                           rangeEnd: 0x1dd as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(79 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1df as libc::c_int,
                           rangeEnd: 0x1ef as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f2 as libc::c_int,
                           rangeEnd: 0x1f2 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f3 as libc::c_int,
                           rangeEnd: 0x1f3 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(2 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f5 as libc::c_int,
                           rangeEnd: 0x1f9 as libc::c_int,
                           step: 4 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb as libc::c_int,
                           rangeEnd: 0x21f as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x223 as libc::c_int,
                           rangeEnd: 0x233 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23c as libc::c_int,
                           rangeEnd: 0x23c as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23f as libc::c_int,
                           rangeEnd: 0x240 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 10815 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x242 as libc::c_int,
                           rangeEnd: 0x247 as libc::c_int,
                           step: 5 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x249 as libc::c_int,
                           rangeEnd: 0x24f as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x250 as libc::c_int,
                           rangeEnd: 0x250 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10783 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x251 as libc::c_int,
                           rangeEnd: 0x251 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10780 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x252 as libc::c_int,
                           rangeEnd: 0x252 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10782 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x253 as libc::c_int,
                           rangeEnd: 0x253 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(210 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x254 as libc::c_int,
                           rangeEnd: 0x254 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(206 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x256 as libc::c_int,
                           rangeEnd: 0x257 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(205 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x259 as libc::c_int,
                           rangeEnd: 0x259 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(202 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x25b as libc::c_int,
                           rangeEnd: 0x25b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(203 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x25c as libc::c_int,
                           rangeEnd: 0x25c as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42319 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x260 as libc::c_int,
                           rangeEnd: 0x260 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(205 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x261 as libc::c_int,
                           rangeEnd: 0x261 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42315 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x263 as libc::c_int,
                           rangeEnd: 0x263 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(207 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x265 as libc::c_int,
                           rangeEnd: 0x265 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42280 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x266 as libc::c_int,
                           rangeEnd: 0x266 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42308 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x268 as libc::c_int,
                           rangeEnd: 0x268 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(209 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x269 as libc::c_int,
                           rangeEnd: 0x269 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(211 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x26a as libc::c_int,
                           rangeEnd: 0x26a as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42308 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x26b as libc::c_int,
                           rangeEnd: 0x26b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10743 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x26c as libc::c_int,
                           rangeEnd: 0x26c as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42305 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x26f as libc::c_int,
                           rangeEnd: 0x26f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(211 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x271 as libc::c_int,
                           rangeEnd: 0x271 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10749 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x272 as libc::c_int,
                           rangeEnd: 0x272 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(213 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x275 as libc::c_int,
                           rangeEnd: 0x275 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(214 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x27d as libc::c_int,
                           rangeEnd: 0x27d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10727 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x280 as libc::c_int,
                           rangeEnd: 0x280 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(218 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x282 as libc::c_int,
                           rangeEnd: 0x282 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42307 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x283 as libc::c_int,
                           rangeEnd: 0x283 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(218 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x287 as libc::c_int,
                           rangeEnd: 0x287 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42282 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x288 as libc::c_int,
                           rangeEnd: 0x288 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(218 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x289 as libc::c_int,
                           rangeEnd: 0x289 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(69 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x28a as libc::c_int,
                           rangeEnd: 0x28b as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(217 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x28c as libc::c_int,
                           rangeEnd: 0x28c as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(71 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x292 as libc::c_int,
                           rangeEnd: 0x292 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(219 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x29d as libc::c_int,
                           rangeEnd: 0x29d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42261 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x29e as libc::c_int,
                           rangeEnd: 0x29e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 42258 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x345 as libc::c_int,
                           rangeEnd: 0x345 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 84 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x371 as libc::c_int,
                           rangeEnd: 0x373 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x377 as libc::c_int,
                           rangeEnd: 0x377 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x37b as libc::c_int,
                           rangeEnd: 0x37d as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 130 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3ac as libc::c_int,
                           rangeEnd: 0x3ac as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(38 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3ad as libc::c_int,
                           rangeEnd: 0x3af as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(37 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3b1 as libc::c_int,
                           rangeEnd: 0x3c1 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(32 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3c2 as libc::c_int,
                           rangeEnd: 0x3c2 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(31 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3c3 as libc::c_int,
                           rangeEnd: 0x3cb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(32 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3cc as libc::c_int,
                           rangeEnd: 0x3cc as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(64 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3cd as libc::c_int,
                           rangeEnd: 0x3ce as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(63 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d0 as libc::c_int,
                           rangeEnd: 0x3d0 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(62 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d1 as libc::c_int,
                           rangeEnd: 0x3d1 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(57 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d5 as libc::c_int,
                           rangeEnd: 0x3d5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(47 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d6 as libc::c_int,
                           rangeEnd: 0x3d6 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(54 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d7 as libc::c_int,
                           rangeEnd: 0x3d7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d9 as libc::c_int,
                           rangeEnd: 0x3ef as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f0 as libc::c_int,
                           rangeEnd: 0x3f0 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(86 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f1 as libc::c_int,
                           rangeEnd: 0x3f1 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(80 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f2 as libc::c_int,
                           rangeEnd: 0x3f2 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f3 as libc::c_int,
                           rangeEnd: 0x3f3 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(116 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f5 as libc::c_int,
                           rangeEnd: 0x3f5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(96 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f8 as libc::c_int,
                           rangeEnd: 0x3fb as libc::c_int,
                           step: 3 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x430 as libc::c_int,
                           rangeEnd: 0x44f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(32 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x450 as libc::c_int,
                           rangeEnd: 0x45f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(80 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x461 as libc::c_int,
                           rangeEnd: 0x481 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x48b as libc::c_int,
                           rangeEnd: 0x4bf as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c2 as libc::c_int,
                           rangeEnd: 0x4ce as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4cf as libc::c_int,
                           rangeEnd: 0x4cf as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(15 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4d1 as libc::c_int,
                           rangeEnd: 0x52f as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x561 as libc::c_int,
                           rangeEnd: 0x586 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(48 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10d0 as libc::c_int,
                           rangeEnd: 0x10fa as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 3008 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10fd as libc::c_int,
                           rangeEnd: 0x10ff as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 3008 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13f8 as libc::c_int,
                           rangeEnd: 0x13fd as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c80 as libc::c_int,
                           rangeEnd: 0x1c80 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6254 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c81 as libc::c_int,
                           rangeEnd: 0x1c81 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6253 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c82 as libc::c_int,
                           rangeEnd: 0x1c82 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6244 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c83 as libc::c_int,
                           rangeEnd: 0x1c84 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(6242 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c85 as libc::c_int,
                           rangeEnd: 0x1c85 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6243 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c86 as libc::c_int,
                           rangeEnd: 0x1c86 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6236 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c87 as libc::c_int,
                           rangeEnd: 0x1c87 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6181 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c88 as libc::c_int,
                           rangeEnd: 0x1c88 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 35266 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1d79 as libc::c_int,
                           rangeEnd: 0x1d79 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 35332 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1d7d as libc::c_int,
                           rangeEnd: 0x1d7d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 3814 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1d8e as libc::c_int,
                           rangeEnd: 0x1d8e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 35384 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e01 as libc::c_int,
                           rangeEnd: 0x1e95 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e9b as libc::c_int,
                           rangeEnd: 0x1e9b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(59 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ea1 as libc::c_int,
                           rangeEnd: 0x1eff as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f00 as libc::c_int,
                           rangeEnd: 0x1f07 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f10 as libc::c_int,
                           rangeEnd: 0x1f15 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f20 as libc::c_int,
                           rangeEnd: 0x1f27 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f30 as libc::c_int,
                           rangeEnd: 0x1f37 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f40 as libc::c_int,
                           rangeEnd: 0x1f45 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f51 as libc::c_int,
                           rangeEnd: 0x1f57 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f60 as libc::c_int,
                           rangeEnd: 0x1f67 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f70 as libc::c_int,
                           rangeEnd: 0x1f71 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 74 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f72 as libc::c_int,
                           rangeEnd: 0x1f75 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 86 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f76 as libc::c_int,
                           rangeEnd: 0x1f77 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 100 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f78 as libc::c_int,
                           rangeEnd: 0x1f79 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 128 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f7a as libc::c_int,
                           rangeEnd: 0x1f7b as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 112 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f7c as libc::c_int,
                           rangeEnd: 0x1f7d as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 126 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f80 as libc::c_int,
                           rangeEnd: 0x1f87 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f90 as libc::c_int,
                           rangeEnd: 0x1f97 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fa0 as libc::c_int,
                           rangeEnd: 0x1fa7 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb0 as libc::c_int,
                           rangeEnd: 0x1fb1 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb3 as libc::c_int,
                           rangeEnd: 0x1fb3 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fbe as libc::c_int,
                           rangeEnd: 0x1fbe as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7205 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fc3 as libc::c_int,
                           rangeEnd: 0x1fc3 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fd0 as libc::c_int,
                           rangeEnd: 0x1fd1 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fe0 as libc::c_int,
                           rangeEnd: 0x1fe1 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fe5 as libc::c_int,
                           rangeEnd: 0x1fe5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 7 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ff3 as libc::c_int,
                           rangeEnd: 0x1ff3 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 9 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x214e as libc::c_int,
                           rangeEnd: 0x214e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(28 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2170 as libc::c_int,
                           rangeEnd: 0x217f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(16 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2184 as libc::c_int,
                           rangeEnd: 0x2184 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x24d0 as libc::c_int,
                           rangeEnd: 0x24e9 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(26 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c30 as libc::c_int,
                           rangeEnd: 0x2c5e as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(48 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c61 as libc::c_int,
                           rangeEnd: 0x2c61 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c65 as libc::c_int,
                           rangeEnd: 0x2c65 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10795 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c66 as libc::c_int,
                           rangeEnd: 0x2c66 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10792 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c68 as libc::c_int,
                           rangeEnd: 0x2c6c as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c73 as libc::c_int,
                           rangeEnd: 0x2c76 as libc::c_int,
                           step: 3 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c81 as libc::c_int,
                           rangeEnd: 0x2ce3 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2cec as libc::c_int,
                           rangeEnd: 0x2cee as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2cf3 as libc::c_int,
                           rangeEnd: 0x2cf3 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2d00 as libc::c_int,
                           rangeEnd: 0x2d25 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(7264 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2d27 as libc::c_int,
                           rangeEnd: 0x2d2d as libc::c_int,
                           step: 6 as libc::c_int,
                           offset: -(7264 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa641 as libc::c_int,
                           rangeEnd: 0xa66d as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa681 as libc::c_int,
                           rangeEnd: 0xa69b as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa723 as libc::c_int,
                           rangeEnd: 0xa72f as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa733 as libc::c_int,
                           rangeEnd: 0xa76f as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77a as libc::c_int,
                           rangeEnd: 0xa77c as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77f as libc::c_int,
                           rangeEnd: 0xa787 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78c as libc::c_int,
                           rangeEnd: 0xa791 as libc::c_int,
                           step: 5 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa793 as libc::c_int,
                           rangeEnd: 0xa793 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa794 as libc::c_int,
                           rangeEnd: 0xa794 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 48 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa797 as libc::c_int,
                           rangeEnd: 0xa7a9 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b5 as libc::c_int,
                           rangeEnd: 0xa7bf as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c3 as libc::c_int,
                           rangeEnd: 0xa7c3 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xab53 as libc::c_int,
                           rangeEnd: 0xab53 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(928 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xab70 as libc::c_int,
                           rangeEnd: 0xabbf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(38864 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xff41 as libc::c_int,
                           rangeEnd: 0xff5a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(32 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10428 as libc::c_int,
                           rangeEnd: 0x1044f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(40 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x104d8 as libc::c_int,
                           rangeEnd: 0x104fb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(40 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10cc0 as libc::c_int,
                           rangeEnd: 0x10cf2 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(64 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x118c0 as libc::c_int,
                           rangeEnd: 0x118df as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(32 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x16e60 as libc::c_int,
                           rangeEnd: 0x16e7f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(32 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e922 as libc::c_int,
                           rangeEnd: 0x1e943 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(34 as libc::c_int),};
         init
     }];
pub static mut combining: [interval; 280] =
    [{
         let mut init =
             interval{first: 0x300 as libc::c_int as libc::c_long,
                      last: 0x36f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x483 as libc::c_int as libc::c_long,
                      last: 0x489 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x591 as libc::c_int as libc::c_long,
                      last: 0x5bd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x5bf as libc::c_int as libc::c_long,
                      last: 0x5bf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x5c1 as libc::c_int as libc::c_long,
                      last: 0x5c2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x5c4 as libc::c_int as libc::c_long,
                      last: 0x5c5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x5c7 as libc::c_int as libc::c_long,
                      last: 0x5c7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x610 as libc::c_int as libc::c_long,
                      last: 0x61a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x64b as libc::c_int as libc::c_long,
                      last: 0x65f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x670 as libc::c_int as libc::c_long,
                      last: 0x670 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x6d6 as libc::c_int as libc::c_long,
                      last: 0x6dc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x6df as libc::c_int as libc::c_long,
                      last: 0x6e4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x6e7 as libc::c_int as libc::c_long,
                      last: 0x6e8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x6ea as libc::c_int as libc::c_long,
                      last: 0x6ed as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x711 as libc::c_int as libc::c_long,
                      last: 0x711 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x730 as libc::c_int as libc::c_long,
                      last: 0x74a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x7a6 as libc::c_int as libc::c_long,
                      last: 0x7b0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x7eb as libc::c_int as libc::c_long,
                      last: 0x7f3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x7fd as libc::c_int as libc::c_long,
                      last: 0x7fd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x816 as libc::c_int as libc::c_long,
                      last: 0x819 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x81b as libc::c_int as libc::c_long,
                      last: 0x823 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x825 as libc::c_int as libc::c_long,
                      last: 0x827 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x829 as libc::c_int as libc::c_long,
                      last: 0x82d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x859 as libc::c_int as libc::c_long,
                      last: 0x85b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x8d3 as libc::c_int as libc::c_long,
                      last: 0x8e1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x8e3 as libc::c_int as libc::c_long,
                      last: 0x903 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x93a as libc::c_int as libc::c_long,
                      last: 0x93c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x93e as libc::c_int as libc::c_long,
                      last: 0x94f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x951 as libc::c_int as libc::c_long,
                      last: 0x957 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x962 as libc::c_int as libc::c_long,
                      last: 0x963 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x981 as libc::c_int as libc::c_long,
                      last: 0x983 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x9bc as libc::c_int as libc::c_long,
                      last: 0x9bc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x9be as libc::c_int as libc::c_long,
                      last: 0x9c4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x9c7 as libc::c_int as libc::c_long,
                      last: 0x9c8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x9cb as libc::c_int as libc::c_long,
                      last: 0x9cd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x9d7 as libc::c_int as libc::c_long,
                      last: 0x9d7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x9e2 as libc::c_int as libc::c_long,
                      last: 0x9e3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x9fe as libc::c_int as libc::c_long,
                      last: 0x9fe as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa01 as libc::c_int as libc::c_long,
                      last: 0xa03 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa3c as libc::c_int as libc::c_long,
                      last: 0xa3c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa3e as libc::c_int as libc::c_long,
                      last: 0xa42 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa47 as libc::c_int as libc::c_long,
                      last: 0xa48 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa4b as libc::c_int as libc::c_long,
                      last: 0xa4d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa51 as libc::c_int as libc::c_long,
                      last: 0xa51 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa70 as libc::c_int as libc::c_long,
                      last: 0xa71 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa75 as libc::c_int as libc::c_long,
                      last: 0xa75 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa81 as libc::c_int as libc::c_long,
                      last: 0xa83 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xabc as libc::c_int as libc::c_long,
                      last: 0xabc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xabe as libc::c_int as libc::c_long,
                      last: 0xac5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xac7 as libc::c_int as libc::c_long,
                      last: 0xac9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xacb as libc::c_int as libc::c_long,
                      last: 0xacd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xae2 as libc::c_int as libc::c_long,
                      last: 0xae3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xafa as libc::c_int as libc::c_long,
                      last: 0xaff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb01 as libc::c_int as libc::c_long,
                      last: 0xb03 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb3c as libc::c_int as libc::c_long,
                      last: 0xb3c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb3e as libc::c_int as libc::c_long,
                      last: 0xb44 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb47 as libc::c_int as libc::c_long,
                      last: 0xb48 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb4b as libc::c_int as libc::c_long,
                      last: 0xb4d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb56 as libc::c_int as libc::c_long,
                      last: 0xb57 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb62 as libc::c_int as libc::c_long,
                      last: 0xb63 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb82 as libc::c_int as libc::c_long,
                      last: 0xb82 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xbbe as libc::c_int as libc::c_long,
                      last: 0xbc2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xbc6 as libc::c_int as libc::c_long,
                      last: 0xbc8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xbca as libc::c_int as libc::c_long,
                      last: 0xbcd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xbd7 as libc::c_int as libc::c_long,
                      last: 0xbd7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xc00 as libc::c_int as libc::c_long,
                      last: 0xc04 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xc3e as libc::c_int as libc::c_long,
                      last: 0xc44 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xc46 as libc::c_int as libc::c_long,
                      last: 0xc48 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xc4a as libc::c_int as libc::c_long,
                      last: 0xc4d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xc55 as libc::c_int as libc::c_long,
                      last: 0xc56 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xc62 as libc::c_int as libc::c_long,
                      last: 0xc63 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xc81 as libc::c_int as libc::c_long,
                      last: 0xc83 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xcbc as libc::c_int as libc::c_long,
                      last: 0xcbc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xcbe as libc::c_int as libc::c_long,
                      last: 0xcc4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xcc6 as libc::c_int as libc::c_long,
                      last: 0xcc8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xcca as libc::c_int as libc::c_long,
                      last: 0xccd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xcd5 as libc::c_int as libc::c_long,
                      last: 0xcd6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xce2 as libc::c_int as libc::c_long,
                      last: 0xce3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd00 as libc::c_int as libc::c_long,
                      last: 0xd03 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd3b as libc::c_int as libc::c_long,
                      last: 0xd3c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd3e as libc::c_int as libc::c_long,
                      last: 0xd44 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd46 as libc::c_int as libc::c_long,
                      last: 0xd48 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd4a as libc::c_int as libc::c_long,
                      last: 0xd4d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd57 as libc::c_int as libc::c_long,
                      last: 0xd57 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd62 as libc::c_int as libc::c_long,
                      last: 0xd63 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd82 as libc::c_int as libc::c_long,
                      last: 0xd83 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xdca as libc::c_int as libc::c_long,
                      last: 0xdca as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xdcf as libc::c_int as libc::c_long,
                      last: 0xdd4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xdd6 as libc::c_int as libc::c_long,
                      last: 0xdd6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xdd8 as libc::c_int as libc::c_long,
                      last: 0xddf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xdf2 as libc::c_int as libc::c_long,
                      last: 0xdf3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xe31 as libc::c_int as libc::c_long,
                      last: 0xe31 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xe34 as libc::c_int as libc::c_long,
                      last: 0xe3a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xe47 as libc::c_int as libc::c_long,
                      last: 0xe4e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xeb1 as libc::c_int as libc::c_long,
                      last: 0xeb1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xeb4 as libc::c_int as libc::c_long,
                      last: 0xebc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xec8 as libc::c_int as libc::c_long,
                      last: 0xecd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf18 as libc::c_int as libc::c_long,
                      last: 0xf19 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf35 as libc::c_int as libc::c_long,
                      last: 0xf35 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf37 as libc::c_int as libc::c_long,
                      last: 0xf37 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf39 as libc::c_int as libc::c_long,
                      last: 0xf39 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf3e as libc::c_int as libc::c_long,
                      last: 0xf3f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf71 as libc::c_int as libc::c_long,
                      last: 0xf84 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf86 as libc::c_int as libc::c_long,
                      last: 0xf87 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf8d as libc::c_int as libc::c_long,
                      last: 0xf97 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf99 as libc::c_int as libc::c_long,
                      last: 0xfbc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfc6 as libc::c_int as libc::c_long,
                      last: 0xfc6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x102b as libc::c_int as libc::c_long,
                      last: 0x103e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1056 as libc::c_int as libc::c_long,
                      last: 0x1059 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x105e as libc::c_int as libc::c_long,
                      last: 0x1060 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1062 as libc::c_int as libc::c_long,
                      last: 0x1064 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1067 as libc::c_int as libc::c_long,
                      last: 0x106d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1071 as libc::c_int as libc::c_long,
                      last: 0x1074 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1082 as libc::c_int as libc::c_long,
                      last: 0x108d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x108f as libc::c_int as libc::c_long,
                      last: 0x108f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x109a as libc::c_int as libc::c_long,
                      last: 0x109d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x135d as libc::c_int as libc::c_long,
                      last: 0x135f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1712 as libc::c_int as libc::c_long,
                      last: 0x1714 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1732 as libc::c_int as libc::c_long,
                      last: 0x1734 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1752 as libc::c_int as libc::c_long,
                      last: 0x1753 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1772 as libc::c_int as libc::c_long,
                      last: 0x1773 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x17b4 as libc::c_int as libc::c_long,
                      last: 0x17d3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x17dd as libc::c_int as libc::c_long,
                      last: 0x17dd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x180b as libc::c_int as libc::c_long,
                      last: 0x180d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1885 as libc::c_int as libc::c_long,
                      last: 0x1886 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x18a9 as libc::c_int as libc::c_long,
                      last: 0x18a9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1920 as libc::c_int as libc::c_long,
                      last: 0x192b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1930 as libc::c_int as libc::c_long,
                      last: 0x193b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1a17 as libc::c_int as libc::c_long,
                      last: 0x1a1b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1a55 as libc::c_int as libc::c_long,
                      last: 0x1a5e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1a60 as libc::c_int as libc::c_long,
                      last: 0x1a7c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1a7f as libc::c_int as libc::c_long,
                      last: 0x1a7f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1ab0 as libc::c_int as libc::c_long,
                      last: 0x1abe as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b00 as libc::c_int as libc::c_long,
                      last: 0x1b04 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b34 as libc::c_int as libc::c_long,
                      last: 0x1b44 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b6b as libc::c_int as libc::c_long,
                      last: 0x1b73 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b80 as libc::c_int as libc::c_long,
                      last: 0x1b82 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1ba1 as libc::c_int as libc::c_long,
                      last: 0x1bad as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1be6 as libc::c_int as libc::c_long,
                      last: 0x1bf3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1c24 as libc::c_int as libc::c_long,
                      last: 0x1c37 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1cd0 as libc::c_int as libc::c_long,
                      last: 0x1cd2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1cd4 as libc::c_int as libc::c_long,
                      last: 0x1ce8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1ced as libc::c_int as libc::c_long,
                      last: 0x1ced as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1cf4 as libc::c_int as libc::c_long,
                      last: 0x1cf4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1cf7 as libc::c_int as libc::c_long,
                      last: 0x1cf9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1dc0 as libc::c_int as libc::c_long,
                      last: 0x1df9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1dfb as libc::c_int as libc::c_long,
                      last: 0x1dff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x20d0 as libc::c_int as libc::c_long,
                      last: 0x20f0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2cef as libc::c_int as libc::c_long,
                      last: 0x2cf1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2d7f as libc::c_int as libc::c_long,
                      last: 0x2d7f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2de0 as libc::c_int as libc::c_long,
                      last: 0x2dff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x302a as libc::c_int as libc::c_long,
                      last: 0x302f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3099 as libc::c_int as libc::c_long,
                      last: 0x309a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa66f as libc::c_int as libc::c_long,
                      last: 0xa672 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa674 as libc::c_int as libc::c_long,
                      last: 0xa67d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa69e as libc::c_int as libc::c_long,
                      last: 0xa69f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa6f0 as libc::c_int as libc::c_long,
                      last: 0xa6f1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa802 as libc::c_int as libc::c_long,
                      last: 0xa802 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa806 as libc::c_int as libc::c_long,
                      last: 0xa806 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa80b as libc::c_int as libc::c_long,
                      last: 0xa80b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa823 as libc::c_int as libc::c_long,
                      last: 0xa827 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa880 as libc::c_int as libc::c_long,
                      last: 0xa881 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa8b4 as libc::c_int as libc::c_long,
                      last: 0xa8c5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa8e0 as libc::c_int as libc::c_long,
                      last: 0xa8f1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa8ff as libc::c_int as libc::c_long,
                      last: 0xa8ff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa926 as libc::c_int as libc::c_long,
                      last: 0xa92d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa947 as libc::c_int as libc::c_long,
                      last: 0xa953 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa980 as libc::c_int as libc::c_long,
                      last: 0xa983 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa9b3 as libc::c_int as libc::c_long,
                      last: 0xa9c0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa9e5 as libc::c_int as libc::c_long,
                      last: 0xa9e5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa29 as libc::c_int as libc::c_long,
                      last: 0xaa36 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa43 as libc::c_int as libc::c_long,
                      last: 0xaa43 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa4c as libc::c_int as libc::c_long,
                      last: 0xaa4d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa7b as libc::c_int as libc::c_long,
                      last: 0xaa7d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaab0 as libc::c_int as libc::c_long,
                      last: 0xaab0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaab2 as libc::c_int as libc::c_long,
                      last: 0xaab4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaab7 as libc::c_int as libc::c_long,
                      last: 0xaab8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaabe as libc::c_int as libc::c_long,
                      last: 0xaabf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaac1 as libc::c_int as libc::c_long,
                      last: 0xaac1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaaeb as libc::c_int as libc::c_long,
                      last: 0xaaef as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaaf5 as libc::c_int as libc::c_long,
                      last: 0xaaf6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xabe3 as libc::c_int as libc::c_long,
                      last: 0xabea as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xabec as libc::c_int as libc::c_long,
                      last: 0xabed as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfb1e as libc::c_int as libc::c_long,
                      last: 0xfb1e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe00 as libc::c_int as libc::c_long,
                      last: 0xfe0f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe20 as libc::c_int as libc::c_long,
                      last: 0xfe2f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x101fd as libc::c_int as libc::c_long,
                      last: 0x101fd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x102e0 as libc::c_int as libc::c_long,
                      last: 0x102e0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x10376 as libc::c_int as libc::c_long,
                      last: 0x1037a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a01 as libc::c_int as libc::c_long,
                      last: 0x10a03 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a05 as libc::c_int as libc::c_long,
                      last: 0x10a06 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a0c as libc::c_int as libc::c_long,
                      last: 0x10a0f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a38 as libc::c_int as libc::c_long,
                      last: 0x10a3a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a3f as libc::c_int as libc::c_long,
                      last: 0x10a3f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x10ae5 as libc::c_int as libc::c_long,
                      last: 0x10ae6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x10d24 as libc::c_int as libc::c_long,
                      last: 0x10d27 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x10f46 as libc::c_int as libc::c_long,
                      last: 0x10f50 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11000 as libc::c_int as libc::c_long,
                      last: 0x11002 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11038 as libc::c_int as libc::c_long,
                      last: 0x11046 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1107f as libc::c_int as libc::c_long,
                      last: 0x11082 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x110b0 as libc::c_int as libc::c_long,
                      last: 0x110ba as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11100 as libc::c_int as libc::c_long,
                      last: 0x11102 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11127 as libc::c_int as libc::c_long,
                      last: 0x11134 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11145 as libc::c_int as libc::c_long,
                      last: 0x11146 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11173 as libc::c_int as libc::c_long,
                      last: 0x11173 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11180 as libc::c_int as libc::c_long,
                      last: 0x11182 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x111b3 as libc::c_int as libc::c_long,
                      last: 0x111c0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x111c9 as libc::c_int as libc::c_long,
                      last: 0x111cc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1122c as libc::c_int as libc::c_long,
                      last: 0x11237 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1123e as libc::c_int as libc::c_long,
                      last: 0x1123e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x112df as libc::c_int as libc::c_long,
                      last: 0x112ea as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11300 as libc::c_int as libc::c_long,
                      last: 0x11303 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1133b as libc::c_int as libc::c_long,
                      last: 0x1133c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1133e as libc::c_int as libc::c_long,
                      last: 0x11344 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11347 as libc::c_int as libc::c_long,
                      last: 0x11348 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1134b as libc::c_int as libc::c_long,
                      last: 0x1134d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11357 as libc::c_int as libc::c_long,
                      last: 0x11357 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11362 as libc::c_int as libc::c_long,
                      last: 0x11363 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11366 as libc::c_int as libc::c_long,
                      last: 0x1136c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11370 as libc::c_int as libc::c_long,
                      last: 0x11374 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11435 as libc::c_int as libc::c_long,
                      last: 0x11446 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1145e as libc::c_int as libc::c_long,
                      last: 0x1145e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x114b0 as libc::c_int as libc::c_long,
                      last: 0x114c3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x115af as libc::c_int as libc::c_long,
                      last: 0x115b5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x115b8 as libc::c_int as libc::c_long,
                      last: 0x115c0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x115dc as libc::c_int as libc::c_long,
                      last: 0x115dd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11630 as libc::c_int as libc::c_long,
                      last: 0x11640 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x116ab as libc::c_int as libc::c_long,
                      last: 0x116b7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1171d as libc::c_int as libc::c_long,
                      last: 0x1172b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1182c as libc::c_int as libc::c_long,
                      last: 0x1183a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x119d1 as libc::c_int as libc::c_long,
                      last: 0x119d7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x119da as libc::c_int as libc::c_long,
                      last: 0x119e0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x119e4 as libc::c_int as libc::c_long,
                      last: 0x119e4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a01 as libc::c_int as libc::c_long,
                      last: 0x11a0a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a33 as libc::c_int as libc::c_long,
                      last: 0x11a39 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a3b as libc::c_int as libc::c_long,
                      last: 0x11a3e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a47 as libc::c_int as libc::c_long,
                      last: 0x11a47 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a51 as libc::c_int as libc::c_long,
                      last: 0x11a5b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a8a as libc::c_int as libc::c_long,
                      last: 0x11a99 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11c2f as libc::c_int as libc::c_long,
                      last: 0x11c36 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11c38 as libc::c_int as libc::c_long,
                      last: 0x11c3f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11c92 as libc::c_int as libc::c_long,
                      last: 0x11ca7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11ca9 as libc::c_int as libc::c_long,
                      last: 0x11cb6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d31 as libc::c_int as libc::c_long,
                      last: 0x11d36 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d3a as libc::c_int as libc::c_long,
                      last: 0x11d3a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d3c as libc::c_int as libc::c_long,
                      last: 0x11d3d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d3f as libc::c_int as libc::c_long,
                      last: 0x11d45 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d47 as libc::c_int as libc::c_long,
                      last: 0x11d47 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d8a as libc::c_int as libc::c_long,
                      last: 0x11d8e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d90 as libc::c_int as libc::c_long,
                      last: 0x11d91 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d93 as libc::c_int as libc::c_long,
                      last: 0x11d97 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11ef3 as libc::c_int as libc::c_long,
                      last: 0x11ef6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x16af0 as libc::c_int as libc::c_long,
                      last: 0x16af4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x16b30 as libc::c_int as libc::c_long,
                      last: 0x16b36 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x16f4f as libc::c_int as libc::c_long,
                      last: 0x16f4f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x16f51 as libc::c_int as libc::c_long,
                      last: 0x16f87 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x16f8f as libc::c_int as libc::c_long,
                      last: 0x16f92 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1bc9d as libc::c_int as libc::c_long,
                      last: 0x1bc9e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d165 as libc::c_int as libc::c_long,
                      last: 0x1d169 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d16d as libc::c_int as libc::c_long,
                      last: 0x1d172 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d17b as libc::c_int as libc::c_long,
                      last: 0x1d182 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d185 as libc::c_int as libc::c_long,
                      last: 0x1d18b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d1aa as libc::c_int as libc::c_long,
                      last: 0x1d1ad as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d242 as libc::c_int as libc::c_long,
                      last: 0x1d244 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da00 as libc::c_int as libc::c_long,
                      last: 0x1da36 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da3b as libc::c_int as libc::c_long,
                      last: 0x1da6c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da75 as libc::c_int as libc::c_long,
                      last: 0x1da75 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da84 as libc::c_int as libc::c_long,
                      last: 0x1da84 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da9b as libc::c_int as libc::c_long,
                      last: 0x1da9f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1daa1 as libc::c_int as libc::c_long,
                      last: 0x1daaf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e000 as libc::c_int as libc::c_long,
                      last: 0x1e006 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e008 as libc::c_int as libc::c_long,
                      last: 0x1e018 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e01b as libc::c_int as libc::c_long,
                      last: 0x1e021 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e023 as libc::c_int as libc::c_long,
                      last: 0x1e024 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e026 as libc::c_int as libc::c_long,
                      last: 0x1e02a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e130 as libc::c_int as libc::c_long,
                      last: 0x1e136 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e2ec as libc::c_int as libc::c_long,
                      last: 0x1e2ef as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e8d0 as libc::c_int as libc::c_long,
                      last: 0x1e8d6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e944 as libc::c_int as libc::c_long,
                      last: 0x1e94a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xe0100 as libc::c_int as libc::c_long,
                      last: 0xe01ef as libc::c_int as libc::c_long,};
         init
     }];
pub static mut foldCase: [convertStruct; 192] =
    [{
         let mut init =
             convertStruct{rangeStart: 0x41 as libc::c_int,
                           rangeEnd: 0x5a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xb5 as libc::c_int,
                           rangeEnd: 0xb5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 775 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xc0 as libc::c_int,
                           rangeEnd: 0xd6 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xd8 as libc::c_int,
                           rangeEnd: 0xde as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x100 as libc::c_int,
                           rangeEnd: 0x12e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x132 as libc::c_int,
                           rangeEnd: 0x136 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x139 as libc::c_int,
                           rangeEnd: 0x147 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x14a as libc::c_int,
                           rangeEnd: 0x176 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x178 as libc::c_int,
                           rangeEnd: 0x178 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(121 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x179 as libc::c_int,
                           rangeEnd: 0x17d as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x17f as libc::c_int,
                           rangeEnd: 0x17f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(268 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x181 as libc::c_int,
                           rangeEnd: 0x181 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 210 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x182 as libc::c_int,
                           rangeEnd: 0x184 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x186 as libc::c_int,
                           rangeEnd: 0x186 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 206 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x187 as libc::c_int,
                           rangeEnd: 0x187 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x189 as libc::c_int,
                           rangeEnd: 0x18a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 205 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18b as libc::c_int,
                           rangeEnd: 0x18b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18e as libc::c_int,
                           rangeEnd: 0x18e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 79 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18f as libc::c_int,
                           rangeEnd: 0x18f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 202 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x190 as libc::c_int,
                           rangeEnd: 0x190 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 203 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x191 as libc::c_int,
                           rangeEnd: 0x191 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x193 as libc::c_int,
                           rangeEnd: 0x193 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 205 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x194 as libc::c_int,
                           rangeEnd: 0x194 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 207 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x196 as libc::c_int,
                           rangeEnd: 0x196 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 211 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x197 as libc::c_int,
                           rangeEnd: 0x197 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 209 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x198 as libc::c_int,
                           rangeEnd: 0x198 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19c as libc::c_int,
                           rangeEnd: 0x19c as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 211 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19d as libc::c_int,
                           rangeEnd: 0x19d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 213 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19f as libc::c_int,
                           rangeEnd: 0x19f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 214 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a0 as libc::c_int,
                           rangeEnd: 0x1a4 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a6 as libc::c_int,
                           rangeEnd: 0x1a6 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 218 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a7 as libc::c_int,
                           rangeEnd: 0x1a7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a9 as libc::c_int,
                           rangeEnd: 0x1a9 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 218 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ac as libc::c_int,
                           rangeEnd: 0x1ac as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ae as libc::c_int,
                           rangeEnd: 0x1ae as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 218 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1af as libc::c_int,
                           rangeEnd: 0x1af as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b1 as libc::c_int,
                           rangeEnd: 0x1b2 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 217 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b3 as libc::c_int,
                           rangeEnd: 0x1b5 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b7 as libc::c_int,
                           rangeEnd: 0x1b7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 219 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b8 as libc::c_int,
                           rangeEnd: 0x1bc as libc::c_int,
                           step: 4 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c4 as libc::c_int,
                           rangeEnd: 0x1c4 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c5 as libc::c_int,
                           rangeEnd: 0x1c5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c7 as libc::c_int,
                           rangeEnd: 0x1c7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c8 as libc::c_int,
                           rangeEnd: 0x1c8 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ca as libc::c_int,
                           rangeEnd: 0x1ca as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cb as libc::c_int,
                           rangeEnd: 0x1db as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1de as libc::c_int,
                           rangeEnd: 0x1ee as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f1 as libc::c_int,
                           rangeEnd: 0x1f1 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 2 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f2 as libc::c_int,
                           rangeEnd: 0x1f4 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f6 as libc::c_int,
                           rangeEnd: 0x1f6 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(97 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f7 as libc::c_int,
                           rangeEnd: 0x1f7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(56 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f8 as libc::c_int,
                           rangeEnd: 0x21e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x220 as libc::c_int,
                           rangeEnd: 0x220 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(130 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x222 as libc::c_int,
                           rangeEnd: 0x232 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23a as libc::c_int,
                           rangeEnd: 0x23a as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10795 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23b as libc::c_int,
                           rangeEnd: 0x23b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23d as libc::c_int,
                           rangeEnd: 0x23d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(163 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23e as libc::c_int,
                           rangeEnd: 0x23e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 10792 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x241 as libc::c_int,
                           rangeEnd: 0x241 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x243 as libc::c_int,
                           rangeEnd: 0x243 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(195 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x244 as libc::c_int,
                           rangeEnd: 0x244 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 69 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x245 as libc::c_int,
                           rangeEnd: 0x245 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 71 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x246 as libc::c_int,
                           rangeEnd: 0x24e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x345 as libc::c_int,
                           rangeEnd: 0x345 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 116 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x370 as libc::c_int,
                           rangeEnd: 0x372 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x376 as libc::c_int,
                           rangeEnd: 0x376 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x37f as libc::c_int,
                           rangeEnd: 0x37f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 116 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x386 as libc::c_int,
                           rangeEnd: 0x386 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 38 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x388 as libc::c_int,
                           rangeEnd: 0x38a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 37 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x38c as libc::c_int,
                           rangeEnd: 0x38c as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 64 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x38e as libc::c_int,
                           rangeEnd: 0x38f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 63 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x391 as libc::c_int,
                           rangeEnd: 0x3a1 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3a3 as libc::c_int,
                           rangeEnd: 0x3ab as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3c2 as libc::c_int,
                           rangeEnd: 0x3c2 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3cf as libc::c_int,
                           rangeEnd: 0x3cf as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 8 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d0 as libc::c_int,
                           rangeEnd: 0x3d0 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(30 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d1 as libc::c_int,
                           rangeEnd: 0x3d1 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(25 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d5 as libc::c_int,
                           rangeEnd: 0x3d5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(15 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d6 as libc::c_int,
                           rangeEnd: 0x3d6 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(22 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d8 as libc::c_int,
                           rangeEnd: 0x3ee as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f0 as libc::c_int,
                           rangeEnd: 0x3f0 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(54 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f1 as libc::c_int,
                           rangeEnd: 0x3f1 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(48 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f4 as libc::c_int,
                           rangeEnd: 0x3f4 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(60 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f5 as libc::c_int,
                           rangeEnd: 0x3f5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(64 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f7 as libc::c_int,
                           rangeEnd: 0x3f7 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f9 as libc::c_int,
                           rangeEnd: 0x3f9 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3fa as libc::c_int,
                           rangeEnd: 0x3fa as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3fd as libc::c_int,
                           rangeEnd: 0x3ff as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(130 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x400 as libc::c_int,
                           rangeEnd: 0x40f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 80 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x410 as libc::c_int,
                           rangeEnd: 0x42f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x460 as libc::c_int,
                           rangeEnd: 0x480 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x48a as libc::c_int,
                           rangeEnd: 0x4be as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c0 as libc::c_int,
                           rangeEnd: 0x4c0 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 15 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c1 as libc::c_int,
                           rangeEnd: 0x4cd as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4d0 as libc::c_int,
                           rangeEnd: 0x52e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x531 as libc::c_int,
                           rangeEnd: 0x556 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 48 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10a0 as libc::c_int,
                           rangeEnd: 0x10c5 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 7264 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10c7 as libc::c_int,
                           rangeEnd: 0x10cd as libc::c_int,
                           step: 6 as libc::c_int,
                           offset: 7264 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13f8 as libc::c_int,
                           rangeEnd: 0x13fd as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c80 as libc::c_int,
                           rangeEnd: 0x1c80 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6222 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c81 as libc::c_int,
                           rangeEnd: 0x1c81 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6221 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c82 as libc::c_int,
                           rangeEnd: 0x1c82 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6212 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c83 as libc::c_int,
                           rangeEnd: 0x1c84 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(6210 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c85 as libc::c_int,
                           rangeEnd: 0x1c85 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6211 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c86 as libc::c_int,
                           rangeEnd: 0x1c86 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6204 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c87 as libc::c_int,
                           rangeEnd: 0x1c87 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(6180 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c88 as libc::c_int,
                           rangeEnd: 0x1c88 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 35267 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c90 as libc::c_int,
                           rangeEnd: 0x1cba as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(3008 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cbd as libc::c_int,
                           rangeEnd: 0x1cbf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(3008 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e00 as libc::c_int,
                           rangeEnd: 0x1e94 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e9b as libc::c_int,
                           rangeEnd: 0x1e9b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(58 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e9e as libc::c_int,
                           rangeEnd: 0x1e9e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7615 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ea0 as libc::c_int,
                           rangeEnd: 0x1efe as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f08 as libc::c_int,
                           rangeEnd: 0x1f0f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f18 as libc::c_int,
                           rangeEnd: 0x1f1d as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f28 as libc::c_int,
                           rangeEnd: 0x1f2f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f38 as libc::c_int,
                           rangeEnd: 0x1f3f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f48 as libc::c_int,
                           rangeEnd: 0x1f4d as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f59 as libc::c_int,
                           rangeEnd: 0x1f5f as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f68 as libc::c_int,
                           rangeEnd: 0x1f6f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f88 as libc::c_int,
                           rangeEnd: 0x1f8f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f98 as libc::c_int,
                           rangeEnd: 0x1f9f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fa8 as libc::c_int,
                           rangeEnd: 0x1faf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb8 as libc::c_int,
                           rangeEnd: 0x1fb9 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fba as libc::c_int,
                           rangeEnd: 0x1fbb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(74 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fbc as libc::c_int,
                           rangeEnd: 0x1fbc as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fbe as libc::c_int,
                           rangeEnd: 0x1fbe as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7173 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fc8 as libc::c_int,
                           rangeEnd: 0x1fcb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(86 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fcc as libc::c_int,
                           rangeEnd: 0x1fcc as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fd8 as libc::c_int,
                           rangeEnd: 0x1fd9 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fda as libc::c_int,
                           rangeEnd: 0x1fdb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(100 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fe8 as libc::c_int,
                           rangeEnd: 0x1fe9 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(8 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fea as libc::c_int,
                           rangeEnd: 0x1feb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(112 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fec as libc::c_int,
                           rangeEnd: 0x1fec as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ff8 as libc::c_int,
                           rangeEnd: 0x1ff9 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(128 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ffa as libc::c_int,
                           rangeEnd: 0x1ffb as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(126 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ffc as libc::c_int,
                           rangeEnd: 0x1ffc as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(9 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2126 as libc::c_int,
                           rangeEnd: 0x2126 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(7517 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x212a as libc::c_int,
                           rangeEnd: 0x212a as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(8383 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x212b as libc::c_int,
                           rangeEnd: 0x212b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(8262 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2132 as libc::c_int,
                           rangeEnd: 0x2132 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 28 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2160 as libc::c_int,
                           rangeEnd: 0x216f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 16 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2183 as libc::c_int,
                           rangeEnd: 0x2183 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x24b6 as libc::c_int,
                           rangeEnd: 0x24cf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 26 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c00 as libc::c_int,
                           rangeEnd: 0x2c2e as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 48 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c60 as libc::c_int,
                           rangeEnd: 0x2c60 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c62 as libc::c_int,
                           rangeEnd: 0x2c62 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10743 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c63 as libc::c_int,
                           rangeEnd: 0x2c63 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(3814 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c64 as libc::c_int,
                           rangeEnd: 0x2c64 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10727 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c67 as libc::c_int,
                           rangeEnd: 0x2c6b as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6d as libc::c_int,
                           rangeEnd: 0x2c6d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10780 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6e as libc::c_int,
                           rangeEnd: 0x2c6e as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10749 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6f as libc::c_int,
                           rangeEnd: 0x2c6f as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10783 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c70 as libc::c_int,
                           rangeEnd: 0x2c70 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(10782 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c72 as libc::c_int,
                           rangeEnd: 0x2c75 as libc::c_int,
                           step: 3 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c7e as libc::c_int,
                           rangeEnd: 0x2c7f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(10815 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c80 as libc::c_int,
                           rangeEnd: 0x2ce2 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2ceb as libc::c_int,
                           rangeEnd: 0x2ced as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2cf2 as libc::c_int,
                           rangeEnd: 0xa640 as libc::c_int,
                           step: 31054 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa642 as libc::c_int,
                           rangeEnd: 0xa66c as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa680 as libc::c_int,
                           rangeEnd: 0xa69a as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa722 as libc::c_int,
                           rangeEnd: 0xa72e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa732 as libc::c_int,
                           rangeEnd: 0xa76e as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa779 as libc::c_int,
                           rangeEnd: 0xa77b as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77d as libc::c_int,
                           rangeEnd: 0xa77d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(35332 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77e as libc::c_int,
                           rangeEnd: 0xa786 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78b as libc::c_int,
                           rangeEnd: 0xa78b as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78d as libc::c_int,
                           rangeEnd: 0xa78d as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42280 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa790 as libc::c_int,
                           rangeEnd: 0xa792 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa796 as libc::c_int,
                           rangeEnd: 0xa7a8 as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7aa as libc::c_int,
                           rangeEnd: 0xa7aa as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42308 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ab as libc::c_int,
                           rangeEnd: 0xa7ab as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42319 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ac as libc::c_int,
                           rangeEnd: 0xa7ac as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42315 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ad as libc::c_int,
                           rangeEnd: 0xa7ad as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42305 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ae as libc::c_int,
                           rangeEnd: 0xa7ae as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42308 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b0 as libc::c_int,
                           rangeEnd: 0xa7b0 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42258 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b1 as libc::c_int,
                           rangeEnd: 0xa7b1 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42282 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b2 as libc::c_int,
                           rangeEnd: 0xa7b2 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42261 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b3 as libc::c_int,
                           rangeEnd: 0xa7b3 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 928 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b4 as libc::c_int,
                           rangeEnd: 0xa7be as libc::c_int,
                           step: 2 as libc::c_int,
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c2 as libc::c_int,
                           rangeEnd: 0xa7c2 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: 1 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c4 as libc::c_int,
                           rangeEnd: 0xa7c4 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(48 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c5 as libc::c_int,
                           rangeEnd: 0xa7c5 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(42307 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c6 as libc::c_int,
                           rangeEnd: 0xa7c6 as libc::c_int,
                           step: -(1 as libc::c_int),
                           offset: -(35384 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xab70 as libc::c_int,
                           rangeEnd: 0xabbf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: -(38864 as libc::c_int),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xff21 as libc::c_int,
                           rangeEnd: 0xff3a as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10400 as libc::c_int,
                           rangeEnd: 0x10427 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 40 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x104b0 as libc::c_int,
                           rangeEnd: 0x104d3 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 40 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10c80 as libc::c_int,
                           rangeEnd: 0x10cb2 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 64 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x118a0 as libc::c_int,
                           rangeEnd: 0x118bf as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x16e40 as libc::c_int,
                           rangeEnd: 0x16e5f as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 32 as libc::c_int,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e900 as libc::c_int,
                           rangeEnd: 0x1e921 as libc::c_int,
                           step: 1 as libc::c_int,
                           offset: 34 as libc::c_int,};
         init
     }];
pub static mut doublewidth: [interval; 113] =
    [{
         let mut init =
             interval{first: 0x1100 as libc::c_int as libc::c_long,
                      last: 0x115f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x231a as libc::c_int as libc::c_long,
                      last: 0x231b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2329 as libc::c_int as libc::c_long,
                      last: 0x232a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x23e9 as libc::c_int as libc::c_long,
                      last: 0x23ec as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x23f0 as libc::c_int as libc::c_long,
                      last: 0x23f0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x23f3 as libc::c_int as libc::c_long,
                      last: 0x23f3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25fd as libc::c_int as libc::c_long,
                      last: 0x25fe as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2614 as libc::c_int as libc::c_long,
                      last: 0x2615 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2648 as libc::c_int as libc::c_long,
                      last: 0x2653 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x267f as libc::c_int as libc::c_long,
                      last: 0x267f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2693 as libc::c_int as libc::c_long,
                      last: 0x2693 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26a1 as libc::c_int as libc::c_long,
                      last: 0x26a1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26aa as libc::c_int as libc::c_long,
                      last: 0x26ab as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26bd as libc::c_int as libc::c_long,
                      last: 0x26be as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26c4 as libc::c_int as libc::c_long,
                      last: 0x26c5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26ce as libc::c_int as libc::c_long,
                      last: 0x26ce as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26d4 as libc::c_int as libc::c_long,
                      last: 0x26d4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26ea as libc::c_int as libc::c_long,
                      last: 0x26ea as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f2 as libc::c_int as libc::c_long,
                      last: 0x26f3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f5 as libc::c_int as libc::c_long,
                      last: 0x26f5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fa as libc::c_int as libc::c_long,
                      last: 0x26fa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fd as libc::c_int as libc::c_long,
                      last: 0x26fd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2705 as libc::c_int as libc::c_long,
                      last: 0x2705 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x270a as libc::c_int as libc::c_long,
                      last: 0x270b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2728 as libc::c_int as libc::c_long,
                      last: 0x2728 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x274c as libc::c_int as libc::c_long,
                      last: 0x274c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x274e as libc::c_int as libc::c_long,
                      last: 0x274e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2753 as libc::c_int as libc::c_long,
                      last: 0x2755 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2757 as libc::c_int as libc::c_long,
                      last: 0x2757 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2795 as libc::c_int as libc::c_long,
                      last: 0x2797 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x27b0 as libc::c_int as libc::c_long,
                      last: 0x27b0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x27bf as libc::c_int as libc::c_long,
                      last: 0x27bf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b1b as libc::c_int as libc::c_long,
                      last: 0x2b1c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b50 as libc::c_int as libc::c_long,
                      last: 0x2b50 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b55 as libc::c_int as libc::c_long,
                      last: 0x2b55 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2e80 as libc::c_int as libc::c_long,
                      last: 0x2e99 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2e9b as libc::c_int as libc::c_long,
                      last: 0x2ef3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2f00 as libc::c_int as libc::c_long,
                      last: 0x2fd5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2ff0 as libc::c_int as libc::c_long,
                      last: 0x2ffb as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3000 as libc::c_int as libc::c_long,
                      last: 0x303e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3041 as libc::c_int as libc::c_long,
                      last: 0x3096 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3099 as libc::c_int as libc::c_long,
                      last: 0x30ff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3105 as libc::c_int as libc::c_long,
                      last: 0x312f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3131 as libc::c_int as libc::c_long,
                      last: 0x318e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3190 as libc::c_int as libc::c_long,
                      last: 0x31ba as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x31c0 as libc::c_int as libc::c_long,
                      last: 0x31e3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x31f0 as libc::c_int as libc::c_long,
                      last: 0x321e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3220 as libc::c_int as libc::c_long,
                      last: 0x3247 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3250 as libc::c_int as libc::c_long,
                      last: 0x4dbf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x4e00 as libc::c_int as libc::c_long,
                      last: 0xa48c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa490 as libc::c_int as libc::c_long,
                      last: 0xa4c6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa960 as libc::c_int as libc::c_long,
                      last: 0xa97c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xac00 as libc::c_int as libc::c_long,
                      last: 0xd7a3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf900 as libc::c_int as libc::c_long,
                      last: 0xfaff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe10 as libc::c_int as libc::c_long,
                      last: 0xfe19 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe30 as libc::c_int as libc::c_long,
                      last: 0xfe52 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe54 as libc::c_int as libc::c_long,
                      last: 0xfe66 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe68 as libc::c_int as libc::c_long,
                      last: 0xfe6b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xff01 as libc::c_int as libc::c_long,
                      last: 0xff60 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xffe0 as libc::c_int as libc::c_long,
                      last: 0xffe6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x16fe0 as libc::c_int as libc::c_long,
                      last: 0x16fe3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x17000 as libc::c_int as libc::c_long,
                      last: 0x187f7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x18800 as libc::c_int as libc::c_long,
                      last: 0x18af2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b000 as libc::c_int as libc::c_long,
                      last: 0x1b11e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b150 as libc::c_int as libc::c_long,
                      last: 0x1b152 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b164 as libc::c_int as libc::c_long,
                      last: 0x1b167 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b170 as libc::c_int as libc::c_long,
                      last: 0x1b2fb as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f004 as libc::c_int as libc::c_long,
                      last: 0x1f004 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f0cf as libc::c_int as libc::c_long,
                      last: 0x1f0cf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f18e as libc::c_int as libc::c_long,
                      last: 0x1f18e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f191 as libc::c_int as libc::c_long,
                      last: 0x1f19a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f200 as libc::c_int as libc::c_long,
                      last: 0x1f202 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f210 as libc::c_int as libc::c_long,
                      last: 0x1f23b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f240 as libc::c_int as libc::c_long,
                      last: 0x1f248 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f250 as libc::c_int as libc::c_long,
                      last: 0x1f251 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f260 as libc::c_int as libc::c_long,
                      last: 0x1f265 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f300 as libc::c_int as libc::c_long,
                      last: 0x1f320 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f32d as libc::c_int as libc::c_long,
                      last: 0x1f335 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f337 as libc::c_int as libc::c_long,
                      last: 0x1f37c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f37e as libc::c_int as libc::c_long,
                      last: 0x1f393 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3a0 as libc::c_int as libc::c_long,
                      last: 0x1f3ca as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3cf as libc::c_int as libc::c_long,
                      last: 0x1f3d3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3e0 as libc::c_int as libc::c_long,
                      last: 0x1f3f0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f4 as libc::c_int as libc::c_long,
                      last: 0x1f3f4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f8 as libc::c_int as libc::c_long,
                      last: 0x1f43e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f440 as libc::c_int as libc::c_long,
                      last: 0x1f440 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f442 as libc::c_int as libc::c_long,
                      last: 0x1f4fc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f4ff as libc::c_int as libc::c_long,
                      last: 0x1f53d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f54b as libc::c_int as libc::c_long,
                      last: 0x1f54e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f550 as libc::c_int as libc::c_long,
                      last: 0x1f567 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f57a as libc::c_int as libc::c_long,
                      last: 0x1f57a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f595 as libc::c_int as libc::c_long,
                      last: 0x1f596 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a4 as libc::c_int as libc::c_long,
                      last: 0x1f5a4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5fb as libc::c_int as libc::c_long,
                      last: 0x1f64f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f680 as libc::c_int as libc::c_long,
                      last: 0x1f6c5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6cc as libc::c_int as libc::c_long,
                      last: 0x1f6cc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6d0 as libc::c_int as libc::c_long,
                      last: 0x1f6d2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6d5 as libc::c_int as libc::c_long,
                      last: 0x1f6d5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6eb as libc::c_int as libc::c_long,
                      last: 0x1f6ec as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f4 as libc::c_int as libc::c_long,
                      last: 0x1f6fa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f7e0 as libc::c_int as libc::c_long,
                      last: 0x1f7eb as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f90d as libc::c_int as libc::c_long,
                      last: 0x1f971 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f973 as libc::c_int as libc::c_long,
                      last: 0x1f976 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f97a as libc::c_int as libc::c_long,
                      last: 0x1f9a2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9a5 as libc::c_int as libc::c_long,
                      last: 0x1f9aa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9ae as libc::c_int as libc::c_long,
                      last: 0x1f9ca as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9cd as libc::c_int as libc::c_long,
                      last: 0x1f9ff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa70 as libc::c_int as libc::c_long,
                      last: 0x1fa73 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa78 as libc::c_int as libc::c_long,
                      last: 0x1fa7a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa80 as libc::c_int as libc::c_long,
                      last: 0x1fa82 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa90 as libc::c_int as libc::c_long,
                      last: 0x1fa95 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x20000 as libc::c_int as libc::c_long,
                      last: 0x2fffd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x30000 as libc::c_int as libc::c_long,
                      last: 0x3fffd as libc::c_int as libc::c_long,};
         init
     }];
pub static mut ambiguous: [interval; 179] =
    [{
         let mut init =
             interval{first: 0xa1 as libc::c_int as libc::c_long,
                      last: 0xa1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa4 as libc::c_int as libc::c_long,
                      last: 0xa4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa7 as libc::c_int as libc::c_long,
                      last: 0xa8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa as libc::c_int as libc::c_long,
                      last: 0xaa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xad as libc::c_int as libc::c_long,
                      last: 0xae as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb0 as libc::c_int as libc::c_long,
                      last: 0xb4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xb6 as libc::c_int as libc::c_long,
                      last: 0xba as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xbc as libc::c_int as libc::c_long,
                      last: 0xbf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xc6 as libc::c_int as libc::c_long,
                      last: 0xc6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd0 as libc::c_int as libc::c_long,
                      last: 0xd0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xd7 as libc::c_int as libc::c_long,
                      last: 0xd8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xde as libc::c_int as libc::c_long,
                      last: 0xe1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xe6 as libc::c_int as libc::c_long,
                      last: 0xe6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xe8 as libc::c_int as libc::c_long,
                      last: 0xea as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xec as libc::c_int as libc::c_long,
                      last: 0xed as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf0 as libc::c_int as libc::c_long,
                      last: 0xf0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf2 as libc::c_int as libc::c_long,
                      last: 0xf3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf7 as libc::c_int as libc::c_long,
                      last: 0xfa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfc as libc::c_int as libc::c_long,
                      last: 0xfc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe as libc::c_int as libc::c_long,
                      last: 0xfe as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x101 as libc::c_int as libc::c_long,
                      last: 0x101 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x111 as libc::c_int as libc::c_long,
                      last: 0x111 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x113 as libc::c_int as libc::c_long,
                      last: 0x113 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x11b as libc::c_int as libc::c_long,
                      last: 0x11b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x126 as libc::c_int as libc::c_long,
                      last: 0x127 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x12b as libc::c_int as libc::c_long,
                      last: 0x12b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x131 as libc::c_int as libc::c_long,
                      last: 0x133 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x138 as libc::c_int as libc::c_long,
                      last: 0x138 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x13f as libc::c_int as libc::c_long,
                      last: 0x142 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x144 as libc::c_int as libc::c_long,
                      last: 0x144 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x148 as libc::c_int as libc::c_long,
                      last: 0x14b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x14d as libc::c_int as libc::c_long,
                      last: 0x14d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x152 as libc::c_int as libc::c_long,
                      last: 0x153 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x166 as libc::c_int as libc::c_long,
                      last: 0x167 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x16b as libc::c_int as libc::c_long,
                      last: 0x16b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1ce as libc::c_int as libc::c_long,
                      last: 0x1ce as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d0 as libc::c_int as libc::c_long,
                      last: 0x1d0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d2 as libc::c_int as libc::c_long,
                      last: 0x1d2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d4 as libc::c_int as libc::c_long,
                      last: 0x1d4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d6 as libc::c_int as libc::c_long,
                      last: 0x1d6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d8 as libc::c_int as libc::c_long,
                      last: 0x1d8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da as libc::c_int as libc::c_long,
                      last: 0x1da as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1dc as libc::c_int as libc::c_long,
                      last: 0x1dc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x251 as libc::c_int as libc::c_long,
                      last: 0x251 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x261 as libc::c_int as libc::c_long,
                      last: 0x261 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2c4 as libc::c_int as libc::c_long,
                      last: 0x2c4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2c7 as libc::c_int as libc::c_long,
                      last: 0x2c7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2c9 as libc::c_int as libc::c_long,
                      last: 0x2cb as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2cd as libc::c_int as libc::c_long,
                      last: 0x2cd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2d0 as libc::c_int as libc::c_long,
                      last: 0x2d0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2d8 as libc::c_int as libc::c_long,
                      last: 0x2db as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2dd as libc::c_int as libc::c_long,
                      last: 0x2dd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2df as libc::c_int as libc::c_long,
                      last: 0x2df as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x300 as libc::c_int as libc::c_long,
                      last: 0x36f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x391 as libc::c_int as libc::c_long,
                      last: 0x3a1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3a3 as libc::c_int as libc::c_long,
                      last: 0x3a9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3b1 as libc::c_int as libc::c_long,
                      last: 0x3c1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3c3 as libc::c_int as libc::c_long,
                      last: 0x3c9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x401 as libc::c_int as libc::c_long,
                      last: 0x401 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x410 as libc::c_int as libc::c_long,
                      last: 0x44f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x451 as libc::c_int as libc::c_long,
                      last: 0x451 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2010 as libc::c_int as libc::c_long,
                      last: 0x2010 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2013 as libc::c_int as libc::c_long,
                      last: 0x2016 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2018 as libc::c_int as libc::c_long,
                      last: 0x2019 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x201c as libc::c_int as libc::c_long,
                      last: 0x201d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2020 as libc::c_int as libc::c_long,
                      last: 0x2022 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2024 as libc::c_int as libc::c_long,
                      last: 0x2027 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2030 as libc::c_int as libc::c_long,
                      last: 0x2030 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2032 as libc::c_int as libc::c_long,
                      last: 0x2033 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2035 as libc::c_int as libc::c_long,
                      last: 0x2035 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x203b as libc::c_int as libc::c_long,
                      last: 0x203b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x203e as libc::c_int as libc::c_long,
                      last: 0x203e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2074 as libc::c_int as libc::c_long,
                      last: 0x2074 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x207f as libc::c_int as libc::c_long,
                      last: 0x207f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2081 as libc::c_int as libc::c_long,
                      last: 0x2084 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x20ac as libc::c_int as libc::c_long,
                      last: 0x20ac as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2103 as libc::c_int as libc::c_long,
                      last: 0x2103 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2105 as libc::c_int as libc::c_long,
                      last: 0x2105 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2109 as libc::c_int as libc::c_long,
                      last: 0x2109 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2113 as libc::c_int as libc::c_long,
                      last: 0x2113 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2116 as libc::c_int as libc::c_long,
                      last: 0x2116 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2121 as libc::c_int as libc::c_long,
                      last: 0x2122 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2126 as libc::c_int as libc::c_long,
                      last: 0x2126 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x212b as libc::c_int as libc::c_long,
                      last: 0x212b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2153 as libc::c_int as libc::c_long,
                      last: 0x2154 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x215b as libc::c_int as libc::c_long,
                      last: 0x215e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2160 as libc::c_int as libc::c_long,
                      last: 0x216b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2170 as libc::c_int as libc::c_long,
                      last: 0x2179 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2189 as libc::c_int as libc::c_long,
                      last: 0x2189 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2190 as libc::c_int as libc::c_long,
                      last: 0x2199 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x21b8 as libc::c_int as libc::c_long,
                      last: 0x21b9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x21d2 as libc::c_int as libc::c_long,
                      last: 0x21d2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x21d4 as libc::c_int as libc::c_long,
                      last: 0x21d4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x21e7 as libc::c_int as libc::c_long,
                      last: 0x21e7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2200 as libc::c_int as libc::c_long,
                      last: 0x2200 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2202 as libc::c_int as libc::c_long,
                      last: 0x2203 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2207 as libc::c_int as libc::c_long,
                      last: 0x2208 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x220b as libc::c_int as libc::c_long,
                      last: 0x220b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x220f as libc::c_int as libc::c_long,
                      last: 0x220f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2211 as libc::c_int as libc::c_long,
                      last: 0x2211 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2215 as libc::c_int as libc::c_long,
                      last: 0x2215 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x221a as libc::c_int as libc::c_long,
                      last: 0x221a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x221d as libc::c_int as libc::c_long,
                      last: 0x2220 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2223 as libc::c_int as libc::c_long,
                      last: 0x2223 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2225 as libc::c_int as libc::c_long,
                      last: 0x2225 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2227 as libc::c_int as libc::c_long,
                      last: 0x222c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x222e as libc::c_int as libc::c_long,
                      last: 0x222e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2234 as libc::c_int as libc::c_long,
                      last: 0x2237 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x223c as libc::c_int as libc::c_long,
                      last: 0x223d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2248 as libc::c_int as libc::c_long,
                      last: 0x2248 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x224c as libc::c_int as libc::c_long,
                      last: 0x224c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2252 as libc::c_int as libc::c_long,
                      last: 0x2252 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2260 as libc::c_int as libc::c_long,
                      last: 0x2261 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2264 as libc::c_int as libc::c_long,
                      last: 0x2267 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x226a as libc::c_int as libc::c_long,
                      last: 0x226b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x226e as libc::c_int as libc::c_long,
                      last: 0x226f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2282 as libc::c_int as libc::c_long,
                      last: 0x2283 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2286 as libc::c_int as libc::c_long,
                      last: 0x2287 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2295 as libc::c_int as libc::c_long,
                      last: 0x2295 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2299 as libc::c_int as libc::c_long,
                      last: 0x2299 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x22a5 as libc::c_int as libc::c_long,
                      last: 0x22a5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x22bf as libc::c_int as libc::c_long,
                      last: 0x22bf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2312 as libc::c_int as libc::c_long,
                      last: 0x2312 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2460 as libc::c_int as libc::c_long,
                      last: 0x24e9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x24eb as libc::c_int as libc::c_long,
                      last: 0x254b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2550 as libc::c_int as libc::c_long,
                      last: 0x2573 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2580 as libc::c_int as libc::c_long,
                      last: 0x258f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2592 as libc::c_int as libc::c_long,
                      last: 0x2595 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25a0 as libc::c_int as libc::c_long,
                      last: 0x25a1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25a3 as libc::c_int as libc::c_long,
                      last: 0x25a9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25b2 as libc::c_int as libc::c_long,
                      last: 0x25b3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25b6 as libc::c_int as libc::c_long,
                      last: 0x25b7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25bc as libc::c_int as libc::c_long,
                      last: 0x25bd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25c0 as libc::c_int as libc::c_long,
                      last: 0x25c1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25c6 as libc::c_int as libc::c_long,
                      last: 0x25c8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25cb as libc::c_int as libc::c_long,
                      last: 0x25cb as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25ce as libc::c_int as libc::c_long,
                      last: 0x25d1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25e2 as libc::c_int as libc::c_long,
                      last: 0x25e5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25ef as libc::c_int as libc::c_long,
                      last: 0x25ef as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2605 as libc::c_int as libc::c_long,
                      last: 0x2606 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2609 as libc::c_int as libc::c_long,
                      last: 0x2609 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x260e as libc::c_int as libc::c_long,
                      last: 0x260f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x261c as libc::c_int as libc::c_long,
                      last: 0x261c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x261e as libc::c_int as libc::c_long,
                      last: 0x261e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2640 as libc::c_int as libc::c_long,
                      last: 0x2640 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2642 as libc::c_int as libc::c_long,
                      last: 0x2642 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2660 as libc::c_int as libc::c_long,
                      last: 0x2661 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2663 as libc::c_int as libc::c_long,
                      last: 0x2665 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2667 as libc::c_int as libc::c_long,
                      last: 0x266a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x266c as libc::c_int as libc::c_long,
                      last: 0x266d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x266f as libc::c_int as libc::c_long,
                      last: 0x266f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x269e as libc::c_int as libc::c_long,
                      last: 0x269f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26bf as libc::c_int as libc::c_long,
                      last: 0x26bf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26c6 as libc::c_int as libc::c_long,
                      last: 0x26cd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26cf as libc::c_int as libc::c_long,
                      last: 0x26d3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26d5 as libc::c_int as libc::c_long,
                      last: 0x26e1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26e3 as libc::c_int as libc::c_long,
                      last: 0x26e3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26e8 as libc::c_int as libc::c_long,
                      last: 0x26e9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26eb as libc::c_int as libc::c_long,
                      last: 0x26f1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f4 as libc::c_int as libc::c_long,
                      last: 0x26f4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f6 as libc::c_int as libc::c_long,
                      last: 0x26f9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fb as libc::c_int as libc::c_long,
                      last: 0x26fc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fe as libc::c_int as libc::c_long,
                      last: 0x26ff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x273d as libc::c_int as libc::c_long,
                      last: 0x273d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2776 as libc::c_int as libc::c_long,
                      last: 0x277f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b56 as libc::c_int as libc::c_long,
                      last: 0x2b59 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3248 as libc::c_int as libc::c_long,
                      last: 0x324f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xe000 as libc::c_int as libc::c_long,
                      last: 0xf8ff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe00 as libc::c_int as libc::c_long,
                      last: 0xfe0f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xfffd as libc::c_int as libc::c_long,
                      last: 0xfffd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f100 as libc::c_int as libc::c_long,
                      last: 0x1f10a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f110 as libc::c_int as libc::c_long,
                      last: 0x1f12d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f130 as libc::c_int as libc::c_long,
                      last: 0x1f169 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f170 as libc::c_int as libc::c_long,
                      last: 0x1f18d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f18f as libc::c_int as libc::c_long,
                      last: 0x1f190 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f19b as libc::c_int as libc::c_long,
                      last: 0x1f1ac as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xe0100 as libc::c_int as libc::c_long,
                      last: 0xe01ef as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xf0000 as libc::c_int as libc::c_long,
                      last: 0xffffd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x100000 as libc::c_int as libc::c_long,
                      last: 0x10fffd as libc::c_int as libc::c_long,};
         init
     }];
pub static mut emoji_all: [interval; 151] =
    [{
         let mut init =
             interval{first: 0x23 as libc::c_int as libc::c_long,
                      last: 0x23 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2a as libc::c_int as libc::c_long,
                      last: 0x2a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x30 as libc::c_int as libc::c_long,
                      last: 0x39 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xa9 as libc::c_int as libc::c_long,
                      last: 0xa9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0xae as libc::c_int as libc::c_long,
                      last: 0xae as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x203c as libc::c_int as libc::c_long,
                      last: 0x203c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2049 as libc::c_int as libc::c_long,
                      last: 0x2049 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2122 as libc::c_int as libc::c_long,
                      last: 0x2122 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2139 as libc::c_int as libc::c_long,
                      last: 0x2139 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2194 as libc::c_int as libc::c_long,
                      last: 0x2199 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x21a9 as libc::c_int as libc::c_long,
                      last: 0x21aa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x231a as libc::c_int as libc::c_long,
                      last: 0x231b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2328 as libc::c_int as libc::c_long,
                      last: 0x2328 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x23cf as libc::c_int as libc::c_long,
                      last: 0x23cf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x23e9 as libc::c_int as libc::c_long,
                      last: 0x23f3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x23f8 as libc::c_int as libc::c_long,
                      last: 0x23fa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x24c2 as libc::c_int as libc::c_long,
                      last: 0x24c2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25aa as libc::c_int as libc::c_long,
                      last: 0x25ab as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25b6 as libc::c_int as libc::c_long,
                      last: 0x25b6 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25c0 as libc::c_int as libc::c_long,
                      last: 0x25c0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x25fb as libc::c_int as libc::c_long,
                      last: 0x25fe as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2600 as libc::c_int as libc::c_long,
                      last: 0x2604 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x260e as libc::c_int as libc::c_long,
                      last: 0x260e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2611 as libc::c_int as libc::c_long,
                      last: 0x2611 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2614 as libc::c_int as libc::c_long,
                      last: 0x2615 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2618 as libc::c_int as libc::c_long,
                      last: 0x2618 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x261d as libc::c_int as libc::c_long,
                      last: 0x261d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2620 as libc::c_int as libc::c_long,
                      last: 0x2620 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2622 as libc::c_int as libc::c_long,
                      last: 0x2623 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2626 as libc::c_int as libc::c_long,
                      last: 0x2626 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x262a as libc::c_int as libc::c_long,
                      last: 0x262a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x262e as libc::c_int as libc::c_long,
                      last: 0x262f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2638 as libc::c_int as libc::c_long,
                      last: 0x263a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2640 as libc::c_int as libc::c_long,
                      last: 0x2640 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2642 as libc::c_int as libc::c_long,
                      last: 0x2642 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2648 as libc::c_int as libc::c_long,
                      last: 0x2653 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x265f as libc::c_int as libc::c_long,
                      last: 0x2660 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2663 as libc::c_int as libc::c_long,
                      last: 0x2663 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2665 as libc::c_int as libc::c_long,
                      last: 0x2666 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2668 as libc::c_int as libc::c_long,
                      last: 0x2668 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x267b as libc::c_int as libc::c_long,
                      last: 0x267b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x267e as libc::c_int as libc::c_long,
                      last: 0x267f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2692 as libc::c_int as libc::c_long,
                      last: 0x2697 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2699 as libc::c_int as libc::c_long,
                      last: 0x2699 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x269b as libc::c_int as libc::c_long,
                      last: 0x269c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26a0 as libc::c_int as libc::c_long,
                      last: 0x26a1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26aa as libc::c_int as libc::c_long,
                      last: 0x26ab as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26b0 as libc::c_int as libc::c_long,
                      last: 0x26b1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26bd as libc::c_int as libc::c_long,
                      last: 0x26be as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26c4 as libc::c_int as libc::c_long,
                      last: 0x26c5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26c8 as libc::c_int as libc::c_long,
                      last: 0x26c8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26ce as libc::c_int as libc::c_long,
                      last: 0x26cf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26d1 as libc::c_int as libc::c_long,
                      last: 0x26d1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26d3 as libc::c_int as libc::c_long,
                      last: 0x26d4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26e9 as libc::c_int as libc::c_long,
                      last: 0x26ea as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f0 as libc::c_int as libc::c_long,
                      last: 0x26f5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f7 as libc::c_int as libc::c_long,
                      last: 0x26fa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fd as libc::c_int as libc::c_long,
                      last: 0x26fd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2702 as libc::c_int as libc::c_long,
                      last: 0x2702 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2705 as libc::c_int as libc::c_long,
                      last: 0x2705 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2708 as libc::c_int as libc::c_long,
                      last: 0x270d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x270f as libc::c_int as libc::c_long,
                      last: 0x270f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2712 as libc::c_int as libc::c_long,
                      last: 0x2712 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2714 as libc::c_int as libc::c_long,
                      last: 0x2714 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2716 as libc::c_int as libc::c_long,
                      last: 0x2716 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x271d as libc::c_int as libc::c_long,
                      last: 0x271d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2721 as libc::c_int as libc::c_long,
                      last: 0x2721 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2728 as libc::c_int as libc::c_long,
                      last: 0x2728 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2733 as libc::c_int as libc::c_long,
                      last: 0x2734 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2744 as libc::c_int as libc::c_long,
                      last: 0x2744 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2747 as libc::c_int as libc::c_long,
                      last: 0x2747 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x274c as libc::c_int as libc::c_long,
                      last: 0x274c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x274e as libc::c_int as libc::c_long,
                      last: 0x274e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2753 as libc::c_int as libc::c_long,
                      last: 0x2755 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2757 as libc::c_int as libc::c_long,
                      last: 0x2757 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2763 as libc::c_int as libc::c_long,
                      last: 0x2764 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2795 as libc::c_int as libc::c_long,
                      last: 0x2797 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x27a1 as libc::c_int as libc::c_long,
                      last: 0x27a1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x27b0 as libc::c_int as libc::c_long,
                      last: 0x27b0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x27bf as libc::c_int as libc::c_long,
                      last: 0x27bf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2934 as libc::c_int as libc::c_long,
                      last: 0x2935 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b05 as libc::c_int as libc::c_long,
                      last: 0x2b07 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b1b as libc::c_int as libc::c_long,
                      last: 0x2b1c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b50 as libc::c_int as libc::c_long,
                      last: 0x2b50 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b55 as libc::c_int as libc::c_long,
                      last: 0x2b55 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3030 as libc::c_int as libc::c_long,
                      last: 0x3030 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x303d as libc::c_int as libc::c_long,
                      last: 0x303d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3297 as libc::c_int as libc::c_long,
                      last: 0x3297 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x3299 as libc::c_int as libc::c_long,
                      last: 0x3299 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f004 as libc::c_int as libc::c_long,
                      last: 0x1f004 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f0cf as libc::c_int as libc::c_long,
                      last: 0x1f0cf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f170 as libc::c_int as libc::c_long,
                      last: 0x1f171 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f17e as libc::c_int as libc::c_long,
                      last: 0x1f17f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f18e as libc::c_int as libc::c_long,
                      last: 0x1f18e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f191 as libc::c_int as libc::c_long,
                      last: 0x1f19a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f1e6 as libc::c_int as libc::c_long,
                      last: 0x1f1ff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f201 as libc::c_int as libc::c_long,
                      last: 0x1f202 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f21a as libc::c_int as libc::c_long,
                      last: 0x1f21a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f22f as libc::c_int as libc::c_long,
                      last: 0x1f22f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f232 as libc::c_int as libc::c_long,
                      last: 0x1f23a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f250 as libc::c_int as libc::c_long,
                      last: 0x1f251 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f300 as libc::c_int as libc::c_long,
                      last: 0x1f321 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f324 as libc::c_int as libc::c_long,
                      last: 0x1f393 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f396 as libc::c_int as libc::c_long,
                      last: 0x1f397 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f399 as libc::c_int as libc::c_long,
                      last: 0x1f39b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f39e as libc::c_int as libc::c_long,
                      last: 0x1f3f0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f3 as libc::c_int as libc::c_long,
                      last: 0x1f3f5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f7 as libc::c_int as libc::c_long,
                      last: 0x1f4fd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f4ff as libc::c_int as libc::c_long,
                      last: 0x1f53d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f549 as libc::c_int as libc::c_long,
                      last: 0x1f54e as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f550 as libc::c_int as libc::c_long,
                      last: 0x1f567 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f56f as libc::c_int as libc::c_long,
                      last: 0x1f570 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f573 as libc::c_int as libc::c_long,
                      last: 0x1f57a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f587 as libc::c_int as libc::c_long,
                      last: 0x1f587 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f58a as libc::c_int as libc::c_long,
                      last: 0x1f58d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f590 as libc::c_int as libc::c_long,
                      last: 0x1f590 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f595 as libc::c_int as libc::c_long,
                      last: 0x1f596 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a4 as libc::c_int as libc::c_long,
                      last: 0x1f5a5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a8 as libc::c_int as libc::c_long,
                      last: 0x1f5a8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5b1 as libc::c_int as libc::c_long,
                      last: 0x1f5b2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5bc as libc::c_int as libc::c_long,
                      last: 0x1f5bc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5c2 as libc::c_int as libc::c_long,
                      last: 0x1f5c4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5d1 as libc::c_int as libc::c_long,
                      last: 0x1f5d3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5dc as libc::c_int as libc::c_long,
                      last: 0x1f5de as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e1 as libc::c_int as libc::c_long,
                      last: 0x1f5e1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e3 as libc::c_int as libc::c_long,
                      last: 0x1f5e3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e8 as libc::c_int as libc::c_long,
                      last: 0x1f5e8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5ef as libc::c_int as libc::c_long,
                      last: 0x1f5ef as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5f3 as libc::c_int as libc::c_long,
                      last: 0x1f5f3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5fa as libc::c_int as libc::c_long,
                      last: 0x1f64f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f680 as libc::c_int as libc::c_long,
                      last: 0x1f6c5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6cb as libc::c_int as libc::c_long,
                      last: 0x1f6d2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6d5 as libc::c_int as libc::c_long,
                      last: 0x1f6d5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6e0 as libc::c_int as libc::c_long,
                      last: 0x1f6e5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6e9 as libc::c_int as libc::c_long,
                      last: 0x1f6e9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6eb as libc::c_int as libc::c_long,
                      last: 0x1f6ec as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f0 as libc::c_int as libc::c_long,
                      last: 0x1f6f0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f3 as libc::c_int as libc::c_long,
                      last: 0x1f6fa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f7e0 as libc::c_int as libc::c_long,
                      last: 0x1f7eb as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f90d as libc::c_int as libc::c_long,
                      last: 0x1f93a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f93c as libc::c_int as libc::c_long,
                      last: 0x1f945 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f947 as libc::c_int as libc::c_long,
                      last: 0x1f971 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f973 as libc::c_int as libc::c_long,
                      last: 0x1f976 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f97a as libc::c_int as libc::c_long,
                      last: 0x1f9a2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9a5 as libc::c_int as libc::c_long,
                      last: 0x1f9aa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9ae as libc::c_int as libc::c_long,
                      last: 0x1f9ca as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9cd as libc::c_int as libc::c_long,
                      last: 0x1f9ff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa70 as libc::c_int as libc::c_long,
                      last: 0x1fa73 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa78 as libc::c_int as libc::c_long,
                      last: 0x1fa7a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa80 as libc::c_int as libc::c_long,
                      last: 0x1fa82 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa90 as libc::c_int as libc::c_long,
                      last: 0x1fa95 as libc::c_int as libc::c_long,};
         init
     }];
pub static mut emoji_width: [interval; 39] =
    [{
         let mut init =
             interval{first: 0x1f1e6 as libc::c_int as libc::c_long,
                      last: 0x1f1ff as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f321 as libc::c_int as libc::c_long,
                      last: 0x1f321 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f324 as libc::c_int as libc::c_long,
                      last: 0x1f32c as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f336 as libc::c_int as libc::c_long,
                      last: 0x1f336 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f37d as libc::c_int as libc::c_long,
                      last: 0x1f37d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f396 as libc::c_int as libc::c_long,
                      last: 0x1f397 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f399 as libc::c_int as libc::c_long,
                      last: 0x1f39b as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f39e as libc::c_int as libc::c_long,
                      last: 0x1f39f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3cb as libc::c_int as libc::c_long,
                      last: 0x1f3ce as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3d4 as libc::c_int as libc::c_long,
                      last: 0x1f3df as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f3 as libc::c_int as libc::c_long,
                      last: 0x1f3f5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f7 as libc::c_int as libc::c_long,
                      last: 0x1f3f7 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f43f as libc::c_int as libc::c_long,
                      last: 0x1f43f as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f441 as libc::c_int as libc::c_long,
                      last: 0x1f441 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f4fd as libc::c_int as libc::c_long,
                      last: 0x1f4fd as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f549 as libc::c_int as libc::c_long,
                      last: 0x1f54a as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f56f as libc::c_int as libc::c_long,
                      last: 0x1f570 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f573 as libc::c_int as libc::c_long,
                      last: 0x1f579 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f587 as libc::c_int as libc::c_long,
                      last: 0x1f587 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f58a as libc::c_int as libc::c_long,
                      last: 0x1f58d as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f590 as libc::c_int as libc::c_long,
                      last: 0x1f590 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a5 as libc::c_int as libc::c_long,
                      last: 0x1f5a5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a8 as libc::c_int as libc::c_long,
                      last: 0x1f5a8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5b1 as libc::c_int as libc::c_long,
                      last: 0x1f5b2 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5bc as libc::c_int as libc::c_long,
                      last: 0x1f5bc as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5c2 as libc::c_int as libc::c_long,
                      last: 0x1f5c4 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5d1 as libc::c_int as libc::c_long,
                      last: 0x1f5d3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5dc as libc::c_int as libc::c_long,
                      last: 0x1f5de as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e1 as libc::c_int as libc::c_long,
                      last: 0x1f5e1 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e3 as libc::c_int as libc::c_long,
                      last: 0x1f5e3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e8 as libc::c_int as libc::c_long,
                      last: 0x1f5e8 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5ef as libc::c_int as libc::c_long,
                      last: 0x1f5ef as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5f3 as libc::c_int as libc::c_long,
                      last: 0x1f5f3 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5fa as libc::c_int as libc::c_long,
                      last: 0x1f5fa as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6cb as libc::c_int as libc::c_long,
                      last: 0x1f6cf as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6e0 as libc::c_int as libc::c_long,
                      last: 0x1f6e5 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6e9 as libc::c_int as libc::c_long,
                      last: 0x1f6e9 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f0 as libc::c_int as libc::c_long,
                      last: 0x1f6f0 as libc::c_int as libc::c_long,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f3 as libc::c_int as libc::c_long,
                      last: 0x1f6f3 as libc::c_int as libc::c_long,};
         init
     }];
