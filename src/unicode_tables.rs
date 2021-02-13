use crate::*;

pub static mut toLower: [convertStruct; 172] =
    [{
         let mut init =
             convertStruct{rangeStart: 0x41,
                           rangeEnd: 0x5a,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xc0,
                           rangeEnd: 0xd6,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xd8,
                           rangeEnd: 0xde,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x100,
                           rangeEnd: 0x12e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x130,
                           rangeEnd: 0x130,
                           step: -(1),
                           offset: -(199),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x132,
                           rangeEnd: 0x136,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x139,
                           rangeEnd: 0x147,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x14a,
                           rangeEnd: 0x176,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x178,
                           rangeEnd: 0x178,
                           step: -(1),
                           offset: -(121),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x179,
                           rangeEnd: 0x17d,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x181,
                           rangeEnd: 0x181,
                           step: -(1),
                           offset: 210,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x182,
                           rangeEnd: 0x184,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x186,
                           rangeEnd: 0x186,
                           step: -(1),
                           offset: 206,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x187,
                           rangeEnd: 0x187,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x189,
                           rangeEnd: 0x18a,
                           step: 1,
                           offset: 205,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18b,
                           rangeEnd: 0x18b,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18e,
                           rangeEnd: 0x18e,
                           step: -(1),
                           offset: 79,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18f,
                           rangeEnd: 0x18f,
                           step: -(1),
                           offset: 202,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x190,
                           rangeEnd: 0x190,
                           step: -(1),
                           offset: 203,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x191,
                           rangeEnd: 0x191,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x193,
                           rangeEnd: 0x193,
                           step: -(1),
                           offset: 205,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x194,
                           rangeEnd: 0x194,
                           step: -(1),
                           offset: 207,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x196,
                           rangeEnd: 0x196,
                           step: -(1),
                           offset: 211,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x197,
                           rangeEnd: 0x197,
                           step: -(1),
                           offset: 209,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x198,
                           rangeEnd: 0x198,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19c,
                           rangeEnd: 0x19c,
                           step: -(1),
                           offset: 211,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19d,
                           rangeEnd: 0x19d,
                           step: -(1),
                           offset: 213,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19f,
                           rangeEnd: 0x19f,
                           step: -(1),
                           offset: 214,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a0,
                           rangeEnd: 0x1a4,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a6,
                           rangeEnd: 0x1a6,
                           step: -(1),
                           offset: 218,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a7,
                           rangeEnd: 0x1a7,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a9,
                           rangeEnd: 0x1a9,
                           step: -(1),
                           offset: 218,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ac,
                           rangeEnd: 0x1ac,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ae,
                           rangeEnd: 0x1ae,
                           step: -(1),
                           offset: 218,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1af,
                           rangeEnd: 0x1af,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b1,
                           rangeEnd: 0x1b2,
                           step: 1,
                           offset: 217,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b3,
                           rangeEnd: 0x1b5,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b7,
                           rangeEnd: 0x1b7,
                           step: -(1),
                           offset: 219,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b8,
                           rangeEnd: 0x1bc,
                           step: 4,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c4,
                           rangeEnd: 0x1c4,
                           step: -(1),
                           offset: 2,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c5,
                           rangeEnd: 0x1c5,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c7,
                           rangeEnd: 0x1c7,
                           step: -(1),
                           offset: 2,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c8,
                           rangeEnd: 0x1c8,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ca,
                           rangeEnd: 0x1ca,
                           step: -(1),
                           offset: 2,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cb,
                           rangeEnd: 0x1db,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1de,
                           rangeEnd: 0x1ee,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f1,
                           rangeEnd: 0x1f1,
                           step: -(1),
                           offset: 2,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f2,
                           rangeEnd: 0x1f4,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f6,
                           rangeEnd: 0x1f6,
                           step: -(1),
                           offset: -(97),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f7,
                           rangeEnd: 0x1f7,
                           step: -(1),
                           offset: -(56),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f8,
                           rangeEnd: 0x21e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x220,
                           rangeEnd: 0x220,
                           step: -(1),
                           offset: -(130),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x222,
                           rangeEnd: 0x232,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23a,
                           rangeEnd: 0x23a,
                           step: -(1),
                           offset: 10795,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23b,
                           rangeEnd: 0x23b,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23d,
                           rangeEnd: 0x23d,
                           step: -(1),
                           offset: -(163),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23e,
                           rangeEnd: 0x23e,
                           step: -(1),
                           offset: 10792,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x241,
                           rangeEnd: 0x241,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x243,
                           rangeEnd: 0x243,
                           step: -(1),
                           offset: -(195),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x244,
                           rangeEnd: 0x244,
                           step: -(1),
                           offset: 69,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x245,
                           rangeEnd: 0x245,
                           step: -(1),
                           offset: 71,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x246,
                           rangeEnd: 0x24e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x370,
                           rangeEnd: 0x372,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x376,
                           rangeEnd: 0x376,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x37f,
                           rangeEnd: 0x37f,
                           step: -(1),
                           offset: 116,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x386,
                           rangeEnd: 0x386,
                           step: -(1),
                           offset: 38,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x388,
                           rangeEnd: 0x38a,
                           step: 1,
                           offset: 37,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x38c,
                           rangeEnd: 0x38c,
                           step: -(1),
                           offset: 64,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x38e,
                           rangeEnd: 0x38f,
                           step: 1,
                           offset: 63,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x391,
                           rangeEnd: 0x3a1,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3a3,
                           rangeEnd: 0x3ab,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3cf,
                           rangeEnd: 0x3cf,
                           step: -(1),
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d8,
                           rangeEnd: 0x3ee,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f4,
                           rangeEnd: 0x3f4,
                           step: -(1),
                           offset: -(60),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f7,
                           rangeEnd: 0x3f7,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f9,
                           rangeEnd: 0x3f9,
                           step: -(1),
                           offset: -(7),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3fa,
                           rangeEnd: 0x3fa,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3fd,
                           rangeEnd: 0x3ff,
                           step: 1,
                           offset: -(130),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x400,
                           rangeEnd: 0x40f,
                           step: 1,
                           offset: 80,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x410,
                           rangeEnd: 0x42f,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x460,
                           rangeEnd: 0x480,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x48a,
                           rangeEnd: 0x4be,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c0,
                           rangeEnd: 0x4c0,
                           step: -(1),
                           offset: 15,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c1,
                           rangeEnd: 0x4cd,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4d0,
                           rangeEnd: 0x52e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x531,
                           rangeEnd: 0x556,
                           step: 1,
                           offset: 48,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10a0,
                           rangeEnd: 0x10c5,
                           step: 1,
                           offset: 7264,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10c7,
                           rangeEnd: 0x10cd,
                           step: 6,
                           offset: 7264,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13a0,
                           rangeEnd: 0x13ef,
                           step: 1,
                           offset: 38864,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13f0,
                           rangeEnd: 0x13f5,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c90,
                           rangeEnd: 0x1cba,
                           step: 1,
                           offset: -(3008),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cbd,
                           rangeEnd: 0x1cbf,
                           step: 1,
                           offset: -(3008),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e00,
                           rangeEnd: 0x1e94,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e9e,
                           rangeEnd: 0x1e9e,
                           step: -(1),
                           offset: -(7615),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ea0,
                           rangeEnd: 0x1efe,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f08,
                           rangeEnd: 0x1f0f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f18,
                           rangeEnd: 0x1f1d,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f28,
                           rangeEnd: 0x1f2f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f38,
                           rangeEnd: 0x1f3f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f48,
                           rangeEnd: 0x1f4d,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f59,
                           rangeEnd: 0x1f5f,
                           step: 2,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f68,
                           rangeEnd: 0x1f6f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f88,
                           rangeEnd: 0x1f8f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f98,
                           rangeEnd: 0x1f9f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fa8,
                           rangeEnd: 0x1faf,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb8,
                           rangeEnd: 0x1fb9,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fba,
                           rangeEnd: 0x1fbb,
                           step: 1,
                           offset: -(74),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fbc,
                           rangeEnd: 0x1fbc,
                           step: -(1),
                           offset: -(9),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fc8,
                           rangeEnd: 0x1fcb,
                           step: 1,
                           offset: -(86),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fcc,
                           rangeEnd: 0x1fcc,
                           step: -(1),
                           offset: -(9),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fd8,
                           rangeEnd: 0x1fd9,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fda,
                           rangeEnd: 0x1fdb,
                           step: 1,
                           offset: -(100),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fe8,
                           rangeEnd: 0x1fe9,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fea,
                           rangeEnd: 0x1feb,
                           step: 1,
                           offset: -(112),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fec,
                           rangeEnd: 0x1fec,
                           step: -(1),
                           offset: -(7),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ff8,
                           rangeEnd: 0x1ff9,
                           step: 1,
                           offset: -(128),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ffa,
                           rangeEnd: 0x1ffb,
                           step: 1,
                           offset: -(126),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ffc,
                           rangeEnd: 0x1ffc,
                           step: -(1),
                           offset: -(9),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2126,
                           rangeEnd: 0x2126,
                           step: -(1),
                           offset: -(7517),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x212a,
                           rangeEnd: 0x212a,
                           step: -(1),
                           offset: -(8383),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x212b,
                           rangeEnd: 0x212b,
                           step: -(1),
                           offset: -(8262),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2132,
                           rangeEnd: 0x2132,
                           step: -(1),
                           offset: 28,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2160,
                           rangeEnd: 0x216f,
                           step: 1,
                           offset: 16,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2183,
                           rangeEnd: 0x2183,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x24b6,
                           rangeEnd: 0x24cf,
                           step: 1,
                           offset: 26,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c00,
                           rangeEnd: 0x2c2e,
                           step: 1,
                           offset: 48,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c60,
                           rangeEnd: 0x2c60,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c62,
                           rangeEnd: 0x2c62,
                           step: -(1),
                           offset: -(10743),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c63,
                           rangeEnd: 0x2c63,
                           step: -(1),
                           offset: -(3814),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c64,
                           rangeEnd: 0x2c64,
                           step: -(1),
                           offset: -(10727),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c67,
                           rangeEnd: 0x2c6b,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6d,
                           rangeEnd: 0x2c6d,
                           step: -(1),
                           offset: -(10780),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6e,
                           rangeEnd: 0x2c6e,
                           step: -(1),
                           offset: -(10749),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6f,
                           rangeEnd: 0x2c6f,
                           step: -(1),
                           offset: -(10783),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c70,
                           rangeEnd: 0x2c70,
                           step: -(1),
                           offset: -(10782),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c72,
                           rangeEnd: 0x2c75,
                           step: 3,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c7e,
                           rangeEnd: 0x2c7f,
                           step: 1,
                           offset: -(10815),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c80,
                           rangeEnd: 0x2ce2,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2ceb,
                           rangeEnd: 0x2ced,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2cf2,
                           rangeEnd: 0xa640,
                           step: 31054,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa642,
                           rangeEnd: 0xa66c,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa680,
                           rangeEnd: 0xa69a,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa722,
                           rangeEnd: 0xa72e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa732,
                           rangeEnd: 0xa76e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa779,
                           rangeEnd: 0xa77b,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77d,
                           rangeEnd: 0xa77d,
                           step: -(1),
                           offset: -(35332),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77e,
                           rangeEnd: 0xa786,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78b,
                           rangeEnd: 0xa78b,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78d,
                           rangeEnd: 0xa78d,
                           step: -(1),
                           offset: -(42280),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa790,
                           rangeEnd: 0xa792,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa796,
                           rangeEnd: 0xa7a8,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7aa,
                           rangeEnd: 0xa7aa,
                           step: -(1),
                           offset: -(42308),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ab,
                           rangeEnd: 0xa7ab,
                           step: -(1),
                           offset: -(42319),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ac,
                           rangeEnd: 0xa7ac,
                           step: -(1),
                           offset: -(42315),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ad,
                           rangeEnd: 0xa7ad,
                           step: -(1),
                           offset: -(42305),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ae,
                           rangeEnd: 0xa7ae,
                           step: -(1),
                           offset: -(42308),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b0,
                           rangeEnd: 0xa7b0,
                           step: -(1),
                           offset: -(42258),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b1,
                           rangeEnd: 0xa7b1,
                           step: -(1),
                           offset: -(42282),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b2,
                           rangeEnd: 0xa7b2,
                           step: -(1),
                           offset: -(42261),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b3,
                           rangeEnd: 0xa7b3,
                           step: -(1),
                           offset: 928,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b4,
                           rangeEnd: 0xa7be,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c2,
                           rangeEnd: 0xa7c2,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c4,
                           rangeEnd: 0xa7c4,
                           step: -(1),
                           offset: -(48),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c5,
                           rangeEnd: 0xa7c5,
                           step: -(1),
                           offset: -(42307),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c6,
                           rangeEnd: 0xa7c6,
                           step: -(1),
                           offset: -(35384),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xff21,
                           rangeEnd: 0xff3a,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10400,
                           rangeEnd: 0x10427,
                           step: 1,
                           offset: 40,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x104b0,
                           rangeEnd: 0x104d3,
                           step: 1,
                           offset: 40,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10c80,
                           rangeEnd: 0x10cb2,
                           step: 1,
                           offset: 64,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x118a0,
                           rangeEnd: 0x118bf,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x16e40,
                           rangeEnd: 0x16e5f,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e900,
                           rangeEnd: 0x1e921,
                           step: 1,
                           offset: 34,};
         init
     }];
pub static mut toUpper: [convertStruct; 187] =
    [{
         let mut init =
             convertStruct{rangeStart: 0x61,
                           rangeEnd: 0x7a,
                           step: 1,
                           offset: -(32),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xb5,
                           rangeEnd: 0xb5,
                           step: -(1),
                           offset: 743,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xe0,
                           rangeEnd: 0xf6,
                           step: 1,
                           offset: -(32),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xf8,
                           rangeEnd: 0xfe,
                           step: 1,
                           offset: -(32),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xff,
                           rangeEnd: 0xff,
                           step: -(1),
                           offset: 121,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x101,
                           rangeEnd: 0x12f,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x131,
                           rangeEnd: 0x131,
                           step: -(1),
                           offset: -(232),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x133,
                           rangeEnd: 0x137,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13a,
                           rangeEnd: 0x148,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x14b,
                           rangeEnd: 0x177,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x17a,
                           rangeEnd: 0x17e,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x17f,
                           rangeEnd: 0x17f,
                           step: -(1),
                           offset: -(300),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x180,
                           rangeEnd: 0x180,
                           step: -(1),
                           offset: 195,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x183,
                           rangeEnd: 0x185,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x188,
                           rangeEnd: 0x18c,
                           step: 4,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x192,
                           rangeEnd: 0x192,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x195,
                           rangeEnd: 0x195,
                           step: -(1),
                           offset: 97,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x199,
                           rangeEnd: 0x199,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19a,
                           rangeEnd: 0x19a,
                           step: -(1),
                           offset: 163,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19e,
                           rangeEnd: 0x19e,
                           step: -(1),
                           offset: 130,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a1,
                           rangeEnd: 0x1a5,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a8,
                           rangeEnd: 0x1ad,
                           step: 5,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b0,
                           rangeEnd: 0x1b4,
                           step: 4,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b6,
                           rangeEnd: 0x1b9,
                           step: 3,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1bd,
                           rangeEnd: 0x1bd,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1bf,
                           rangeEnd: 0x1bf,
                           step: -(1),
                           offset: 56,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c5,
                           rangeEnd: 0x1c5,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c6,
                           rangeEnd: 0x1c6,
                           step: -(1),
                           offset: -(2),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c8,
                           rangeEnd: 0x1c8,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c9,
                           rangeEnd: 0x1c9,
                           step: -(1),
                           offset: -(2),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cb,
                           rangeEnd: 0x1cb,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cc,
                           rangeEnd: 0x1cc,
                           step: -(1),
                           offset: -(2),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ce,
                           rangeEnd: 0x1dc,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1dd,
                           rangeEnd: 0x1dd,
                           step: -(1),
                           offset: -(79),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1df,
                           rangeEnd: 0x1ef,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f2,
                           rangeEnd: 0x1f2,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f3,
                           rangeEnd: 0x1f3,
                           step: -(1),
                           offset: -(2),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f5,
                           rangeEnd: 0x1f9,
                           step: 4,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb,
                           rangeEnd: 0x21f,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x223,
                           rangeEnd: 0x233,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23c,
                           rangeEnd: 0x23c,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23f,
                           rangeEnd: 0x240,
                           step: 1,
                           offset: 10815,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x242,
                           rangeEnd: 0x247,
                           step: 5,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x249,
                           rangeEnd: 0x24f,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x250,
                           rangeEnd: 0x250,
                           step: -(1),
                           offset: 10783,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x251,
                           rangeEnd: 0x251,
                           step: -(1),
                           offset: 10780,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x252,
                           rangeEnd: 0x252,
                           step: -(1),
                           offset: 10782,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x253,
                           rangeEnd: 0x253,
                           step: -(1),
                           offset: -(210),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x254,
                           rangeEnd: 0x254,
                           step: -(1),
                           offset: -(206),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x256,
                           rangeEnd: 0x257,
                           step: 1,
                           offset: -(205),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x259,
                           rangeEnd: 0x259,
                           step: -(1),
                           offset: -(202),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x25b,
                           rangeEnd: 0x25b,
                           step: -(1),
                           offset: -(203),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x25c,
                           rangeEnd: 0x25c,
                           step: -(1),
                           offset: 42319,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x260,
                           rangeEnd: 0x260,
                           step: -(1),
                           offset: -(205),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x261,
                           rangeEnd: 0x261,
                           step: -(1),
                           offset: 42315,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x263,
                           rangeEnd: 0x263,
                           step: -(1),
                           offset: -(207),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x265,
                           rangeEnd: 0x265,
                           step: -(1),
                           offset: 42280,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x266,
                           rangeEnd: 0x266,
                           step: -(1),
                           offset: 42308,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x268,
                           rangeEnd: 0x268,
                           step: -(1),
                           offset: -(209),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x269,
                           rangeEnd: 0x269,
                           step: -(1),
                           offset: -(211),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x26a,
                           rangeEnd: 0x26a,
                           step: -(1),
                           offset: 42308,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x26b,
                           rangeEnd: 0x26b,
                           step: -(1),
                           offset: 10743,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x26c,
                           rangeEnd: 0x26c,
                           step: -(1),
                           offset: 42305,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x26f,
                           rangeEnd: 0x26f,
                           step: -(1),
                           offset: -(211),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x271,
                           rangeEnd: 0x271,
                           step: -(1),
                           offset: 10749,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x272,
                           rangeEnd: 0x272,
                           step: -(1),
                           offset: -(213),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x275,
                           rangeEnd: 0x275,
                           step: -(1),
                           offset: -(214),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x27d,
                           rangeEnd: 0x27d,
                           step: -(1),
                           offset: 10727,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x280,
                           rangeEnd: 0x280,
                           step: -(1),
                           offset: -(218),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x282,
                           rangeEnd: 0x282,
                           step: -(1),
                           offset: 42307,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x283,
                           rangeEnd: 0x283,
                           step: -(1),
                           offset: -(218),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x287,
                           rangeEnd: 0x287,
                           step: -(1),
                           offset: 42282,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x288,
                           rangeEnd: 0x288,
                           step: -(1),
                           offset: -(218),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x289,
                           rangeEnd: 0x289,
                           step: -(1),
                           offset: -(69),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x28a,
                           rangeEnd: 0x28b,
                           step: 1,
                           offset: -(217),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x28c,
                           rangeEnd: 0x28c,
                           step: -(1),
                           offset: -(71),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x292,
                           rangeEnd: 0x292,
                           step: -(1),
                           offset: -(219),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x29d,
                           rangeEnd: 0x29d,
                           step: -(1),
                           offset: 42261,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x29e,
                           rangeEnd: 0x29e,
                           step: -(1),
                           offset: 42258,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x345,
                           rangeEnd: 0x345,
                           step: -(1),
                           offset: 84,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x371,
                           rangeEnd: 0x373,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x377,
                           rangeEnd: 0x377,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x37b,
                           rangeEnd: 0x37d,
                           step: 1,
                           offset: 130,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3ac,
                           rangeEnd: 0x3ac,
                           step: -(1),
                           offset: -(38),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3ad,
                           rangeEnd: 0x3af,
                           step: 1,
                           offset: -(37),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3b1,
                           rangeEnd: 0x3c1,
                           step: 1,
                           offset: -(32),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3c2,
                           rangeEnd: 0x3c2,
                           step: -(1),
                           offset: -(31),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3c3,
                           rangeEnd: 0x3cb,
                           step: 1,
                           offset: -(32),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3cc,
                           rangeEnd: 0x3cc,
                           step: -(1),
                           offset: -(64),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3cd,
                           rangeEnd: 0x3ce,
                           step: 1,
                           offset: -(63),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d0,
                           rangeEnd: 0x3d0,
                           step: -(1),
                           offset: -(62),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d1,
                           rangeEnd: 0x3d1,
                           step: -(1),
                           offset: -(57),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d5,
                           rangeEnd: 0x3d5,
                           step: -(1),
                           offset: -(47),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d6,
                           rangeEnd: 0x3d6,
                           step: -(1),
                           offset: -(54),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d7,
                           rangeEnd: 0x3d7,
                           step: -(1),
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d9,
                           rangeEnd: 0x3ef,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f0,
                           rangeEnd: 0x3f0,
                           step: -(1),
                           offset: -(86),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f1,
                           rangeEnd: 0x3f1,
                           step: -(1),
                           offset: -(80),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f2,
                           rangeEnd: 0x3f2,
                           step: -(1),
                           offset: 7,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f3,
                           rangeEnd: 0x3f3,
                           step: -(1),
                           offset: -(116),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f5,
                           rangeEnd: 0x3f5,
                           step: -(1),
                           offset: -(96),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f8,
                           rangeEnd: 0x3fb,
                           step: 3,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x430,
                           rangeEnd: 0x44f,
                           step: 1,
                           offset: -(32),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x450,
                           rangeEnd: 0x45f,
                           step: 1,
                           offset: -(80),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x461,
                           rangeEnd: 0x481,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x48b,
                           rangeEnd: 0x4bf,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c2,
                           rangeEnd: 0x4ce,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4cf,
                           rangeEnd: 0x4cf,
                           step: -(1),
                           offset: -(15),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4d1,
                           rangeEnd: 0x52f,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x561,
                           rangeEnd: 0x586,
                           step: 1,
                           offset: -(48),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10d0,
                           rangeEnd: 0x10fa,
                           step: 1,
                           offset: 3008,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10fd,
                           rangeEnd: 0x10ff,
                           step: 1,
                           offset: 3008,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13f8,
                           rangeEnd: 0x13fd,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c80,
                           rangeEnd: 0x1c80,
                           step: -(1),
                           offset: -(6254),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c81,
                           rangeEnd: 0x1c81,
                           step: -(1),
                           offset: -(6253),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c82,
                           rangeEnd: 0x1c82,
                           step: -(1),
                           offset: -(6244),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c83,
                           rangeEnd: 0x1c84,
                           step: 1,
                           offset: -(6242),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c85,
                           rangeEnd: 0x1c85,
                           step: -(1),
                           offset: -(6243),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c86,
                           rangeEnd: 0x1c86,
                           step: -(1),
                           offset: -(6236),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c87,
                           rangeEnd: 0x1c87,
                           step: -(1),
                           offset: -(6181),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c88,
                           rangeEnd: 0x1c88,
                           step: -(1),
                           offset: 35266,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1d79,
                           rangeEnd: 0x1d79,
                           step: -(1),
                           offset: 35332,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1d7d,
                           rangeEnd: 0x1d7d,
                           step: -(1),
                           offset: 3814,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1d8e,
                           rangeEnd: 0x1d8e,
                           step: -(1),
                           offset: 35384,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e01,
                           rangeEnd: 0x1e95,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e9b,
                           rangeEnd: 0x1e9b,
                           step: -(1),
                           offset: -(59),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ea1,
                           rangeEnd: 0x1eff,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f00,
                           rangeEnd: 0x1f07,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f10,
                           rangeEnd: 0x1f15,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f20,
                           rangeEnd: 0x1f27,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f30,
                           rangeEnd: 0x1f37,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f40,
                           rangeEnd: 0x1f45,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f51,
                           rangeEnd: 0x1f57,
                           step: 2,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f60,
                           rangeEnd: 0x1f67,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f70,
                           rangeEnd: 0x1f71,
                           step: 1,
                           offset: 74,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f72,
                           rangeEnd: 0x1f75,
                           step: 1,
                           offset: 86,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f76,
                           rangeEnd: 0x1f77,
                           step: 1,
                           offset: 100,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f78,
                           rangeEnd: 0x1f79,
                           step: 1,
                           offset: 128,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f7a,
                           rangeEnd: 0x1f7b,
                           step: 1,
                           offset: 112,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f7c,
                           rangeEnd: 0x1f7d,
                           step: 1,
                           offset: 126,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f80,
                           rangeEnd: 0x1f87,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f90,
                           rangeEnd: 0x1f97,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fa0,
                           rangeEnd: 0x1fa7,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb0,
                           rangeEnd: 0x1fb1,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb3,
                           rangeEnd: 0x1fb3,
                           step: -(1),
                           offset: 9,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fbe,
                           rangeEnd: 0x1fbe,
                           step: -(1),
                           offset: -(7205),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fc3,
                           rangeEnd: 0x1fc3,
                           step: -(1),
                           offset: 9,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fd0,
                           rangeEnd: 0x1fd1,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fe0,
                           rangeEnd: 0x1fe1,
                           step: 1,
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fe5,
                           rangeEnd: 0x1fe5,
                           step: -(1),
                           offset: 7,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ff3,
                           rangeEnd: 0x1ff3,
                           step: -(1),
                           offset: 9,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x214e,
                           rangeEnd: 0x214e,
                           step: -(1),
                           offset: -(28),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2170,
                           rangeEnd: 0x217f,
                           step: 1,
                           offset: -(16),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2184,
                           rangeEnd: 0x2184,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x24d0,
                           rangeEnd: 0x24e9,
                           step: 1,
                           offset: -(26),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c30,
                           rangeEnd: 0x2c5e,
                           step: 1,
                           offset: -(48),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c61,
                           rangeEnd: 0x2c61,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c65,
                           rangeEnd: 0x2c65,
                           step: -(1),
                           offset: -(10795),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c66,
                           rangeEnd: 0x2c66,
                           step: -(1),
                           offset: -(10792),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c68,
                           rangeEnd: 0x2c6c,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c73,
                           rangeEnd: 0x2c76,
                           step: 3,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c81,
                           rangeEnd: 0x2ce3,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2cec,
                           rangeEnd: 0x2cee,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2cf3,
                           rangeEnd: 0x2cf3,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2d00,
                           rangeEnd: 0x2d25,
                           step: 1,
                           offset: -(7264),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2d27,
                           rangeEnd: 0x2d2d,
                           step: 6,
                           offset: -(7264),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa641,
                           rangeEnd: 0xa66d,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa681,
                           rangeEnd: 0xa69b,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa723,
                           rangeEnd: 0xa72f,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa733,
                           rangeEnd: 0xa76f,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77a,
                           rangeEnd: 0xa77c,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77f,
                           rangeEnd: 0xa787,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78c,
                           rangeEnd: 0xa791,
                           step: 5,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa793,
                           rangeEnd: 0xa793,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa794,
                           rangeEnd: 0xa794,
                           step: -(1),
                           offset: 48,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa797,
                           rangeEnd: 0xa7a9,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b5,
                           rangeEnd: 0xa7bf,
                           step: 2,
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c3,
                           rangeEnd: 0xa7c3,
                           step: -(1),
                           offset: -(1),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xab53,
                           rangeEnd: 0xab53,
                           step: -(1),
                           offset: -(928),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xab70,
                           rangeEnd: 0xabbf,
                           step: 1,
                           offset: -(38864),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xff41,
                           rangeEnd: 0xff5a,
                           step: 1,
                           offset: -(32),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10428,
                           rangeEnd: 0x1044f,
                           step: 1,
                           offset: -(40),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x104d8,
                           rangeEnd: 0x104fb,
                           step: 1,
                           offset: -(40),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10cc0,
                           rangeEnd: 0x10cf2,
                           step: 1,
                           offset: -(64),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x118c0,
                           rangeEnd: 0x118df,
                           step: 1,
                           offset: -(32),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x16e60,
                           rangeEnd: 0x16e7f,
                           step: 1,
                           offset: -(32),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e922,
                           rangeEnd: 0x1e943,
                           step: 1,
                           offset: -(34),};
         init
     }];
pub static mut combining: [interval; 280] =
    [{
         let mut init =
             interval{first: 0x300,
                      last: 0x36f,};
         init
     },
     {
         let mut init =
             interval{first: 0x483,
                      last: 0x489,};
         init
     },
     {
         let mut init =
             interval{first: 0x591,
                      last: 0x5bd,};
         init
     },
     {
         let mut init =
             interval{first: 0x5bf,
                      last: 0x5bf,};
         init
     },
     {
         let mut init =
             interval{first: 0x5c1,
                      last: 0x5c2,};
         init
     },
     {
         let mut init =
             interval{first: 0x5c4,
                      last: 0x5c5,};
         init
     },
     {
         let mut init =
             interval{first: 0x5c7,
                      last: 0x5c7,};
         init
     },
     {
         let mut init =
             interval{first: 0x610,
                      last: 0x61a,};
         init
     },
     {
         let mut init =
             interval{first: 0x64b,
                      last: 0x65f,};
         init
     },
     {
         let mut init =
             interval{first: 0x670,
                      last: 0x670,};
         init
     },
     {
         let mut init =
             interval{first: 0x6d6,
                      last: 0x6dc,};
         init
     },
     {
         let mut init =
             interval{first: 0x6df,
                      last: 0x6e4,};
         init
     },
     {
         let mut init =
             interval{first: 0x6e7,
                      last: 0x6e8,};
         init
     },
     {
         let mut init =
             interval{first: 0x6ea,
                      last: 0x6ed,};
         init
     },
     {
         let mut init =
             interval{first: 0x711,
                      last: 0x711,};
         init
     },
     {
         let mut init =
             interval{first: 0x730,
                      last: 0x74a,};
         init
     },
     {
         let mut init =
             interval{first: 0x7a6,
                      last: 0x7b0,};
         init
     },
     {
         let mut init =
             interval{first: 0x7eb,
                      last: 0x7f3,};
         init
     },
     {
         let mut init =
             interval{first: 0x7fd,
                      last: 0x7fd,};
         init
     },
     {
         let mut init =
             interval{first: 0x816,
                      last: 0x819,};
         init
     },
     {
         let mut init =
             interval{first: 0x81b,
                      last: 0x823,};
         init
     },
     {
         let mut init =
             interval{first: 0x825,
                      last: 0x827,};
         init
     },
     {
         let mut init =
             interval{first: 0x829,
                      last: 0x82d,};
         init
     },
     {
         let mut init =
             interval{first: 0x859,
                      last: 0x85b,};
         init
     },
     {
         let mut init =
             interval{first: 0x8d3,
                      last: 0x8e1,};
         init
     },
     {
         let mut init =
             interval{first: 0x8e3,
                      last: 0x903,};
         init
     },
     {
         let mut init =
             interval{first: 0x93a,
                      last: 0x93c,};
         init
     },
     {
         let mut init =
             interval{first: 0x93e,
                      last: 0x94f,};
         init
     },
     {
         let mut init =
             interval{first: 0x951,
                      last: 0x957,};
         init
     },
     {
         let mut init =
             interval{first: 0x962,
                      last: 0x963,};
         init
     },
     {
         let mut init =
             interval{first: 0x981,
                      last: 0x983,};
         init
     },
     {
         let mut init =
             interval{first: 0x9bc,
                      last: 0x9bc,};
         init
     },
     {
         let mut init =
             interval{first: 0x9be,
                      last: 0x9c4,};
         init
     },
     {
         let mut init =
             interval{first: 0x9c7,
                      last: 0x9c8,};
         init
     },
     {
         let mut init =
             interval{first: 0x9cb,
                      last: 0x9cd,};
         init
     },
     {
         let mut init =
             interval{first: 0x9d7,
                      last: 0x9d7,};
         init
     },
     {
         let mut init =
             interval{first: 0x9e2,
                      last: 0x9e3,};
         init
     },
     {
         let mut init =
             interval{first: 0x9fe,
                      last: 0x9fe,};
         init
     },
     {
         let mut init =
             interval{first: 0xa01,
                      last: 0xa03,};
         init
     },
     {
         let mut init =
             interval{first: 0xa3c,
                      last: 0xa3c,};
         init
     },
     {
         let mut init =
             interval{first: 0xa3e,
                      last: 0xa42,};
         init
     },
     {
         let mut init =
             interval{first: 0xa47,
                      last: 0xa48,};
         init
     },
     {
         let mut init =
             interval{first: 0xa4b,
                      last: 0xa4d,};
         init
     },
     {
         let mut init =
             interval{first: 0xa51,
                      last: 0xa51,};
         init
     },
     {
         let mut init =
             interval{first: 0xa70,
                      last: 0xa71,};
         init
     },
     {
         let mut init =
             interval{first: 0xa75,
                      last: 0xa75,};
         init
     },
     {
         let mut init =
             interval{first: 0xa81,
                      last: 0xa83,};
         init
     },
     {
         let mut init =
             interval{first: 0xabc,
                      last: 0xabc,};
         init
     },
     {
         let mut init =
             interval{first: 0xabe,
                      last: 0xac5,};
         init
     },
     {
         let mut init =
             interval{first: 0xac7,
                      last: 0xac9,};
         init
     },
     {
         let mut init =
             interval{first: 0xacb,
                      last: 0xacd,};
         init
     },
     {
         let mut init =
             interval{first: 0xae2,
                      last: 0xae3,};
         init
     },
     {
         let mut init =
             interval{first: 0xafa,
                      last: 0xaff,};
         init
     },
     {
         let mut init =
             interval{first: 0xb01,
                      last: 0xb03,};
         init
     },
     {
         let mut init =
             interval{first: 0xb3c,
                      last: 0xb3c,};
         init
     },
     {
         let mut init =
             interval{first: 0xb3e,
                      last: 0xb44,};
         init
     },
     {
         let mut init =
             interval{first: 0xb47,
                      last: 0xb48,};
         init
     },
     {
         let mut init =
             interval{first: 0xb4b,
                      last: 0xb4d,};
         init
     },
     {
         let mut init =
             interval{first: 0xb56,
                      last: 0xb57,};
         init
     },
     {
         let mut init =
             interval{first: 0xb62,
                      last: 0xb63,};
         init
     },
     {
         let mut init =
             interval{first: 0xb82,
                      last: 0xb82,};
         init
     },
     {
         let mut init =
             interval{first: 0xbbe,
                      last: 0xbc2,};
         init
     },
     {
         let mut init =
             interval{first: 0xbc6,
                      last: 0xbc8,};
         init
     },
     {
         let mut init =
             interval{first: 0xbca,
                      last: 0xbcd,};
         init
     },
     {
         let mut init =
             interval{first: 0xbd7,
                      last: 0xbd7,};
         init
     },
     {
         let mut init =
             interval{first: 0xc00,
                      last: 0xc04,};
         init
     },
     {
         let mut init =
             interval{first: 0xc3e,
                      last: 0xc44,};
         init
     },
     {
         let mut init =
             interval{first: 0xc46,
                      last: 0xc48,};
         init
     },
     {
         let mut init =
             interval{first: 0xc4a,
                      last: 0xc4d,};
         init
     },
     {
         let mut init =
             interval{first: 0xc55,
                      last: 0xc56,};
         init
     },
     {
         let mut init =
             interval{first: 0xc62,
                      last: 0xc63,};
         init
     },
     {
         let mut init =
             interval{first: 0xc81,
                      last: 0xc83,};
         init
     },
     {
         let mut init =
             interval{first: 0xcbc,
                      last: 0xcbc,};
         init
     },
     {
         let mut init =
             interval{first: 0xcbe,
                      last: 0xcc4,};
         init
     },
     {
         let mut init =
             interval{first: 0xcc6,
                      last: 0xcc8,};
         init
     },
     {
         let mut init =
             interval{first: 0xcca,
                      last: 0xccd,};
         init
     },
     {
         let mut init =
             interval{first: 0xcd5,
                      last: 0xcd6,};
         init
     },
     {
         let mut init =
             interval{first: 0xce2,
                      last: 0xce3,};
         init
     },
     {
         let mut init =
             interval{first: 0xd00,
                      last: 0xd03,};
         init
     },
     {
         let mut init =
             interval{first: 0xd3b,
                      last: 0xd3c,};
         init
     },
     {
         let mut init =
             interval{first: 0xd3e,
                      last: 0xd44,};
         init
     },
     {
         let mut init =
             interval{first: 0xd46,
                      last: 0xd48,};
         init
     },
     {
         let mut init =
             interval{first: 0xd4a,
                      last: 0xd4d,};
         init
     },
     {
         let mut init =
             interval{first: 0xd57,
                      last: 0xd57,};
         init
     },
     {
         let mut init =
             interval{first: 0xd62,
                      last: 0xd63,};
         init
     },
     {
         let mut init =
             interval{first: 0xd82,
                      last: 0xd83,};
         init
     },
     {
         let mut init =
             interval{first: 0xdca,
                      last: 0xdca,};
         init
     },
     {
         let mut init =
             interval{first: 0xdcf,
                      last: 0xdd4,};
         init
     },
     {
         let mut init =
             interval{first: 0xdd6,
                      last: 0xdd6,};
         init
     },
     {
         let mut init =
             interval{first: 0xdd8,
                      last: 0xddf,};
         init
     },
     {
         let mut init =
             interval{first: 0xdf2,
                      last: 0xdf3,};
         init
     },
     {
         let mut init =
             interval{first: 0xe31,
                      last: 0xe31,};
         init
     },
     {
         let mut init =
             interval{first: 0xe34,
                      last: 0xe3a,};
         init
     },
     {
         let mut init =
             interval{first: 0xe47,
                      last: 0xe4e,};
         init
     },
     {
         let mut init =
             interval{first: 0xeb1,
                      last: 0xeb1,};
         init
     },
     {
         let mut init =
             interval{first: 0xeb4,
                      last: 0xebc,};
         init
     },
     {
         let mut init =
             interval{first: 0xec8,
                      last: 0xecd,};
         init
     },
     {
         let mut init =
             interval{first: 0xf18,
                      last: 0xf19,};
         init
     },
     {
         let mut init =
             interval{first: 0xf35,
                      last: 0xf35,};
         init
     },
     {
         let mut init =
             interval{first: 0xf37,
                      last: 0xf37,};
         init
     },
     {
         let mut init =
             interval{first: 0xf39,
                      last: 0xf39,};
         init
     },
     {
         let mut init =
             interval{first: 0xf3e,
                      last: 0xf3f,};
         init
     },
     {
         let mut init =
             interval{first: 0xf71,
                      last: 0xf84,};
         init
     },
     {
         let mut init =
             interval{first: 0xf86,
                      last: 0xf87,};
         init
     },
     {
         let mut init =
             interval{first: 0xf8d,
                      last: 0xf97,};
         init
     },
     {
         let mut init =
             interval{first: 0xf99,
                      last: 0xfbc,};
         init
     },
     {
         let mut init =
             interval{first: 0xfc6,
                      last: 0xfc6,};
         init
     },
     {
         let mut init =
             interval{first: 0x102b,
                      last: 0x103e,};
         init
     },
     {
         let mut init =
             interval{first: 0x1056,
                      last: 0x1059,};
         init
     },
     {
         let mut init =
             interval{first: 0x105e,
                      last: 0x1060,};
         init
     },
     {
         let mut init =
             interval{first: 0x1062,
                      last: 0x1064,};
         init
     },
     {
         let mut init =
             interval{first: 0x1067,
                      last: 0x106d,};
         init
     },
     {
         let mut init =
             interval{first: 0x1071,
                      last: 0x1074,};
         init
     },
     {
         let mut init =
             interval{first: 0x1082,
                      last: 0x108d,};
         init
     },
     {
         let mut init =
             interval{first: 0x108f,
                      last: 0x108f,};
         init
     },
     {
         let mut init =
             interval{first: 0x109a,
                      last: 0x109d,};
         init
     },
     {
         let mut init =
             interval{first: 0x135d,
                      last: 0x135f,};
         init
     },
     {
         let mut init =
             interval{first: 0x1712,
                      last: 0x1714,};
         init
     },
     {
         let mut init =
             interval{first: 0x1732,
                      last: 0x1734,};
         init
     },
     {
         let mut init =
             interval{first: 0x1752,
                      last: 0x1753,};
         init
     },
     {
         let mut init =
             interval{first: 0x1772,
                      last: 0x1773,};
         init
     },
     {
         let mut init =
             interval{first: 0x17b4,
                      last: 0x17d3,};
         init
     },
     {
         let mut init =
             interval{first: 0x17dd,
                      last: 0x17dd,};
         init
     },
     {
         let mut init =
             interval{first: 0x180b,
                      last: 0x180d,};
         init
     },
     {
         let mut init =
             interval{first: 0x1885,
                      last: 0x1886,};
         init
     },
     {
         let mut init =
             interval{first: 0x18a9,
                      last: 0x18a9,};
         init
     },
     {
         let mut init =
             interval{first: 0x1920,
                      last: 0x192b,};
         init
     },
     {
         let mut init =
             interval{first: 0x1930,
                      last: 0x193b,};
         init
     },
     {
         let mut init =
             interval{first: 0x1a17,
                      last: 0x1a1b,};
         init
     },
     {
         let mut init =
             interval{first: 0x1a55,
                      last: 0x1a5e,};
         init
     },
     {
         let mut init =
             interval{first: 0x1a60,
                      last: 0x1a7c,};
         init
     },
     {
         let mut init =
             interval{first: 0x1a7f,
                      last: 0x1a7f,};
         init
     },
     {
         let mut init =
             interval{first: 0x1ab0,
                      last: 0x1abe,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b00,
                      last: 0x1b04,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b34,
                      last: 0x1b44,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b6b,
                      last: 0x1b73,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b80,
                      last: 0x1b82,};
         init
     },
     {
         let mut init =
             interval{first: 0x1ba1,
                      last: 0x1bad,};
         init
     },
     {
         let mut init =
             interval{first: 0x1be6,
                      last: 0x1bf3,};
         init
     },
     {
         let mut init =
             interval{first: 0x1c24,
                      last: 0x1c37,};
         init
     },
     {
         let mut init =
             interval{first: 0x1cd0,
                      last: 0x1cd2,};
         init
     },
     {
         let mut init =
             interval{first: 0x1cd4,
                      last: 0x1ce8,};
         init
     },
     {
         let mut init =
             interval{first: 0x1ced,
                      last: 0x1ced,};
         init
     },
     {
         let mut init =
             interval{first: 0x1cf4,
                      last: 0x1cf4,};
         init
     },
     {
         let mut init =
             interval{first: 0x1cf7,
                      last: 0x1cf9,};
         init
     },
     {
         let mut init =
             interval{first: 0x1dc0,
                      last: 0x1df9,};
         init
     },
     {
         let mut init =
             interval{first: 0x1dfb,
                      last: 0x1dff,};
         init
     },
     {
         let mut init =
             interval{first: 0x20d0,
                      last: 0x20f0,};
         init
     },
     {
         let mut init =
             interval{first: 0x2cef,
                      last: 0x2cf1,};
         init
     },
     {
         let mut init =
             interval{first: 0x2d7f,
                      last: 0x2d7f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2de0,
                      last: 0x2dff,};
         init
     },
     {
         let mut init =
             interval{first: 0x302a,
                      last: 0x302f,};
         init
     },
     {
         let mut init =
             interval{first: 0x3099,
                      last: 0x309a,};
         init
     },
     {
         let mut init =
             interval{first: 0xa66f,
                      last: 0xa672,};
         init
     },
     {
         let mut init =
             interval{first: 0xa674,
                      last: 0xa67d,};
         init
     },
     {
         let mut init =
             interval{first: 0xa69e,
                      last: 0xa69f,};
         init
     },
     {
         let mut init =
             interval{first: 0xa6f0,
                      last: 0xa6f1,};
         init
     },
     {
         let mut init =
             interval{first: 0xa802,
                      last: 0xa802,};
         init
     },
     {
         let mut init =
             interval{first: 0xa806,
                      last: 0xa806,};
         init
     },
     {
         let mut init =
             interval{first: 0xa80b,
                      last: 0xa80b,};
         init
     },
     {
         let mut init =
             interval{first: 0xa823,
                      last: 0xa827,};
         init
     },
     {
         let mut init =
             interval{first: 0xa880,
                      last: 0xa881,};
         init
     },
     {
         let mut init =
             interval{first: 0xa8b4,
                      last: 0xa8c5,};
         init
     },
     {
         let mut init =
             interval{first: 0xa8e0,
                      last: 0xa8f1,};
         init
     },
     {
         let mut init =
             interval{first: 0xa8ff,
                      last: 0xa8ff,};
         init
     },
     {
         let mut init =
             interval{first: 0xa926,
                      last: 0xa92d,};
         init
     },
     {
         let mut init =
             interval{first: 0xa947,
                      last: 0xa953,};
         init
     },
     {
         let mut init =
             interval{first: 0xa980,
                      last: 0xa983,};
         init
     },
     {
         let mut init =
             interval{first: 0xa9b3,
                      last: 0xa9c0,};
         init
     },
     {
         let mut init =
             interval{first: 0xa9e5,
                      last: 0xa9e5,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa29,
                      last: 0xaa36,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa43,
                      last: 0xaa43,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa4c,
                      last: 0xaa4d,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa7b,
                      last: 0xaa7d,};
         init
     },
     {
         let mut init =
             interval{first: 0xaab0,
                      last: 0xaab0,};
         init
     },
     {
         let mut init =
             interval{first: 0xaab2,
                      last: 0xaab4,};
         init
     },
     {
         let mut init =
             interval{first: 0xaab7,
                      last: 0xaab8,};
         init
     },
     {
         let mut init =
             interval{first: 0xaabe,
                      last: 0xaabf,};
         init
     },
     {
         let mut init =
             interval{first: 0xaac1,
                      last: 0xaac1,};
         init
     },
     {
         let mut init =
             interval{first: 0xaaeb,
                      last: 0xaaef,};
         init
     },
     {
         let mut init =
             interval{first: 0xaaf5,
                      last: 0xaaf6,};
         init
     },
     {
         let mut init =
             interval{first: 0xabe3,
                      last: 0xabea,};
         init
     },
     {
         let mut init =
             interval{first: 0xabec,
                      last: 0xabed,};
         init
     },
     {
         let mut init =
             interval{first: 0xfb1e,
                      last: 0xfb1e,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe00,
                      last: 0xfe0f,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe20,
                      last: 0xfe2f,};
         init
     },
     {
         let mut init =
             interval{first: 0x101fd,
                      last: 0x101fd,};
         init
     },
     {
         let mut init =
             interval{first: 0x102e0,
                      last: 0x102e0,};
         init
     },
     {
         let mut init =
             interval{first: 0x10376,
                      last: 0x1037a,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a01,
                      last: 0x10a03,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a05,
                      last: 0x10a06,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a0c,
                      last: 0x10a0f,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a38,
                      last: 0x10a3a,};
         init
     },
     {
         let mut init =
             interval{first: 0x10a3f,
                      last: 0x10a3f,};
         init
     },
     {
         let mut init =
             interval{first: 0x10ae5,
                      last: 0x10ae6,};
         init
     },
     {
         let mut init =
             interval{first: 0x10d24,
                      last: 0x10d27,};
         init
     },
     {
         let mut init =
             interval{first: 0x10f46,
                      last: 0x10f50,};
         init
     },
     {
         let mut init =
             interval{first: 0x11000,
                      last: 0x11002,};
         init
     },
     {
         let mut init =
             interval{first: 0x11038,
                      last: 0x11046,};
         init
     },
     {
         let mut init =
             interval{first: 0x1107f,
                      last: 0x11082,};
         init
     },
     {
         let mut init =
             interval{first: 0x110b0,
                      last: 0x110ba,};
         init
     },
     {
         let mut init =
             interval{first: 0x11100,
                      last: 0x11102,};
         init
     },
     {
         let mut init =
             interval{first: 0x11127,
                      last: 0x11134,};
         init
     },
     {
         let mut init =
             interval{first: 0x11145,
                      last: 0x11146,};
         init
     },
     {
         let mut init =
             interval{first: 0x11173,
                      last: 0x11173,};
         init
     },
     {
         let mut init =
             interval{first: 0x11180,
                      last: 0x11182,};
         init
     },
     {
         let mut init =
             interval{first: 0x111b3,
                      last: 0x111c0,};
         init
     },
     {
         let mut init =
             interval{first: 0x111c9,
                      last: 0x111cc,};
         init
     },
     {
         let mut init =
             interval{first: 0x1122c,
                      last: 0x11237,};
         init
     },
     {
         let mut init =
             interval{first: 0x1123e,
                      last: 0x1123e,};
         init
     },
     {
         let mut init =
             interval{first: 0x112df,
                      last: 0x112ea,};
         init
     },
     {
         let mut init =
             interval{first: 0x11300,
                      last: 0x11303,};
         init
     },
     {
         let mut init =
             interval{first: 0x1133b,
                      last: 0x1133c,};
         init
     },
     {
         let mut init =
             interval{first: 0x1133e,
                      last: 0x11344,};
         init
     },
     {
         let mut init =
             interval{first: 0x11347,
                      last: 0x11348,};
         init
     },
     {
         let mut init =
             interval{first: 0x1134b,
                      last: 0x1134d,};
         init
     },
     {
         let mut init =
             interval{first: 0x11357,
                      last: 0x11357,};
         init
     },
     {
         let mut init =
             interval{first: 0x11362,
                      last: 0x11363,};
         init
     },
     {
         let mut init =
             interval{first: 0x11366,
                      last: 0x1136c,};
         init
     },
     {
         let mut init =
             interval{first: 0x11370,
                      last: 0x11374,};
         init
     },
     {
         let mut init =
             interval{first: 0x11435,
                      last: 0x11446,};
         init
     },
     {
         let mut init =
             interval{first: 0x1145e,
                      last: 0x1145e,};
         init
     },
     {
         let mut init =
             interval{first: 0x114b0,
                      last: 0x114c3,};
         init
     },
     {
         let mut init =
             interval{first: 0x115af,
                      last: 0x115b5,};
         init
     },
     {
         let mut init =
             interval{first: 0x115b8,
                      last: 0x115c0,};
         init
     },
     {
         let mut init =
             interval{first: 0x115dc,
                      last: 0x115dd,};
         init
     },
     {
         let mut init =
             interval{first: 0x11630,
                      last: 0x11640,};
         init
     },
     {
         let mut init =
             interval{first: 0x116ab,
                      last: 0x116b7,};
         init
     },
     {
         let mut init =
             interval{first: 0x1171d,
                      last: 0x1172b,};
         init
     },
     {
         let mut init =
             interval{first: 0x1182c,
                      last: 0x1183a,};
         init
     },
     {
         let mut init =
             interval{first: 0x119d1,
                      last: 0x119d7,};
         init
     },
     {
         let mut init =
             interval{first: 0x119da,
                      last: 0x119e0,};
         init
     },
     {
         let mut init =
             interval{first: 0x119e4,
                      last: 0x119e4,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a01,
                      last: 0x11a0a,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a33,
                      last: 0x11a39,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a3b,
                      last: 0x11a3e,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a47,
                      last: 0x11a47,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a51,
                      last: 0x11a5b,};
         init
     },
     {
         let mut init =
             interval{first: 0x11a8a,
                      last: 0x11a99,};
         init
     },
     {
         let mut init =
             interval{first: 0x11c2f,
                      last: 0x11c36,};
         init
     },
     {
         let mut init =
             interval{first: 0x11c38,
                      last: 0x11c3f,};
         init
     },
     {
         let mut init =
             interval{first: 0x11c92,
                      last: 0x11ca7,};
         init
     },
     {
         let mut init =
             interval{first: 0x11ca9,
                      last: 0x11cb6,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d31,
                      last: 0x11d36,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d3a,
                      last: 0x11d3a,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d3c,
                      last: 0x11d3d,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d3f,
                      last: 0x11d45,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d47,
                      last: 0x11d47,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d8a,
                      last: 0x11d8e,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d90,
                      last: 0x11d91,};
         init
     },
     {
         let mut init =
             interval{first: 0x11d93,
                      last: 0x11d97,};
         init
     },
     {
         let mut init =
             interval{first: 0x11ef3,
                      last: 0x11ef6,};
         init
     },
     {
         let mut init =
             interval{first: 0x16af0,
                      last: 0x16af4,};
         init
     },
     {
         let mut init =
             interval{first: 0x16b30,
                      last: 0x16b36,};
         init
     },
     {
         let mut init =
             interval{first: 0x16f4f,
                      last: 0x16f4f,};
         init
     },
     {
         let mut init =
             interval{first: 0x16f51,
                      last: 0x16f87,};
         init
     },
     {
         let mut init =
             interval{first: 0x16f8f,
                      last: 0x16f92,};
         init
     },
     {
         let mut init =
             interval{first: 0x1bc9d,
                      last: 0x1bc9e,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d165,
                      last: 0x1d169,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d16d,
                      last: 0x1d172,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d17b,
                      last: 0x1d182,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d185,
                      last: 0x1d18b,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d1aa,
                      last: 0x1d1ad,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d242,
                      last: 0x1d244,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da00,
                      last: 0x1da36,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da3b,
                      last: 0x1da6c,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da75,
                      last: 0x1da75,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da84,
                      last: 0x1da84,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da9b,
                      last: 0x1da9f,};
         init
     },
     {
         let mut init =
             interval{first: 0x1daa1,
                      last: 0x1daaf,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e000,
                      last: 0x1e006,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e008,
                      last: 0x1e018,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e01b,
                      last: 0x1e021,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e023,
                      last: 0x1e024,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e026,
                      last: 0x1e02a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e130,
                      last: 0x1e136,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e2ec,
                      last: 0x1e2ef,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e8d0,
                      last: 0x1e8d6,};
         init
     },
     {
         let mut init =
             interval{first: 0x1e944,
                      last: 0x1e94a,};
         init
     },
     {
         let mut init =
             interval{first: 0xe0100,
                      last: 0xe01ef,};
         init
     }];
pub static mut foldCase: [convertStruct; 192] =
    [{
         let mut init =
             convertStruct{rangeStart: 0x41,
                           rangeEnd: 0x5a,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xb5,
                           rangeEnd: 0xb5,
                           step: -(1),
                           offset: 775,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xc0,
                           rangeEnd: 0xd6,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xd8,
                           rangeEnd: 0xde,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x100,
                           rangeEnd: 0x12e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x132,
                           rangeEnd: 0x136,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x139,
                           rangeEnd: 0x147,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x14a,
                           rangeEnd: 0x176,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x178,
                           rangeEnd: 0x178,
                           step: -(1),
                           offset: -(121),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x179,
                           rangeEnd: 0x17d,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x17f,
                           rangeEnd: 0x17f,
                           step: -(1),
                           offset: -(268),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x181,
                           rangeEnd: 0x181,
                           step: -(1),
                           offset: 210,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x182,
                           rangeEnd: 0x184,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x186,
                           rangeEnd: 0x186,
                           step: -(1),
                           offset: 206,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x187,
                           rangeEnd: 0x187,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x189,
                           rangeEnd: 0x18a,
                           step: 1,
                           offset: 205,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18b,
                           rangeEnd: 0x18b,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18e,
                           rangeEnd: 0x18e,
                           step: -(1),
                           offset: 79,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x18f,
                           rangeEnd: 0x18f,
                           step: -(1),
                           offset: 202,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x190,
                           rangeEnd: 0x190,
                           step: -(1),
                           offset: 203,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x191,
                           rangeEnd: 0x191,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x193,
                           rangeEnd: 0x193,
                           step: -(1),
                           offset: 205,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x194,
                           rangeEnd: 0x194,
                           step: -(1),
                           offset: 207,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x196,
                           rangeEnd: 0x196,
                           step: -(1),
                           offset: 211,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x197,
                           rangeEnd: 0x197,
                           step: -(1),
                           offset: 209,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x198,
                           rangeEnd: 0x198,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19c,
                           rangeEnd: 0x19c,
                           step: -(1),
                           offset: 211,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19d,
                           rangeEnd: 0x19d,
                           step: -(1),
                           offset: 213,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x19f,
                           rangeEnd: 0x19f,
                           step: -(1),
                           offset: 214,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a0,
                           rangeEnd: 0x1a4,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a6,
                           rangeEnd: 0x1a6,
                           step: -(1),
                           offset: 218,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a7,
                           rangeEnd: 0x1a7,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1a9,
                           rangeEnd: 0x1a9,
                           step: -(1),
                           offset: 218,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ac,
                           rangeEnd: 0x1ac,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ae,
                           rangeEnd: 0x1ae,
                           step: -(1),
                           offset: 218,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1af,
                           rangeEnd: 0x1af,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b1,
                           rangeEnd: 0x1b2,
                           step: 1,
                           offset: 217,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b3,
                           rangeEnd: 0x1b5,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b7,
                           rangeEnd: 0x1b7,
                           step: -(1),
                           offset: 219,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1b8,
                           rangeEnd: 0x1bc,
                           step: 4,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c4,
                           rangeEnd: 0x1c4,
                           step: -(1),
                           offset: 2,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c5,
                           rangeEnd: 0x1c5,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c7,
                           rangeEnd: 0x1c7,
                           step: -(1),
                           offset: 2,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c8,
                           rangeEnd: 0x1c8,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ca,
                           rangeEnd: 0x1ca,
                           step: -(1),
                           offset: 2,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cb,
                           rangeEnd: 0x1db,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1de,
                           rangeEnd: 0x1ee,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f1,
                           rangeEnd: 0x1f1,
                           step: -(1),
                           offset: 2,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f2,
                           rangeEnd: 0x1f4,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f6,
                           rangeEnd: 0x1f6,
                           step: -(1),
                           offset: -(97),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f7,
                           rangeEnd: 0x1f7,
                           step: -(1),
                           offset: -(56),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f8,
                           rangeEnd: 0x21e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x220,
                           rangeEnd: 0x220,
                           step: -(1),
                           offset: -(130),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x222,
                           rangeEnd: 0x232,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23a,
                           rangeEnd: 0x23a,
                           step: -(1),
                           offset: 10795,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23b,
                           rangeEnd: 0x23b,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23d,
                           rangeEnd: 0x23d,
                           step: -(1),
                           offset: -(163),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x23e,
                           rangeEnd: 0x23e,
                           step: -(1),
                           offset: 10792,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x241,
                           rangeEnd: 0x241,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x243,
                           rangeEnd: 0x243,
                           step: -(1),
                           offset: -(195),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x244,
                           rangeEnd: 0x244,
                           step: -(1),
                           offset: 69,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x245,
                           rangeEnd: 0x245,
                           step: -(1),
                           offset: 71,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x246,
                           rangeEnd: 0x24e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x345,
                           rangeEnd: 0x345,
                           step: -(1),
                           offset: 116,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x370,
                           rangeEnd: 0x372,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x376,
                           rangeEnd: 0x376,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x37f,
                           rangeEnd: 0x37f,
                           step: -(1),
                           offset: 116,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x386,
                           rangeEnd: 0x386,
                           step: -(1),
                           offset: 38,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x388,
                           rangeEnd: 0x38a,
                           step: 1,
                           offset: 37,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x38c,
                           rangeEnd: 0x38c,
                           step: -(1),
                           offset: 64,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x38e,
                           rangeEnd: 0x38f,
                           step: 1,
                           offset: 63,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x391,
                           rangeEnd: 0x3a1,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3a3,
                           rangeEnd: 0x3ab,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3c2,
                           rangeEnd: 0x3c2,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3cf,
                           rangeEnd: 0x3cf,
                           step: -(1),
                           offset: 8,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d0,
                           rangeEnd: 0x3d0,
                           step: -(1),
                           offset: -(30),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d1,
                           rangeEnd: 0x3d1,
                           step: -(1),
                           offset: -(25),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d5,
                           rangeEnd: 0x3d5,
                           step: -(1),
                           offset: -(15),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d6,
                           rangeEnd: 0x3d6,
                           step: -(1),
                           offset: -(22),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3d8,
                           rangeEnd: 0x3ee,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f0,
                           rangeEnd: 0x3f0,
                           step: -(1),
                           offset: -(54),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f1,
                           rangeEnd: 0x3f1,
                           step: -(1),
                           offset: -(48),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f4,
                           rangeEnd: 0x3f4,
                           step: -(1),
                           offset: -(60),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f5,
                           rangeEnd: 0x3f5,
                           step: -(1),
                           offset: -(64),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f7,
                           rangeEnd: 0x3f7,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3f9,
                           rangeEnd: 0x3f9,
                           step: -(1),
                           offset: -(7),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3fa,
                           rangeEnd: 0x3fa,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x3fd,
                           rangeEnd: 0x3ff,
                           step: 1,
                           offset: -(130),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x400,
                           rangeEnd: 0x40f,
                           step: 1,
                           offset: 80,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x410,
                           rangeEnd: 0x42f,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x460,
                           rangeEnd: 0x480,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x48a,
                           rangeEnd: 0x4be,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c0,
                           rangeEnd: 0x4c0,
                           step: -(1),
                           offset: 15,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4c1,
                           rangeEnd: 0x4cd,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x4d0,
                           rangeEnd: 0x52e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x531,
                           rangeEnd: 0x556,
                           step: 1,
                           offset: 48,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10a0,
                           rangeEnd: 0x10c5,
                           step: 1,
                           offset: 7264,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10c7,
                           rangeEnd: 0x10cd,
                           step: 6,
                           offset: 7264,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x13f8,
                           rangeEnd: 0x13fd,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c80,
                           rangeEnd: 0x1c80,
                           step: -(1),
                           offset: -(6222),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c81,
                           rangeEnd: 0x1c81,
                           step: -(1),
                           offset: -(6221),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c82,
                           rangeEnd: 0x1c82,
                           step: -(1),
                           offset: -(6212),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c83,
                           rangeEnd: 0x1c84,
                           step: 1,
                           offset: -(6210),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c85,
                           rangeEnd: 0x1c85,
                           step: -(1),
                           offset: -(6211),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c86,
                           rangeEnd: 0x1c86,
                           step: -(1),
                           offset: -(6204),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c87,
                           rangeEnd: 0x1c87,
                           step: -(1),
                           offset: -(6180),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c88,
                           rangeEnd: 0x1c88,
                           step: -(1),
                           offset: 35267,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1c90,
                           rangeEnd: 0x1cba,
                           step: 1,
                           offset: -(3008),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1cbd,
                           rangeEnd: 0x1cbf,
                           step: 1,
                           offset: -(3008),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e00,
                           rangeEnd: 0x1e94,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e9b,
                           rangeEnd: 0x1e9b,
                           step: -(1),
                           offset: -(58),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e9e,
                           rangeEnd: 0x1e9e,
                           step: -(1),
                           offset: -(7615),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ea0,
                           rangeEnd: 0x1efe,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f08,
                           rangeEnd: 0x1f0f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f18,
                           rangeEnd: 0x1f1d,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f28,
                           rangeEnd: 0x1f2f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f38,
                           rangeEnd: 0x1f3f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f48,
                           rangeEnd: 0x1f4d,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f59,
                           rangeEnd: 0x1f5f,
                           step: 2,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f68,
                           rangeEnd: 0x1f6f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f88,
                           rangeEnd: 0x1f8f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1f98,
                           rangeEnd: 0x1f9f,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fa8,
                           rangeEnd: 0x1faf,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fb8,
                           rangeEnd: 0x1fb9,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fba,
                           rangeEnd: 0x1fbb,
                           step: 1,
                           offset: -(74),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fbc,
                           rangeEnd: 0x1fbc,
                           step: -(1),
                           offset: -(9),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fbe,
                           rangeEnd: 0x1fbe,
                           step: -(1),
                           offset: -(7173),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fc8,
                           rangeEnd: 0x1fcb,
                           step: 1,
                           offset: -(86),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fcc,
                           rangeEnd: 0x1fcc,
                           step: -(1),
                           offset: -(9),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fd8,
                           rangeEnd: 0x1fd9,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fda,
                           rangeEnd: 0x1fdb,
                           step: 1,
                           offset: -(100),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fe8,
                           rangeEnd: 0x1fe9,
                           step: 1,
                           offset: -(8),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fea,
                           rangeEnd: 0x1feb,
                           step: 1,
                           offset: -(112),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1fec,
                           rangeEnd: 0x1fec,
                           step: -(1),
                           offset: -(7),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ff8,
                           rangeEnd: 0x1ff9,
                           step: 1,
                           offset: -(128),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ffa,
                           rangeEnd: 0x1ffb,
                           step: 1,
                           offset: -(126),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1ffc,
                           rangeEnd: 0x1ffc,
                           step: -(1),
                           offset: -(9),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2126,
                           rangeEnd: 0x2126,
                           step: -(1),
                           offset: -(7517),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x212a,
                           rangeEnd: 0x212a,
                           step: -(1),
                           offset: -(8383),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x212b,
                           rangeEnd: 0x212b,
                           step: -(1),
                           offset: -(8262),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2132,
                           rangeEnd: 0x2132,
                           step: -(1),
                           offset: 28,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2160,
                           rangeEnd: 0x216f,
                           step: 1,
                           offset: 16,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2183,
                           rangeEnd: 0x2183,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x24b6,
                           rangeEnd: 0x24cf,
                           step: 1,
                           offset: 26,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c00,
                           rangeEnd: 0x2c2e,
                           step: 1,
                           offset: 48,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c60,
                           rangeEnd: 0x2c60,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c62,
                           rangeEnd: 0x2c62,
                           step: -(1),
                           offset: -(10743),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c63,
                           rangeEnd: 0x2c63,
                           step: -(1),
                           offset: -(3814),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c64,
                           rangeEnd: 0x2c64,
                           step: -(1),
                           offset: -(10727),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c67,
                           rangeEnd: 0x2c6b,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6d,
                           rangeEnd: 0x2c6d,
                           step: -(1),
                           offset: -(10780),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6e,
                           rangeEnd: 0x2c6e,
                           step: -(1),
                           offset: -(10749),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c6f,
                           rangeEnd: 0x2c6f,
                           step: -(1),
                           offset: -(10783),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c70,
                           rangeEnd: 0x2c70,
                           step: -(1),
                           offset: -(10782),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c72,
                           rangeEnd: 0x2c75,
                           step: 3,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c7e,
                           rangeEnd: 0x2c7f,
                           step: 1,
                           offset: -(10815),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2c80,
                           rangeEnd: 0x2ce2,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2ceb,
                           rangeEnd: 0x2ced,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x2cf2,
                           rangeEnd: 0xa640,
                           step: 31054,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa642,
                           rangeEnd: 0xa66c,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa680,
                           rangeEnd: 0xa69a,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa722,
                           rangeEnd: 0xa72e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa732,
                           rangeEnd: 0xa76e,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa779,
                           rangeEnd: 0xa77b,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77d,
                           rangeEnd: 0xa77d,
                           step: -(1),
                           offset: -(35332),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa77e,
                           rangeEnd: 0xa786,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78b,
                           rangeEnd: 0xa78b,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa78d,
                           rangeEnd: 0xa78d,
                           step: -(1),
                           offset: -(42280),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa790,
                           rangeEnd: 0xa792,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa796,
                           rangeEnd: 0xa7a8,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7aa,
                           rangeEnd: 0xa7aa,
                           step: -(1),
                           offset: -(42308),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ab,
                           rangeEnd: 0xa7ab,
                           step: -(1),
                           offset: -(42319),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ac,
                           rangeEnd: 0xa7ac,
                           step: -(1),
                           offset: -(42315),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ad,
                           rangeEnd: 0xa7ad,
                           step: -(1),
                           offset: -(42305),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7ae,
                           rangeEnd: 0xa7ae,
                           step: -(1),
                           offset: -(42308),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b0,
                           rangeEnd: 0xa7b0,
                           step: -(1),
                           offset: -(42258),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b1,
                           rangeEnd: 0xa7b1,
                           step: -(1),
                           offset: -(42282),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b2,
                           rangeEnd: 0xa7b2,
                           step: -(1),
                           offset: -(42261),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b3,
                           rangeEnd: 0xa7b3,
                           step: -(1),
                           offset: 928,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7b4,
                           rangeEnd: 0xa7be,
                           step: 2,
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c2,
                           rangeEnd: 0xa7c2,
                           step: -(1),
                           offset: 1,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c4,
                           rangeEnd: 0xa7c4,
                           step: -(1),
                           offset: -(48),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c5,
                           rangeEnd: 0xa7c5,
                           step: -(1),
                           offset: -(42307),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xa7c6,
                           rangeEnd: 0xa7c6,
                           step: -(1),
                           offset: -(35384),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xab70,
                           rangeEnd: 0xabbf,
                           step: 1,
                           offset: -(38864),};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0xff21,
                           rangeEnd: 0xff3a,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10400,
                           rangeEnd: 0x10427,
                           step: 1,
                           offset: 40,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x104b0,
                           rangeEnd: 0x104d3,
                           step: 1,
                           offset: 40,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x10c80,
                           rangeEnd: 0x10cb2,
                           step: 1,
                           offset: 64,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x118a0,
                           rangeEnd: 0x118bf,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x16e40,
                           rangeEnd: 0x16e5f,
                           step: 1,
                           offset: 32,};
         init
     },
     {
         let mut init =
             convertStruct{rangeStart: 0x1e900,
                           rangeEnd: 0x1e921,
                           step: 1,
                           offset: 34,};
         init
     }];
pub static mut doublewidth: [interval; 113] =
    [{
         let mut init =
             interval{first: 0x1100,
                      last: 0x115f,};
         init
     },
     {
         let mut init =
             interval{first: 0x231a,
                      last: 0x231b,};
         init
     },
     {
         let mut init =
             interval{first: 0x2329,
                      last: 0x232a,};
         init
     },
     {
         let mut init =
             interval{first: 0x23e9,
                      last: 0x23ec,};
         init
     },
     {
         let mut init =
             interval{first: 0x23f0,
                      last: 0x23f0,};
         init
     },
     {
         let mut init =
             interval{first: 0x23f3,
                      last: 0x23f3,};
         init
     },
     {
         let mut init =
             interval{first: 0x25fd,
                      last: 0x25fe,};
         init
     },
     {
         let mut init =
             interval{first: 0x2614,
                      last: 0x2615,};
         init
     },
     {
         let mut init =
             interval{first: 0x2648,
                      last: 0x2653,};
         init
     },
     {
         let mut init =
             interval{first: 0x267f,
                      last: 0x267f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2693,
                      last: 0x2693,};
         init
     },
     {
         let mut init =
             interval{first: 0x26a1,
                      last: 0x26a1,};
         init
     },
     {
         let mut init =
             interval{first: 0x26aa,
                      last: 0x26ab,};
         init
     },
     {
         let mut init =
             interval{first: 0x26bd,
                      last: 0x26be,};
         init
     },
     {
         let mut init =
             interval{first: 0x26c4,
                      last: 0x26c5,};
         init
     },
     {
         let mut init =
             interval{first: 0x26ce,
                      last: 0x26ce,};
         init
     },
     {
         let mut init =
             interval{first: 0x26d4,
                      last: 0x26d4,};
         init
     },
     {
         let mut init =
             interval{first: 0x26ea,
                      last: 0x26ea,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f2,
                      last: 0x26f3,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f5,
                      last: 0x26f5,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fa,
                      last: 0x26fa,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fd,
                      last: 0x26fd,};
         init
     },
     {
         let mut init =
             interval{first: 0x2705,
                      last: 0x2705,};
         init
     },
     {
         let mut init =
             interval{first: 0x270a,
                      last: 0x270b,};
         init
     },
     {
         let mut init =
             interval{first: 0x2728,
                      last: 0x2728,};
         init
     },
     {
         let mut init =
             interval{first: 0x274c,
                      last: 0x274c,};
         init
     },
     {
         let mut init =
             interval{first: 0x274e,
                      last: 0x274e,};
         init
     },
     {
         let mut init =
             interval{first: 0x2753,
                      last: 0x2755,};
         init
     },
     {
         let mut init =
             interval{first: 0x2757,
                      last: 0x2757,};
         init
     },
     {
         let mut init =
             interval{first: 0x2795,
                      last: 0x2797,};
         init
     },
     {
         let mut init =
             interval{first: 0x27b0,
                      last: 0x27b0,};
         init
     },
     {
         let mut init =
             interval{first: 0x27bf,
                      last: 0x27bf,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b1b,
                      last: 0x2b1c,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b50,
                      last: 0x2b50,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b55,
                      last: 0x2b55,};
         init
     },
     {
         let mut init =
             interval{first: 0x2e80,
                      last: 0x2e99,};
         init
     },
     {
         let mut init =
             interval{first: 0x2e9b,
                      last: 0x2ef3,};
         init
     },
     {
         let mut init =
             interval{first: 0x2f00,
                      last: 0x2fd5,};
         init
     },
     {
         let mut init =
             interval{first: 0x2ff0,
                      last: 0x2ffb,};
         init
     },
     {
         let mut init =
             interval{first: 0x3000,
                      last: 0x303e,};
         init
     },
     {
         let mut init =
             interval{first: 0x3041,
                      last: 0x3096,};
         init
     },
     {
         let mut init =
             interval{first: 0x3099,
                      last: 0x30ff,};
         init
     },
     {
         let mut init =
             interval{first: 0x3105,
                      last: 0x312f,};
         init
     },
     {
         let mut init =
             interval{first: 0x3131,
                      last: 0x318e,};
         init
     },
     {
         let mut init =
             interval{first: 0x3190,
                      last: 0x31ba,};
         init
     },
     {
         let mut init =
             interval{first: 0x31c0,
                      last: 0x31e3,};
         init
     },
     {
         let mut init =
             interval{first: 0x31f0,
                      last: 0x321e,};
         init
     },
     {
         let mut init =
             interval{first: 0x3220,
                      last: 0x3247,};
         init
     },
     {
         let mut init =
             interval{first: 0x3250,
                      last: 0x4dbf,};
         init
     },
     {
         let mut init =
             interval{first: 0x4e00,
                      last: 0xa48c,};
         init
     },
     {
         let mut init =
             interval{first: 0xa490,
                      last: 0xa4c6,};
         init
     },
     {
         let mut init =
             interval{first: 0xa960,
                      last: 0xa97c,};
         init
     },
     {
         let mut init =
             interval{first: 0xac00,
                      last: 0xd7a3,};
         init
     },
     {
         let mut init =
             interval{first: 0xf900,
                      last: 0xfaff,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe10,
                      last: 0xfe19,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe30,
                      last: 0xfe52,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe54,
                      last: 0xfe66,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe68,
                      last: 0xfe6b,};
         init
     },
     {
         let mut init =
             interval{first: 0xff01,
                      last: 0xff60,};
         init
     },
     {
         let mut init =
             interval{first: 0xffe0,
                      last: 0xffe6,};
         init
     },
     {
         let mut init =
             interval{first: 0x16fe0,
                      last: 0x16fe3,};
         init
     },
     {
         let mut init =
             interval{first: 0x17000,
                      last: 0x187f7,};
         init
     },
     {
         let mut init =
             interval{first: 0x18800,
                      last: 0x18af2,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b000,
                      last: 0x1b11e,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b150,
                      last: 0x1b152,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b164,
                      last: 0x1b167,};
         init
     },
     {
         let mut init =
             interval{first: 0x1b170,
                      last: 0x1b2fb,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f004,
                      last: 0x1f004,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f0cf,
                      last: 0x1f0cf,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f18e,
                      last: 0x1f18e,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f191,
                      last: 0x1f19a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f200,
                      last: 0x1f202,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f210,
                      last: 0x1f23b,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f240,
                      last: 0x1f248,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f250,
                      last: 0x1f251,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f260,
                      last: 0x1f265,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f300,
                      last: 0x1f320,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f32d,
                      last: 0x1f335,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f337,
                      last: 0x1f37c,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f37e,
                      last: 0x1f393,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3a0,
                      last: 0x1f3ca,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3cf,
                      last: 0x1f3d3,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3e0,
                      last: 0x1f3f0,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f4,
                      last: 0x1f3f4,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f8,
                      last: 0x1f43e,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f440,
                      last: 0x1f440,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f442,
                      last: 0x1f4fc,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f4ff,
                      last: 0x1f53d,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f54b,
                      last: 0x1f54e,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f550,
                      last: 0x1f567,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f57a,
                      last: 0x1f57a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f595,
                      last: 0x1f596,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a4,
                      last: 0x1f5a4,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5fb,
                      last: 0x1f64f,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f680,
                      last: 0x1f6c5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6cc,
                      last: 0x1f6cc,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6d0,
                      last: 0x1f6d2,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6d5,
                      last: 0x1f6d5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6eb,
                      last: 0x1f6ec,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f4,
                      last: 0x1f6fa,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f7e0,
                      last: 0x1f7eb,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f90d,
                      last: 0x1f971,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f973,
                      last: 0x1f976,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f97a,
                      last: 0x1f9a2,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9a5,
                      last: 0x1f9aa,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9ae,
                      last: 0x1f9ca,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9cd,
                      last: 0x1f9ff,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa70,
                      last: 0x1fa73,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa78,
                      last: 0x1fa7a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa80,
                      last: 0x1fa82,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa90,
                      last: 0x1fa95,};
         init
     },
     {
         let mut init =
             interval{first: 0x20000,
                      last: 0x2fffd,};
         init
     },
     {
         let mut init =
             interval{first: 0x30000,
                      last: 0x3fffd,};
         init
     }];
pub static mut ambiguous: [interval; 179] =
    [{
         let mut init =
             interval{first: 0xa1,
                      last: 0xa1,};
         init
     },
     {
         let mut init =
             interval{first: 0xa4,
                      last: 0xa4,};
         init
     },
     {
         let mut init =
             interval{first: 0xa7,
                      last: 0xa8,};
         init
     },
     {
         let mut init =
             interval{first: 0xaa,
                      last: 0xaa,};
         init
     },
     {
         let mut init =
             interval{first: 0xad,
                      last: 0xae,};
         init
     },
     {
         let mut init =
             interval{first: 0xb0,
                      last: 0xb4,};
         init
     },
     {
         let mut init =
             interval{first: 0xb6,
                      last: 0xba,};
         init
     },
     {
         let mut init =
             interval{first: 0xbc,
                      last: 0xbf,};
         init
     },
     {
         let mut init =
             interval{first: 0xc6,
                      last: 0xc6,};
         init
     },
     {
         let mut init =
             interval{first: 0xd0,
                      last: 0xd0,};
         init
     },
     {
         let mut init =
             interval{first: 0xd7,
                      last: 0xd8,};
         init
     },
     {
         let mut init =
             interval{first: 0xde,
                      last: 0xe1,};
         init
     },
     {
         let mut init =
             interval{first: 0xe6,
                      last: 0xe6,};
         init
     },
     {
         let mut init =
             interval{first: 0xe8,
                      last: 0xea,};
         init
     },
     {
         let mut init =
             interval{first: 0xec,
                      last: 0xed,};
         init
     },
     {
         let mut init =
             interval{first: 0xf0,
                      last: 0xf0,};
         init
     },
     {
         let mut init =
             interval{first: 0xf2,
                      last: 0xf3,};
         init
     },
     {
         let mut init =
             interval{first: 0xf7,
                      last: 0xfa,};
         init
     },
     {
         let mut init =
             interval{first: 0xfc,
                      last: 0xfc,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe,
                      last: 0xfe,};
         init
     },
     {
         let mut init =
             interval{first: 0x101,
                      last: 0x101,};
         init
     },
     {
         let mut init =
             interval{first: 0x111,
                      last: 0x111,};
         init
     },
     {
         let mut init =
             interval{first: 0x113,
                      last: 0x113,};
         init
     },
     {
         let mut init =
             interval{first: 0x11b,
                      last: 0x11b,};
         init
     },
     {
         let mut init =
             interval{first: 0x126,
                      last: 0x127,};
         init
     },
     {
         let mut init =
             interval{first: 0x12b,
                      last: 0x12b,};
         init
     },
     {
         let mut init =
             interval{first: 0x131,
                      last: 0x133,};
         init
     },
     {
         let mut init =
             interval{first: 0x138,
                      last: 0x138,};
         init
     },
     {
         let mut init =
             interval{first: 0x13f,
                      last: 0x142,};
         init
     },
     {
         let mut init =
             interval{first: 0x144,
                      last: 0x144,};
         init
     },
     {
         let mut init =
             interval{first: 0x148,
                      last: 0x14b,};
         init
     },
     {
         let mut init =
             interval{first: 0x14d,
                      last: 0x14d,};
         init
     },
     {
         let mut init =
             interval{first: 0x152,
                      last: 0x153,};
         init
     },
     {
         let mut init =
             interval{first: 0x166,
                      last: 0x167,};
         init
     },
     {
         let mut init =
             interval{first: 0x16b,
                      last: 0x16b,};
         init
     },
     {
         let mut init =
             interval{first: 0x1ce,
                      last: 0x1ce,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d0,
                      last: 0x1d0,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d2,
                      last: 0x1d2,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d4,
                      last: 0x1d4,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d6,
                      last: 0x1d6,};
         init
     },
     {
         let mut init =
             interval{first: 0x1d8,
                      last: 0x1d8,};
         init
     },
     {
         let mut init =
             interval{first: 0x1da,
                      last: 0x1da,};
         init
     },
     {
         let mut init =
             interval{first: 0x1dc,
                      last: 0x1dc,};
         init
     },
     {
         let mut init =
             interval{first: 0x251,
                      last: 0x251,};
         init
     },
     {
         let mut init =
             interval{first: 0x261,
                      last: 0x261,};
         init
     },
     {
         let mut init =
             interval{first: 0x2c4,
                      last: 0x2c4,};
         init
     },
     {
         let mut init =
             interval{first: 0x2c7,
                      last: 0x2c7,};
         init
     },
     {
         let mut init =
             interval{first: 0x2c9,
                      last: 0x2cb,};
         init
     },
     {
         let mut init =
             interval{first: 0x2cd,
                      last: 0x2cd,};
         init
     },
     {
         let mut init =
             interval{first: 0x2d0,
                      last: 0x2d0,};
         init
     },
     {
         let mut init =
             interval{first: 0x2d8,
                      last: 0x2db,};
         init
     },
     {
         let mut init =
             interval{first: 0x2dd,
                      last: 0x2dd,};
         init
     },
     {
         let mut init =
             interval{first: 0x2df,
                      last: 0x2df,};
         init
     },
     {
         let mut init =
             interval{first: 0x300,
                      last: 0x36f,};
         init
     },
     {
         let mut init =
             interval{first: 0x391,
                      last: 0x3a1,};
         init
     },
     {
         let mut init =
             interval{first: 0x3a3,
                      last: 0x3a9,};
         init
     },
     {
         let mut init =
             interval{first: 0x3b1,
                      last: 0x3c1,};
         init
     },
     {
         let mut init =
             interval{first: 0x3c3,
                      last: 0x3c9,};
         init
     },
     {
         let mut init =
             interval{first: 0x401,
                      last: 0x401,};
         init
     },
     {
         let mut init =
             interval{first: 0x410,
                      last: 0x44f,};
         init
     },
     {
         let mut init =
             interval{first: 0x451,
                      last: 0x451,};
         init
     },
     {
         let mut init =
             interval{first: 0x2010,
                      last: 0x2010,};
         init
     },
     {
         let mut init =
             interval{first: 0x2013,
                      last: 0x2016,};
         init
     },
     {
         let mut init =
             interval{first: 0x2018,
                      last: 0x2019,};
         init
     },
     {
         let mut init =
             interval{first: 0x201c,
                      last: 0x201d,};
         init
     },
     {
         let mut init =
             interval{first: 0x2020,
                      last: 0x2022,};
         init
     },
     {
         let mut init =
             interval{first: 0x2024,
                      last: 0x2027,};
         init
     },
     {
         let mut init =
             interval{first: 0x2030,
                      last: 0x2030,};
         init
     },
     {
         let mut init =
             interval{first: 0x2032,
                      last: 0x2033,};
         init
     },
     {
         let mut init =
             interval{first: 0x2035,
                      last: 0x2035,};
         init
     },
     {
         let mut init =
             interval{first: 0x203b,
                      last: 0x203b,};
         init
     },
     {
         let mut init =
             interval{first: 0x203e,
                      last: 0x203e,};
         init
     },
     {
         let mut init =
             interval{first: 0x2074,
                      last: 0x2074,};
         init
     },
     {
         let mut init =
             interval{first: 0x207f,
                      last: 0x207f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2081,
                      last: 0x2084,};
         init
     },
     {
         let mut init =
             interval{first: 0x20ac,
                      last: 0x20ac,};
         init
     },
     {
         let mut init =
             interval{first: 0x2103,
                      last: 0x2103,};
         init
     },
     {
         let mut init =
             interval{first: 0x2105,
                      last: 0x2105,};
         init
     },
     {
         let mut init =
             interval{first: 0x2109,
                      last: 0x2109,};
         init
     },
     {
         let mut init =
             interval{first: 0x2113,
                      last: 0x2113,};
         init
     },
     {
         let mut init =
             interval{first: 0x2116,
                      last: 0x2116,};
         init
     },
     {
         let mut init =
             interval{first: 0x2121,
                      last: 0x2122,};
         init
     },
     {
         let mut init =
             interval{first: 0x2126,
                      last: 0x2126,};
         init
     },
     {
         let mut init =
             interval{first: 0x212b,
                      last: 0x212b,};
         init
     },
     {
         let mut init =
             interval{first: 0x2153,
                      last: 0x2154,};
         init
     },
     {
         let mut init =
             interval{first: 0x215b,
                      last: 0x215e,};
         init
     },
     {
         let mut init =
             interval{first: 0x2160,
                      last: 0x216b,};
         init
     },
     {
         let mut init =
             interval{first: 0x2170,
                      last: 0x2179,};
         init
     },
     {
         let mut init =
             interval{first: 0x2189,
                      last: 0x2189,};
         init
     },
     {
         let mut init =
             interval{first: 0x2190,
                      last: 0x2199,};
         init
     },
     {
         let mut init =
             interval{first: 0x21b8,
                      last: 0x21b9,};
         init
     },
     {
         let mut init =
             interval{first: 0x21d2,
                      last: 0x21d2,};
         init
     },
     {
         let mut init =
             interval{first: 0x21d4,
                      last: 0x21d4,};
         init
     },
     {
         let mut init =
             interval{first: 0x21e7,
                      last: 0x21e7,};
         init
     },
     {
         let mut init =
             interval{first: 0x2200,
                      last: 0x2200,};
         init
     },
     {
         let mut init =
             interval{first: 0x2202,
                      last: 0x2203,};
         init
     },
     {
         let mut init =
             interval{first: 0x2207,
                      last: 0x2208,};
         init
     },
     {
         let mut init =
             interval{first: 0x220b,
                      last: 0x220b,};
         init
     },
     {
         let mut init =
             interval{first: 0x220f,
                      last: 0x220f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2211,
                      last: 0x2211,};
         init
     },
     {
         let mut init =
             interval{first: 0x2215,
                      last: 0x2215,};
         init
     },
     {
         let mut init =
             interval{first: 0x221a,
                      last: 0x221a,};
         init
     },
     {
         let mut init =
             interval{first: 0x221d,
                      last: 0x2220,};
         init
     },
     {
         let mut init =
             interval{first: 0x2223,
                      last: 0x2223,};
         init
     },
     {
         let mut init =
             interval{first: 0x2225,
                      last: 0x2225,};
         init
     },
     {
         let mut init =
             interval{first: 0x2227,
                      last: 0x222c,};
         init
     },
     {
         let mut init =
             interval{first: 0x222e,
                      last: 0x222e,};
         init
     },
     {
         let mut init =
             interval{first: 0x2234,
                      last: 0x2237,};
         init
     },
     {
         let mut init =
             interval{first: 0x223c,
                      last: 0x223d,};
         init
     },
     {
         let mut init =
             interval{first: 0x2248,
                      last: 0x2248,};
         init
     },
     {
         let mut init =
             interval{first: 0x224c,
                      last: 0x224c,};
         init
     },
     {
         let mut init =
             interval{first: 0x2252,
                      last: 0x2252,};
         init
     },
     {
         let mut init =
             interval{first: 0x2260,
                      last: 0x2261,};
         init
     },
     {
         let mut init =
             interval{first: 0x2264,
                      last: 0x2267,};
         init
     },
     {
         let mut init =
             interval{first: 0x226a,
                      last: 0x226b,};
         init
     },
     {
         let mut init =
             interval{first: 0x226e,
                      last: 0x226f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2282,
                      last: 0x2283,};
         init
     },
     {
         let mut init =
             interval{first: 0x2286,
                      last: 0x2287,};
         init
     },
     {
         let mut init =
             interval{first: 0x2295,
                      last: 0x2295,};
         init
     },
     {
         let mut init =
             interval{first: 0x2299,
                      last: 0x2299,};
         init
     },
     {
         let mut init =
             interval{first: 0x22a5,
                      last: 0x22a5,};
         init
     },
     {
         let mut init =
             interval{first: 0x22bf,
                      last: 0x22bf,};
         init
     },
     {
         let mut init =
             interval{first: 0x2312,
                      last: 0x2312,};
         init
     },
     {
         let mut init =
             interval{first: 0x2460,
                      last: 0x24e9,};
         init
     },
     {
         let mut init =
             interval{first: 0x24eb,
                      last: 0x254b,};
         init
     },
     {
         let mut init =
             interval{first: 0x2550,
                      last: 0x2573,};
         init
     },
     {
         let mut init =
             interval{first: 0x2580,
                      last: 0x258f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2592,
                      last: 0x2595,};
         init
     },
     {
         let mut init =
             interval{first: 0x25a0,
                      last: 0x25a1,};
         init
     },
     {
         let mut init =
             interval{first: 0x25a3,
                      last: 0x25a9,};
         init
     },
     {
         let mut init =
             interval{first: 0x25b2,
                      last: 0x25b3,};
         init
     },
     {
         let mut init =
             interval{first: 0x25b6,
                      last: 0x25b7,};
         init
     },
     {
         let mut init =
             interval{first: 0x25bc,
                      last: 0x25bd,};
         init
     },
     {
         let mut init =
             interval{first: 0x25c0,
                      last: 0x25c1,};
         init
     },
     {
         let mut init =
             interval{first: 0x25c6,
                      last: 0x25c8,};
         init
     },
     {
         let mut init =
             interval{first: 0x25cb,
                      last: 0x25cb,};
         init
     },
     {
         let mut init =
             interval{first: 0x25ce,
                      last: 0x25d1,};
         init
     },
     {
         let mut init =
             interval{first: 0x25e2,
                      last: 0x25e5,};
         init
     },
     {
         let mut init =
             interval{first: 0x25ef,
                      last: 0x25ef,};
         init
     },
     {
         let mut init =
             interval{first: 0x2605,
                      last: 0x2606,};
         init
     },
     {
         let mut init =
             interval{first: 0x2609,
                      last: 0x2609,};
         init
     },
     {
         let mut init =
             interval{first: 0x260e,
                      last: 0x260f,};
         init
     },
     {
         let mut init =
             interval{first: 0x261c,
                      last: 0x261c,};
         init
     },
     {
         let mut init =
             interval{first: 0x261e,
                      last: 0x261e,};
         init
     },
     {
         let mut init =
             interval{first: 0x2640,
                      last: 0x2640,};
         init
     },
     {
         let mut init =
             interval{first: 0x2642,
                      last: 0x2642,};
         init
     },
     {
         let mut init =
             interval{first: 0x2660,
                      last: 0x2661,};
         init
     },
     {
         let mut init =
             interval{first: 0x2663,
                      last: 0x2665,};
         init
     },
     {
         let mut init =
             interval{first: 0x2667,
                      last: 0x266a,};
         init
     },
     {
         let mut init =
             interval{first: 0x266c,
                      last: 0x266d,};
         init
     },
     {
         let mut init =
             interval{first: 0x266f,
                      last: 0x266f,};
         init
     },
     {
         let mut init =
             interval{first: 0x269e,
                      last: 0x269f,};
         init
     },
     {
         let mut init =
             interval{first: 0x26bf,
                      last: 0x26bf,};
         init
     },
     {
         let mut init =
             interval{first: 0x26c6,
                      last: 0x26cd,};
         init
     },
     {
         let mut init =
             interval{first: 0x26cf,
                      last: 0x26d3,};
         init
     },
     {
         let mut init =
             interval{first: 0x26d5,
                      last: 0x26e1,};
         init
     },
     {
         let mut init =
             interval{first: 0x26e3,
                      last: 0x26e3,};
         init
     },
     {
         let mut init =
             interval{first: 0x26e8,
                      last: 0x26e9,};
         init
     },
     {
         let mut init =
             interval{first: 0x26eb,
                      last: 0x26f1,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f4,
                      last: 0x26f4,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f6,
                      last: 0x26f9,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fb,
                      last: 0x26fc,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fe,
                      last: 0x26ff,};
         init
     },
     {
         let mut init =
             interval{first: 0x273d,
                      last: 0x273d,};
         init
     },
     {
         let mut init =
             interval{first: 0x2776,
                      last: 0x277f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b56,
                      last: 0x2b59,};
         init
     },
     {
         let mut init =
             interval{first: 0x3248,
                      last: 0x324f,};
         init
     },
     {
         let mut init =
             interval{first: 0xe000,
                      last: 0xf8ff,};
         init
     },
     {
         let mut init =
             interval{first: 0xfe00,
                      last: 0xfe0f,};
         init
     },
     {
         let mut init =
             interval{first: 0xfffd,
                      last: 0xfffd,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f100,
                      last: 0x1f10a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f110,
                      last: 0x1f12d,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f130,
                      last: 0x1f169,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f170,
                      last: 0x1f18d,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f18f,
                      last: 0x1f190,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f19b,
                      last: 0x1f1ac,};
         init
     },
     {
         let mut init =
             interval{first: 0xe0100,
                      last: 0xe01ef,};
         init
     },
     {
         let mut init =
             interval{first: 0xf0000,
                      last: 0xffffd,};
         init
     },
     {
         let mut init =
             interval{first: 0x100000,
                      last: 0x10fffd,};
         init
     }];
pub static mut emoji_all: [interval; 151] =
    [{
         let mut init =
             interval{first: 0x23,
                      last: 0x23,};
         init
     },
     {
         let mut init =
             interval{first: 0x2a,
                      last: 0x2a,};
         init
     },
     {
         let mut init =
             interval{first: 0x30,
                      last: 0x39,};
         init
     },
     {
         let mut init =
             interval{first: 0xa9,
                      last: 0xa9,};
         init
     },
     {
         let mut init =
             interval{first: 0xae,
                      last: 0xae,};
         init
     },
     {
         let mut init =
             interval{first: 0x203c,
                      last: 0x203c,};
         init
     },
     {
         let mut init =
             interval{first: 0x2049,
                      last: 0x2049,};
         init
     },
     {
         let mut init =
             interval{first: 0x2122,
                      last: 0x2122,};
         init
     },
     {
         let mut init =
             interval{first: 0x2139,
                      last: 0x2139,};
         init
     },
     {
         let mut init =
             interval{first: 0x2194,
                      last: 0x2199,};
         init
     },
     {
         let mut init =
             interval{first: 0x21a9,
                      last: 0x21aa,};
         init
     },
     {
         let mut init =
             interval{first: 0x231a,
                      last: 0x231b,};
         init
     },
     {
         let mut init =
             interval{first: 0x2328,
                      last: 0x2328,};
         init
     },
     {
         let mut init =
             interval{first: 0x23cf,
                      last: 0x23cf,};
         init
     },
     {
         let mut init =
             interval{first: 0x23e9,
                      last: 0x23f3,};
         init
     },
     {
         let mut init =
             interval{first: 0x23f8,
                      last: 0x23fa,};
         init
     },
     {
         let mut init =
             interval{first: 0x24c2,
                      last: 0x24c2,};
         init
     },
     {
         let mut init =
             interval{first: 0x25aa,
                      last: 0x25ab,};
         init
     },
     {
         let mut init =
             interval{first: 0x25b6,
                      last: 0x25b6,};
         init
     },
     {
         let mut init =
             interval{first: 0x25c0,
                      last: 0x25c0,};
         init
     },
     {
         let mut init =
             interval{first: 0x25fb,
                      last: 0x25fe,};
         init
     },
     {
         let mut init =
             interval{first: 0x2600,
                      last: 0x2604,};
         init
     },
     {
         let mut init =
             interval{first: 0x260e,
                      last: 0x260e,};
         init
     },
     {
         let mut init =
             interval{first: 0x2611,
                      last: 0x2611,};
         init
     },
     {
         let mut init =
             interval{first: 0x2614,
                      last: 0x2615,};
         init
     },
     {
         let mut init =
             interval{first: 0x2618,
                      last: 0x2618,};
         init
     },
     {
         let mut init =
             interval{first: 0x261d,
                      last: 0x261d,};
         init
     },
     {
         let mut init =
             interval{first: 0x2620,
                      last: 0x2620,};
         init
     },
     {
         let mut init =
             interval{first: 0x2622,
                      last: 0x2623,};
         init
     },
     {
         let mut init =
             interval{first: 0x2626,
                      last: 0x2626,};
         init
     },
     {
         let mut init =
             interval{first: 0x262a,
                      last: 0x262a,};
         init
     },
     {
         let mut init =
             interval{first: 0x262e,
                      last: 0x262f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2638,
                      last: 0x263a,};
         init
     },
     {
         let mut init =
             interval{first: 0x2640,
                      last: 0x2640,};
         init
     },
     {
         let mut init =
             interval{first: 0x2642,
                      last: 0x2642,};
         init
     },
     {
         let mut init =
             interval{first: 0x2648,
                      last: 0x2653,};
         init
     },
     {
         let mut init =
             interval{first: 0x265f,
                      last: 0x2660,};
         init
     },
     {
         let mut init =
             interval{first: 0x2663,
                      last: 0x2663,};
         init
     },
     {
         let mut init =
             interval{first: 0x2665,
                      last: 0x2666,};
         init
     },
     {
         let mut init =
             interval{first: 0x2668,
                      last: 0x2668,};
         init
     },
     {
         let mut init =
             interval{first: 0x267b,
                      last: 0x267b,};
         init
     },
     {
         let mut init =
             interval{first: 0x267e,
                      last: 0x267f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2692,
                      last: 0x2697,};
         init
     },
     {
         let mut init =
             interval{first: 0x2699,
                      last: 0x2699,};
         init
     },
     {
         let mut init =
             interval{first: 0x269b,
                      last: 0x269c,};
         init
     },
     {
         let mut init =
             interval{first: 0x26a0,
                      last: 0x26a1,};
         init
     },
     {
         let mut init =
             interval{first: 0x26aa,
                      last: 0x26ab,};
         init
     },
     {
         let mut init =
             interval{first: 0x26b0,
                      last: 0x26b1,};
         init
     },
     {
         let mut init =
             interval{first: 0x26bd,
                      last: 0x26be,};
         init
     },
     {
         let mut init =
             interval{first: 0x26c4,
                      last: 0x26c5,};
         init
     },
     {
         let mut init =
             interval{first: 0x26c8,
                      last: 0x26c8,};
         init
     },
     {
         let mut init =
             interval{first: 0x26ce,
                      last: 0x26cf,};
         init
     },
     {
         let mut init =
             interval{first: 0x26d1,
                      last: 0x26d1,};
         init
     },
     {
         let mut init =
             interval{first: 0x26d3,
                      last: 0x26d4,};
         init
     },
     {
         let mut init =
             interval{first: 0x26e9,
                      last: 0x26ea,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f0,
                      last: 0x26f5,};
         init
     },
     {
         let mut init =
             interval{first: 0x26f7,
                      last: 0x26fa,};
         init
     },
     {
         let mut init =
             interval{first: 0x26fd,
                      last: 0x26fd,};
         init
     },
     {
         let mut init =
             interval{first: 0x2702,
                      last: 0x2702,};
         init
     },
     {
         let mut init =
             interval{first: 0x2705,
                      last: 0x2705,};
         init
     },
     {
         let mut init =
             interval{first: 0x2708,
                      last: 0x270d,};
         init
     },
     {
         let mut init =
             interval{first: 0x270f,
                      last: 0x270f,};
         init
     },
     {
         let mut init =
             interval{first: 0x2712,
                      last: 0x2712,};
         init
     },
     {
         let mut init =
             interval{first: 0x2714,
                      last: 0x2714,};
         init
     },
     {
         let mut init =
             interval{first: 0x2716,
                      last: 0x2716,};
         init
     },
     {
         let mut init =
             interval{first: 0x271d,
                      last: 0x271d,};
         init
     },
     {
         let mut init =
             interval{first: 0x2721,
                      last: 0x2721,};
         init
     },
     {
         let mut init =
             interval{first: 0x2728,
                      last: 0x2728,};
         init
     },
     {
         let mut init =
             interval{first: 0x2733,
                      last: 0x2734,};
         init
     },
     {
         let mut init =
             interval{first: 0x2744,
                      last: 0x2744,};
         init
     },
     {
         let mut init =
             interval{first: 0x2747,
                      last: 0x2747,};
         init
     },
     {
         let mut init =
             interval{first: 0x274c,
                      last: 0x274c,};
         init
     },
     {
         let mut init =
             interval{first: 0x274e,
                      last: 0x274e,};
         init
     },
     {
         let mut init =
             interval{first: 0x2753,
                      last: 0x2755,};
         init
     },
     {
         let mut init =
             interval{first: 0x2757,
                      last: 0x2757,};
         init
     },
     {
         let mut init =
             interval{first: 0x2763,
                      last: 0x2764,};
         init
     },
     {
         let mut init =
             interval{first: 0x2795,
                      last: 0x2797,};
         init
     },
     {
         let mut init =
             interval{first: 0x27a1,
                      last: 0x27a1,};
         init
     },
     {
         let mut init =
             interval{first: 0x27b0,
                      last: 0x27b0,};
         init
     },
     {
         let mut init =
             interval{first: 0x27bf,
                      last: 0x27bf,};
         init
     },
     {
         let mut init =
             interval{first: 0x2934,
                      last: 0x2935,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b05,
                      last: 0x2b07,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b1b,
                      last: 0x2b1c,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b50,
                      last: 0x2b50,};
         init
     },
     {
         let mut init =
             interval{first: 0x2b55,
                      last: 0x2b55,};
         init
     },
     {
         let mut init =
             interval{first: 0x3030,
                      last: 0x3030,};
         init
     },
     {
         let mut init =
             interval{first: 0x303d,
                      last: 0x303d,};
         init
     },
     {
         let mut init =
             interval{first: 0x3297,
                      last: 0x3297,};
         init
     },
     {
         let mut init =
             interval{first: 0x3299,
                      last: 0x3299,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f004,
                      last: 0x1f004,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f0cf,
                      last: 0x1f0cf,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f170,
                      last: 0x1f171,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f17e,
                      last: 0x1f17f,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f18e,
                      last: 0x1f18e,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f191,
                      last: 0x1f19a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f1e6,
                      last: 0x1f1ff,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f201,
                      last: 0x1f202,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f21a,
                      last: 0x1f21a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f22f,
                      last: 0x1f22f,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f232,
                      last: 0x1f23a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f250,
                      last: 0x1f251,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f300,
                      last: 0x1f321,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f324,
                      last: 0x1f393,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f396,
                      last: 0x1f397,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f399,
                      last: 0x1f39b,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f39e,
                      last: 0x1f3f0,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f3,
                      last: 0x1f3f5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f7,
                      last: 0x1f4fd,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f4ff,
                      last: 0x1f53d,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f549,
                      last: 0x1f54e,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f550,
                      last: 0x1f567,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f56f,
                      last: 0x1f570,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f573,
                      last: 0x1f57a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f587,
                      last: 0x1f587,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f58a,
                      last: 0x1f58d,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f590,
                      last: 0x1f590,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f595,
                      last: 0x1f596,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a4,
                      last: 0x1f5a5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a8,
                      last: 0x1f5a8,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5b1,
                      last: 0x1f5b2,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5bc,
                      last: 0x1f5bc,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5c2,
                      last: 0x1f5c4,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5d1,
                      last: 0x1f5d3,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5dc,
                      last: 0x1f5de,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e1,
                      last: 0x1f5e1,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e3,
                      last: 0x1f5e3,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e8,
                      last: 0x1f5e8,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5ef,
                      last: 0x1f5ef,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5f3,
                      last: 0x1f5f3,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5fa,
                      last: 0x1f64f,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f680,
                      last: 0x1f6c5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6cb,
                      last: 0x1f6d2,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6d5,
                      last: 0x1f6d5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6e0,
                      last: 0x1f6e5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6e9,
                      last: 0x1f6e9,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6eb,
                      last: 0x1f6ec,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f0,
                      last: 0x1f6f0,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f3,
                      last: 0x1f6fa,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f7e0,
                      last: 0x1f7eb,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f90d,
                      last: 0x1f93a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f93c,
                      last: 0x1f945,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f947,
                      last: 0x1f971,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f973,
                      last: 0x1f976,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f97a,
                      last: 0x1f9a2,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9a5,
                      last: 0x1f9aa,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9ae,
                      last: 0x1f9ca,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f9cd,
                      last: 0x1f9ff,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa70,
                      last: 0x1fa73,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa78,
                      last: 0x1fa7a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa80,
                      last: 0x1fa82,};
         init
     },
     {
         let mut init =
             interval{first: 0x1fa90,
                      last: 0x1fa95,};
         init
     }];
pub static mut emoji_width: [interval; 39] =
    [{
         let mut init =
             interval{first: 0x1f1e6,
                      last: 0x1f1ff,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f321,
                      last: 0x1f321,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f324,
                      last: 0x1f32c,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f336,
                      last: 0x1f336,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f37d,
                      last: 0x1f37d,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f396,
                      last: 0x1f397,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f399,
                      last: 0x1f39b,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f39e,
                      last: 0x1f39f,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3cb,
                      last: 0x1f3ce,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3d4,
                      last: 0x1f3df,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f3,
                      last: 0x1f3f5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f3f7,
                      last: 0x1f3f7,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f43f,
                      last: 0x1f43f,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f441,
                      last: 0x1f441,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f4fd,
                      last: 0x1f4fd,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f549,
                      last: 0x1f54a,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f56f,
                      last: 0x1f570,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f573,
                      last: 0x1f579,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f587,
                      last: 0x1f587,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f58a,
                      last: 0x1f58d,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f590,
                      last: 0x1f590,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a5,
                      last: 0x1f5a5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5a8,
                      last: 0x1f5a8,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5b1,
                      last: 0x1f5b2,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5bc,
                      last: 0x1f5bc,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5c2,
                      last: 0x1f5c4,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5d1,
                      last: 0x1f5d3,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5dc,
                      last: 0x1f5de,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e1,
                      last: 0x1f5e1,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e3,
                      last: 0x1f5e3,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5e8,
                      last: 0x1f5e8,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5ef,
                      last: 0x1f5ef,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5f3,
                      last: 0x1f5f3,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f5fa,
                      last: 0x1f5fa,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6cb,
                      last: 0x1f6cf,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6e0,
                      last: 0x1f6e5,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6e9,
                      last: 0x1f6e9,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f0,
                      last: 0x1f6f0,};
         init
     },
     {
         let mut init =
             interval{first: 0x1f6f3,
                      last: 0x1f6f3,};
         init
     }];
