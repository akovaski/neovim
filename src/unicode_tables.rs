use crate::*;

pub static mut toLower: [convertStruct; 172] = [
    convertStruct {
        rangeStart: 0x41,
        rangeEnd: 0x5a,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0xc0,
        rangeEnd: 0xd6,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0xd8,
        rangeEnd: 0xde,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x100,
        rangeEnd: 0x12e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x130,
        rangeEnd: 0x130,
        step: -(1),
        offset: -(199),
    },
    convertStruct {
        rangeStart: 0x132,
        rangeEnd: 0x136,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x139,
        rangeEnd: 0x147,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x14a,
        rangeEnd: 0x176,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x178,
        rangeEnd: 0x178,
        step: -(1),
        offset: -(121),
    },
    convertStruct {
        rangeStart: 0x179,
        rangeEnd: 0x17d,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x181,
        rangeEnd: 0x181,
        step: -(1),
        offset: 210,
    },
    convertStruct {
        rangeStart: 0x182,
        rangeEnd: 0x184,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x186,
        rangeEnd: 0x186,
        step: -(1),
        offset: 206,
    },
    convertStruct {
        rangeStart: 0x187,
        rangeEnd: 0x187,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x189,
        rangeEnd: 0x18a,
        step: 1,
        offset: 205,
    },
    convertStruct {
        rangeStart: 0x18b,
        rangeEnd: 0x18b,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x18e,
        rangeEnd: 0x18e,
        step: -(1),
        offset: 79,
    },
    convertStruct {
        rangeStart: 0x18f,
        rangeEnd: 0x18f,
        step: -(1),
        offset: 202,
    },
    convertStruct {
        rangeStart: 0x190,
        rangeEnd: 0x190,
        step: -(1),
        offset: 203,
    },
    convertStruct {
        rangeStart: 0x191,
        rangeEnd: 0x191,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x193,
        rangeEnd: 0x193,
        step: -(1),
        offset: 205,
    },
    convertStruct {
        rangeStart: 0x194,
        rangeEnd: 0x194,
        step: -(1),
        offset: 207,
    },
    convertStruct {
        rangeStart: 0x196,
        rangeEnd: 0x196,
        step: -(1),
        offset: 211,
    },
    convertStruct {
        rangeStart: 0x197,
        rangeEnd: 0x197,
        step: -(1),
        offset: 209,
    },
    convertStruct {
        rangeStart: 0x198,
        rangeEnd: 0x198,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x19c,
        rangeEnd: 0x19c,
        step: -(1),
        offset: 211,
    },
    convertStruct {
        rangeStart: 0x19d,
        rangeEnd: 0x19d,
        step: -(1),
        offset: 213,
    },
    convertStruct {
        rangeStart: 0x19f,
        rangeEnd: 0x19f,
        step: -(1),
        offset: 214,
    },
    convertStruct {
        rangeStart: 0x1a0,
        rangeEnd: 0x1a4,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1a6,
        rangeEnd: 0x1a6,
        step: -(1),
        offset: 218,
    },
    convertStruct {
        rangeStart: 0x1a7,
        rangeEnd: 0x1a7,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1a9,
        rangeEnd: 0x1a9,
        step: -(1),
        offset: 218,
    },
    convertStruct {
        rangeStart: 0x1ac,
        rangeEnd: 0x1ac,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1ae,
        rangeEnd: 0x1ae,
        step: -(1),
        offset: 218,
    },
    convertStruct {
        rangeStart: 0x1af,
        rangeEnd: 0x1af,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1b1,
        rangeEnd: 0x1b2,
        step: 1,
        offset: 217,
    },
    convertStruct {
        rangeStart: 0x1b3,
        rangeEnd: 0x1b5,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1b7,
        rangeEnd: 0x1b7,
        step: -(1),
        offset: 219,
    },
    convertStruct {
        rangeStart: 0x1b8,
        rangeEnd: 0x1bc,
        step: 4,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1c4,
        rangeEnd: 0x1c4,
        step: -(1),
        offset: 2,
    },
    convertStruct {
        rangeStart: 0x1c5,
        rangeEnd: 0x1c5,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1c7,
        rangeEnd: 0x1c7,
        step: -(1),
        offset: 2,
    },
    convertStruct {
        rangeStart: 0x1c8,
        rangeEnd: 0x1c8,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1ca,
        rangeEnd: 0x1ca,
        step: -(1),
        offset: 2,
    },
    convertStruct {
        rangeStart: 0x1cb,
        rangeEnd: 0x1db,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1de,
        rangeEnd: 0x1ee,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1f1,
        rangeEnd: 0x1f1,
        step: -(1),
        offset: 2,
    },
    convertStruct {
        rangeStart: 0x1f2,
        rangeEnd: 0x1f4,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1f6,
        rangeEnd: 0x1f6,
        step: -(1),
        offset: -(97),
    },
    convertStruct {
        rangeStart: 0x1f7,
        rangeEnd: 0x1f7,
        step: -(1),
        offset: -(56),
    },
    convertStruct {
        rangeStart: 0x1f8,
        rangeEnd: 0x21e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x220,
        rangeEnd: 0x220,
        step: -(1),
        offset: -(130),
    },
    convertStruct {
        rangeStart: 0x222,
        rangeEnd: 0x232,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x23a,
        rangeEnd: 0x23a,
        step: -(1),
        offset: 10795,
    },
    convertStruct {
        rangeStart: 0x23b,
        rangeEnd: 0x23b,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x23d,
        rangeEnd: 0x23d,
        step: -(1),
        offset: -(163),
    },
    convertStruct {
        rangeStart: 0x23e,
        rangeEnd: 0x23e,
        step: -(1),
        offset: 10792,
    },
    convertStruct {
        rangeStart: 0x241,
        rangeEnd: 0x241,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x243,
        rangeEnd: 0x243,
        step: -(1),
        offset: -(195),
    },
    convertStruct {
        rangeStart: 0x244,
        rangeEnd: 0x244,
        step: -(1),
        offset: 69,
    },
    convertStruct {
        rangeStart: 0x245,
        rangeEnd: 0x245,
        step: -(1),
        offset: 71,
    },
    convertStruct {
        rangeStart: 0x246,
        rangeEnd: 0x24e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x370,
        rangeEnd: 0x372,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x376,
        rangeEnd: 0x376,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x37f,
        rangeEnd: 0x37f,
        step: -(1),
        offset: 116,
    },
    convertStruct {
        rangeStart: 0x386,
        rangeEnd: 0x386,
        step: -(1),
        offset: 38,
    },
    convertStruct {
        rangeStart: 0x388,
        rangeEnd: 0x38a,
        step: 1,
        offset: 37,
    },
    convertStruct {
        rangeStart: 0x38c,
        rangeEnd: 0x38c,
        step: -(1),
        offset: 64,
    },
    convertStruct {
        rangeStart: 0x38e,
        rangeEnd: 0x38f,
        step: 1,
        offset: 63,
    },
    convertStruct {
        rangeStart: 0x391,
        rangeEnd: 0x3a1,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x3a3,
        rangeEnd: 0x3ab,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x3cf,
        rangeEnd: 0x3cf,
        step: -(1),
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x3d8,
        rangeEnd: 0x3ee,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x3f4,
        rangeEnd: 0x3f4,
        step: -(1),
        offset: -(60),
    },
    convertStruct {
        rangeStart: 0x3f7,
        rangeEnd: 0x3f7,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x3f9,
        rangeEnd: 0x3f9,
        step: -(1),
        offset: -(7),
    },
    convertStruct {
        rangeStart: 0x3fa,
        rangeEnd: 0x3fa,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x3fd,
        rangeEnd: 0x3ff,
        step: 1,
        offset: -(130),
    },
    convertStruct {
        rangeStart: 0x400,
        rangeEnd: 0x40f,
        step: 1,
        offset: 80,
    },
    convertStruct {
        rangeStart: 0x410,
        rangeEnd: 0x42f,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x460,
        rangeEnd: 0x480,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x48a,
        rangeEnd: 0x4be,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x4c0,
        rangeEnd: 0x4c0,
        step: -(1),
        offset: 15,
    },
    convertStruct {
        rangeStart: 0x4c1,
        rangeEnd: 0x4cd,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x4d0,
        rangeEnd: 0x52e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x531,
        rangeEnd: 0x556,
        step: 1,
        offset: 48,
    },
    convertStruct {
        rangeStart: 0x10a0,
        rangeEnd: 0x10c5,
        step: 1,
        offset: 7264,
    },
    convertStruct {
        rangeStart: 0x10c7,
        rangeEnd: 0x10cd,
        step: 6,
        offset: 7264,
    },
    convertStruct {
        rangeStart: 0x13a0,
        rangeEnd: 0x13ef,
        step: 1,
        offset: 38864,
    },
    convertStruct {
        rangeStart: 0x13f0,
        rangeEnd: 0x13f5,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1c90,
        rangeEnd: 0x1cba,
        step: 1,
        offset: -(3008),
    },
    convertStruct {
        rangeStart: 0x1cbd,
        rangeEnd: 0x1cbf,
        step: 1,
        offset: -(3008),
    },
    convertStruct {
        rangeStart: 0x1e00,
        rangeEnd: 0x1e94,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1e9e,
        rangeEnd: 0x1e9e,
        step: -(1),
        offset: -(7615),
    },
    convertStruct {
        rangeStart: 0x1ea0,
        rangeEnd: 0x1efe,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1f08,
        rangeEnd: 0x1f0f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f18,
        rangeEnd: 0x1f1d,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f28,
        rangeEnd: 0x1f2f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f38,
        rangeEnd: 0x1f3f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f48,
        rangeEnd: 0x1f4d,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f59,
        rangeEnd: 0x1f5f,
        step: 2,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f68,
        rangeEnd: 0x1f6f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f88,
        rangeEnd: 0x1f8f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f98,
        rangeEnd: 0x1f9f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fa8,
        rangeEnd: 0x1faf,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fb8,
        rangeEnd: 0x1fb9,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fba,
        rangeEnd: 0x1fbb,
        step: 1,
        offset: -(74),
    },
    convertStruct {
        rangeStart: 0x1fbc,
        rangeEnd: 0x1fbc,
        step: -(1),
        offset: -(9),
    },
    convertStruct {
        rangeStart: 0x1fc8,
        rangeEnd: 0x1fcb,
        step: 1,
        offset: -(86),
    },
    convertStruct {
        rangeStart: 0x1fcc,
        rangeEnd: 0x1fcc,
        step: -(1),
        offset: -(9),
    },
    convertStruct {
        rangeStart: 0x1fd8,
        rangeEnd: 0x1fd9,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fda,
        rangeEnd: 0x1fdb,
        step: 1,
        offset: -(100),
    },
    convertStruct {
        rangeStart: 0x1fe8,
        rangeEnd: 0x1fe9,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fea,
        rangeEnd: 0x1feb,
        step: 1,
        offset: -(112),
    },
    convertStruct {
        rangeStart: 0x1fec,
        rangeEnd: 0x1fec,
        step: -(1),
        offset: -(7),
    },
    convertStruct {
        rangeStart: 0x1ff8,
        rangeEnd: 0x1ff9,
        step: 1,
        offset: -(128),
    },
    convertStruct {
        rangeStart: 0x1ffa,
        rangeEnd: 0x1ffb,
        step: 1,
        offset: -(126),
    },
    convertStruct {
        rangeStart: 0x1ffc,
        rangeEnd: 0x1ffc,
        step: -(1),
        offset: -(9),
    },
    convertStruct {
        rangeStart: 0x2126,
        rangeEnd: 0x2126,
        step: -(1),
        offset: -(7517),
    },
    convertStruct {
        rangeStart: 0x212a,
        rangeEnd: 0x212a,
        step: -(1),
        offset: -(8383),
    },
    convertStruct {
        rangeStart: 0x212b,
        rangeEnd: 0x212b,
        step: -(1),
        offset: -(8262),
    },
    convertStruct {
        rangeStart: 0x2132,
        rangeEnd: 0x2132,
        step: -(1),
        offset: 28,
    },
    convertStruct {
        rangeStart: 0x2160,
        rangeEnd: 0x216f,
        step: 1,
        offset: 16,
    },
    convertStruct {
        rangeStart: 0x2183,
        rangeEnd: 0x2183,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x24b6,
        rangeEnd: 0x24cf,
        step: 1,
        offset: 26,
    },
    convertStruct {
        rangeStart: 0x2c00,
        rangeEnd: 0x2c2e,
        step: 1,
        offset: 48,
    },
    convertStruct {
        rangeStart: 0x2c60,
        rangeEnd: 0x2c60,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2c62,
        rangeEnd: 0x2c62,
        step: -(1),
        offset: -(10743),
    },
    convertStruct {
        rangeStart: 0x2c63,
        rangeEnd: 0x2c63,
        step: -(1),
        offset: -(3814),
    },
    convertStruct {
        rangeStart: 0x2c64,
        rangeEnd: 0x2c64,
        step: -(1),
        offset: -(10727),
    },
    convertStruct {
        rangeStart: 0x2c67,
        rangeEnd: 0x2c6b,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2c6d,
        rangeEnd: 0x2c6d,
        step: -(1),
        offset: -(10780),
    },
    convertStruct {
        rangeStart: 0x2c6e,
        rangeEnd: 0x2c6e,
        step: -(1),
        offset: -(10749),
    },
    convertStruct {
        rangeStart: 0x2c6f,
        rangeEnd: 0x2c6f,
        step: -(1),
        offset: -(10783),
    },
    convertStruct {
        rangeStart: 0x2c70,
        rangeEnd: 0x2c70,
        step: -(1),
        offset: -(10782),
    },
    convertStruct {
        rangeStart: 0x2c72,
        rangeEnd: 0x2c75,
        step: 3,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2c7e,
        rangeEnd: 0x2c7f,
        step: 1,
        offset: -(10815),
    },
    convertStruct {
        rangeStart: 0x2c80,
        rangeEnd: 0x2ce2,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2ceb,
        rangeEnd: 0x2ced,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2cf2,
        rangeEnd: 0xa640,
        step: 31054,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa642,
        rangeEnd: 0xa66c,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa680,
        rangeEnd: 0xa69a,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa722,
        rangeEnd: 0xa72e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa732,
        rangeEnd: 0xa76e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa779,
        rangeEnd: 0xa77b,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa77d,
        rangeEnd: 0xa77d,
        step: -(1),
        offset: -(35332),
    },
    convertStruct {
        rangeStart: 0xa77e,
        rangeEnd: 0xa786,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa78b,
        rangeEnd: 0xa78b,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa78d,
        rangeEnd: 0xa78d,
        step: -(1),
        offset: -(42280),
    },
    convertStruct {
        rangeStart: 0xa790,
        rangeEnd: 0xa792,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa796,
        rangeEnd: 0xa7a8,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa7aa,
        rangeEnd: 0xa7aa,
        step: -(1),
        offset: -(42308),
    },
    convertStruct {
        rangeStart: 0xa7ab,
        rangeEnd: 0xa7ab,
        step: -(1),
        offset: -(42319),
    },
    convertStruct {
        rangeStart: 0xa7ac,
        rangeEnd: 0xa7ac,
        step: -(1),
        offset: -(42315),
    },
    convertStruct {
        rangeStart: 0xa7ad,
        rangeEnd: 0xa7ad,
        step: -(1),
        offset: -(42305),
    },
    convertStruct {
        rangeStart: 0xa7ae,
        rangeEnd: 0xa7ae,
        step: -(1),
        offset: -(42308),
    },
    convertStruct {
        rangeStart: 0xa7b0,
        rangeEnd: 0xa7b0,
        step: -(1),
        offset: -(42258),
    },
    convertStruct {
        rangeStart: 0xa7b1,
        rangeEnd: 0xa7b1,
        step: -(1),
        offset: -(42282),
    },
    convertStruct {
        rangeStart: 0xa7b2,
        rangeEnd: 0xa7b2,
        step: -(1),
        offset: -(42261),
    },
    convertStruct {
        rangeStart: 0xa7b3,
        rangeEnd: 0xa7b3,
        step: -(1),
        offset: 928,
    },
    convertStruct {
        rangeStart: 0xa7b4,
        rangeEnd: 0xa7be,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa7c2,
        rangeEnd: 0xa7c2,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa7c4,
        rangeEnd: 0xa7c4,
        step: -(1),
        offset: -(48),
    },
    convertStruct {
        rangeStart: 0xa7c5,
        rangeEnd: 0xa7c5,
        step: -(1),
        offset: -(42307),
    },
    convertStruct {
        rangeStart: 0xa7c6,
        rangeEnd: 0xa7c6,
        step: -(1),
        offset: -(35384),
    },
    convertStruct {
        rangeStart: 0xff21,
        rangeEnd: 0xff3a,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x10400,
        rangeEnd: 0x10427,
        step: 1,
        offset: 40,
    },
    convertStruct {
        rangeStart: 0x104b0,
        rangeEnd: 0x104d3,
        step: 1,
        offset: 40,
    },
    convertStruct {
        rangeStart: 0x10c80,
        rangeEnd: 0x10cb2,
        step: 1,
        offset: 64,
    },
    convertStruct {
        rangeStart: 0x118a0,
        rangeEnd: 0x118bf,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x16e40,
        rangeEnd: 0x16e5f,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x1e900,
        rangeEnd: 0x1e921,
        step: 1,
        offset: 34,
    },
];
pub static mut toUpper: [convertStruct; 187] = [
    convertStruct {
        rangeStart: 0x61,
        rangeEnd: 0x7a,
        step: 1,
        offset: -(32),
    },
    convertStruct {
        rangeStart: 0xb5,
        rangeEnd: 0xb5,
        step: -(1),
        offset: 743,
    },
    convertStruct {
        rangeStart: 0xe0,
        rangeEnd: 0xf6,
        step: 1,
        offset: -(32),
    },
    convertStruct {
        rangeStart: 0xf8,
        rangeEnd: 0xfe,
        step: 1,
        offset: -(32),
    },
    convertStruct {
        rangeStart: 0xff,
        rangeEnd: 0xff,
        step: -(1),
        offset: 121,
    },
    convertStruct {
        rangeStart: 0x101,
        rangeEnd: 0x12f,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x131,
        rangeEnd: 0x131,
        step: -(1),
        offset: -(232),
    },
    convertStruct {
        rangeStart: 0x133,
        rangeEnd: 0x137,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x13a,
        rangeEnd: 0x148,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x14b,
        rangeEnd: 0x177,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x17a,
        rangeEnd: 0x17e,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x17f,
        rangeEnd: 0x17f,
        step: -(1),
        offset: -(300),
    },
    convertStruct {
        rangeStart: 0x180,
        rangeEnd: 0x180,
        step: -(1),
        offset: 195,
    },
    convertStruct {
        rangeStart: 0x183,
        rangeEnd: 0x185,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x188,
        rangeEnd: 0x18c,
        step: 4,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x192,
        rangeEnd: 0x192,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x195,
        rangeEnd: 0x195,
        step: -(1),
        offset: 97,
    },
    convertStruct {
        rangeStart: 0x199,
        rangeEnd: 0x199,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x19a,
        rangeEnd: 0x19a,
        step: -(1),
        offset: 163,
    },
    convertStruct {
        rangeStart: 0x19e,
        rangeEnd: 0x19e,
        step: -(1),
        offset: 130,
    },
    convertStruct {
        rangeStart: 0x1a1,
        rangeEnd: 0x1a5,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1a8,
        rangeEnd: 0x1ad,
        step: 5,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1b0,
        rangeEnd: 0x1b4,
        step: 4,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1b6,
        rangeEnd: 0x1b9,
        step: 3,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1bd,
        rangeEnd: 0x1bd,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1bf,
        rangeEnd: 0x1bf,
        step: -(1),
        offset: 56,
    },
    convertStruct {
        rangeStart: 0x1c5,
        rangeEnd: 0x1c5,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1c6,
        rangeEnd: 0x1c6,
        step: -(1),
        offset: -(2),
    },
    convertStruct {
        rangeStart: 0x1c8,
        rangeEnd: 0x1c8,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1c9,
        rangeEnd: 0x1c9,
        step: -(1),
        offset: -(2),
    },
    convertStruct {
        rangeStart: 0x1cb,
        rangeEnd: 0x1cb,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1cc,
        rangeEnd: 0x1cc,
        step: -(1),
        offset: -(2),
    },
    convertStruct {
        rangeStart: 0x1ce,
        rangeEnd: 0x1dc,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1dd,
        rangeEnd: 0x1dd,
        step: -(1),
        offset: -(79),
    },
    convertStruct {
        rangeStart: 0x1df,
        rangeEnd: 0x1ef,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1f2,
        rangeEnd: 0x1f2,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1f3,
        rangeEnd: 0x1f3,
        step: -(1),
        offset: -(2),
    },
    convertStruct {
        rangeStart: 0x1f5,
        rangeEnd: 0x1f9,
        step: 4,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1fb,
        rangeEnd: 0x21f,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x223,
        rangeEnd: 0x233,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x23c,
        rangeEnd: 0x23c,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x23f,
        rangeEnd: 0x240,
        step: 1,
        offset: 10815,
    },
    convertStruct {
        rangeStart: 0x242,
        rangeEnd: 0x247,
        step: 5,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x249,
        rangeEnd: 0x24f,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x250,
        rangeEnd: 0x250,
        step: -(1),
        offset: 10783,
    },
    convertStruct {
        rangeStart: 0x251,
        rangeEnd: 0x251,
        step: -(1),
        offset: 10780,
    },
    convertStruct {
        rangeStart: 0x252,
        rangeEnd: 0x252,
        step: -(1),
        offset: 10782,
    },
    convertStruct {
        rangeStart: 0x253,
        rangeEnd: 0x253,
        step: -(1),
        offset: -(210),
    },
    convertStruct {
        rangeStart: 0x254,
        rangeEnd: 0x254,
        step: -(1),
        offset: -(206),
    },
    convertStruct {
        rangeStart: 0x256,
        rangeEnd: 0x257,
        step: 1,
        offset: -(205),
    },
    convertStruct {
        rangeStart: 0x259,
        rangeEnd: 0x259,
        step: -(1),
        offset: -(202),
    },
    convertStruct {
        rangeStart: 0x25b,
        rangeEnd: 0x25b,
        step: -(1),
        offset: -(203),
    },
    convertStruct {
        rangeStart: 0x25c,
        rangeEnd: 0x25c,
        step: -(1),
        offset: 42319,
    },
    convertStruct {
        rangeStart: 0x260,
        rangeEnd: 0x260,
        step: -(1),
        offset: -(205),
    },
    convertStruct {
        rangeStart: 0x261,
        rangeEnd: 0x261,
        step: -(1),
        offset: 42315,
    },
    convertStruct {
        rangeStart: 0x263,
        rangeEnd: 0x263,
        step: -(1),
        offset: -(207),
    },
    convertStruct {
        rangeStart: 0x265,
        rangeEnd: 0x265,
        step: -(1),
        offset: 42280,
    },
    convertStruct {
        rangeStart: 0x266,
        rangeEnd: 0x266,
        step: -(1),
        offset: 42308,
    },
    convertStruct {
        rangeStart: 0x268,
        rangeEnd: 0x268,
        step: -(1),
        offset: -(209),
    },
    convertStruct {
        rangeStart: 0x269,
        rangeEnd: 0x269,
        step: -(1),
        offset: -(211),
    },
    convertStruct {
        rangeStart: 0x26a,
        rangeEnd: 0x26a,
        step: -(1),
        offset: 42308,
    },
    convertStruct {
        rangeStart: 0x26b,
        rangeEnd: 0x26b,
        step: -(1),
        offset: 10743,
    },
    convertStruct {
        rangeStart: 0x26c,
        rangeEnd: 0x26c,
        step: -(1),
        offset: 42305,
    },
    convertStruct {
        rangeStart: 0x26f,
        rangeEnd: 0x26f,
        step: -(1),
        offset: -(211),
    },
    convertStruct {
        rangeStart: 0x271,
        rangeEnd: 0x271,
        step: -(1),
        offset: 10749,
    },
    convertStruct {
        rangeStart: 0x272,
        rangeEnd: 0x272,
        step: -(1),
        offset: -(213),
    },
    convertStruct {
        rangeStart: 0x275,
        rangeEnd: 0x275,
        step: -(1),
        offset: -(214),
    },
    convertStruct {
        rangeStart: 0x27d,
        rangeEnd: 0x27d,
        step: -(1),
        offset: 10727,
    },
    convertStruct {
        rangeStart: 0x280,
        rangeEnd: 0x280,
        step: -(1),
        offset: -(218),
    },
    convertStruct {
        rangeStart: 0x282,
        rangeEnd: 0x282,
        step: -(1),
        offset: 42307,
    },
    convertStruct {
        rangeStart: 0x283,
        rangeEnd: 0x283,
        step: -(1),
        offset: -(218),
    },
    convertStruct {
        rangeStart: 0x287,
        rangeEnd: 0x287,
        step: -(1),
        offset: 42282,
    },
    convertStruct {
        rangeStart: 0x288,
        rangeEnd: 0x288,
        step: -(1),
        offset: -(218),
    },
    convertStruct {
        rangeStart: 0x289,
        rangeEnd: 0x289,
        step: -(1),
        offset: -(69),
    },
    convertStruct {
        rangeStart: 0x28a,
        rangeEnd: 0x28b,
        step: 1,
        offset: -(217),
    },
    convertStruct {
        rangeStart: 0x28c,
        rangeEnd: 0x28c,
        step: -(1),
        offset: -(71),
    },
    convertStruct {
        rangeStart: 0x292,
        rangeEnd: 0x292,
        step: -(1),
        offset: -(219),
    },
    convertStruct {
        rangeStart: 0x29d,
        rangeEnd: 0x29d,
        step: -(1),
        offset: 42261,
    },
    convertStruct {
        rangeStart: 0x29e,
        rangeEnd: 0x29e,
        step: -(1),
        offset: 42258,
    },
    convertStruct {
        rangeStart: 0x345,
        rangeEnd: 0x345,
        step: -(1),
        offset: 84,
    },
    convertStruct {
        rangeStart: 0x371,
        rangeEnd: 0x373,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x377,
        rangeEnd: 0x377,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x37b,
        rangeEnd: 0x37d,
        step: 1,
        offset: 130,
    },
    convertStruct {
        rangeStart: 0x3ac,
        rangeEnd: 0x3ac,
        step: -(1),
        offset: -(38),
    },
    convertStruct {
        rangeStart: 0x3ad,
        rangeEnd: 0x3af,
        step: 1,
        offset: -(37),
    },
    convertStruct {
        rangeStart: 0x3b1,
        rangeEnd: 0x3c1,
        step: 1,
        offset: -(32),
    },
    convertStruct {
        rangeStart: 0x3c2,
        rangeEnd: 0x3c2,
        step: -(1),
        offset: -(31),
    },
    convertStruct {
        rangeStart: 0x3c3,
        rangeEnd: 0x3cb,
        step: 1,
        offset: -(32),
    },
    convertStruct {
        rangeStart: 0x3cc,
        rangeEnd: 0x3cc,
        step: -(1),
        offset: -(64),
    },
    convertStruct {
        rangeStart: 0x3cd,
        rangeEnd: 0x3ce,
        step: 1,
        offset: -(63),
    },
    convertStruct {
        rangeStart: 0x3d0,
        rangeEnd: 0x3d0,
        step: -(1),
        offset: -(62),
    },
    convertStruct {
        rangeStart: 0x3d1,
        rangeEnd: 0x3d1,
        step: -(1),
        offset: -(57),
    },
    convertStruct {
        rangeStart: 0x3d5,
        rangeEnd: 0x3d5,
        step: -(1),
        offset: -(47),
    },
    convertStruct {
        rangeStart: 0x3d6,
        rangeEnd: 0x3d6,
        step: -(1),
        offset: -(54),
    },
    convertStruct {
        rangeStart: 0x3d7,
        rangeEnd: 0x3d7,
        step: -(1),
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x3d9,
        rangeEnd: 0x3ef,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x3f0,
        rangeEnd: 0x3f0,
        step: -(1),
        offset: -(86),
    },
    convertStruct {
        rangeStart: 0x3f1,
        rangeEnd: 0x3f1,
        step: -(1),
        offset: -(80),
    },
    convertStruct {
        rangeStart: 0x3f2,
        rangeEnd: 0x3f2,
        step: -(1),
        offset: 7,
    },
    convertStruct {
        rangeStart: 0x3f3,
        rangeEnd: 0x3f3,
        step: -(1),
        offset: -(116),
    },
    convertStruct {
        rangeStart: 0x3f5,
        rangeEnd: 0x3f5,
        step: -(1),
        offset: -(96),
    },
    convertStruct {
        rangeStart: 0x3f8,
        rangeEnd: 0x3fb,
        step: 3,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x430,
        rangeEnd: 0x44f,
        step: 1,
        offset: -(32),
    },
    convertStruct {
        rangeStart: 0x450,
        rangeEnd: 0x45f,
        step: 1,
        offset: -(80),
    },
    convertStruct {
        rangeStart: 0x461,
        rangeEnd: 0x481,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x48b,
        rangeEnd: 0x4bf,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x4c2,
        rangeEnd: 0x4ce,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x4cf,
        rangeEnd: 0x4cf,
        step: -(1),
        offset: -(15),
    },
    convertStruct {
        rangeStart: 0x4d1,
        rangeEnd: 0x52f,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x561,
        rangeEnd: 0x586,
        step: 1,
        offset: -(48),
    },
    convertStruct {
        rangeStart: 0x10d0,
        rangeEnd: 0x10fa,
        step: 1,
        offset: 3008,
    },
    convertStruct {
        rangeStart: 0x10fd,
        rangeEnd: 0x10ff,
        step: 1,
        offset: 3008,
    },
    convertStruct {
        rangeStart: 0x13f8,
        rangeEnd: 0x13fd,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1c80,
        rangeEnd: 0x1c80,
        step: -(1),
        offset: -(6254),
    },
    convertStruct {
        rangeStart: 0x1c81,
        rangeEnd: 0x1c81,
        step: -(1),
        offset: -(6253),
    },
    convertStruct {
        rangeStart: 0x1c82,
        rangeEnd: 0x1c82,
        step: -(1),
        offset: -(6244),
    },
    convertStruct {
        rangeStart: 0x1c83,
        rangeEnd: 0x1c84,
        step: 1,
        offset: -(6242),
    },
    convertStruct {
        rangeStart: 0x1c85,
        rangeEnd: 0x1c85,
        step: -(1),
        offset: -(6243),
    },
    convertStruct {
        rangeStart: 0x1c86,
        rangeEnd: 0x1c86,
        step: -(1),
        offset: -(6236),
    },
    convertStruct {
        rangeStart: 0x1c87,
        rangeEnd: 0x1c87,
        step: -(1),
        offset: -(6181),
    },
    convertStruct {
        rangeStart: 0x1c88,
        rangeEnd: 0x1c88,
        step: -(1),
        offset: 35266,
    },
    convertStruct {
        rangeStart: 0x1d79,
        rangeEnd: 0x1d79,
        step: -(1),
        offset: 35332,
    },
    convertStruct {
        rangeStart: 0x1d7d,
        rangeEnd: 0x1d7d,
        step: -(1),
        offset: 3814,
    },
    convertStruct {
        rangeStart: 0x1d8e,
        rangeEnd: 0x1d8e,
        step: -(1),
        offset: 35384,
    },
    convertStruct {
        rangeStart: 0x1e01,
        rangeEnd: 0x1e95,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1e9b,
        rangeEnd: 0x1e9b,
        step: -(1),
        offset: -(59),
    },
    convertStruct {
        rangeStart: 0x1ea1,
        rangeEnd: 0x1eff,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x1f00,
        rangeEnd: 0x1f07,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1f10,
        rangeEnd: 0x1f15,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1f20,
        rangeEnd: 0x1f27,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1f30,
        rangeEnd: 0x1f37,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1f40,
        rangeEnd: 0x1f45,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1f51,
        rangeEnd: 0x1f57,
        step: 2,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1f60,
        rangeEnd: 0x1f67,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1f70,
        rangeEnd: 0x1f71,
        step: 1,
        offset: 74,
    },
    convertStruct {
        rangeStart: 0x1f72,
        rangeEnd: 0x1f75,
        step: 1,
        offset: 86,
    },
    convertStruct {
        rangeStart: 0x1f76,
        rangeEnd: 0x1f77,
        step: 1,
        offset: 100,
    },
    convertStruct {
        rangeStart: 0x1f78,
        rangeEnd: 0x1f79,
        step: 1,
        offset: 128,
    },
    convertStruct {
        rangeStart: 0x1f7a,
        rangeEnd: 0x1f7b,
        step: 1,
        offset: 112,
    },
    convertStruct {
        rangeStart: 0x1f7c,
        rangeEnd: 0x1f7d,
        step: 1,
        offset: 126,
    },
    convertStruct {
        rangeStart: 0x1f80,
        rangeEnd: 0x1f87,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1f90,
        rangeEnd: 0x1f97,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1fa0,
        rangeEnd: 0x1fa7,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1fb0,
        rangeEnd: 0x1fb1,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1fb3,
        rangeEnd: 0x1fb3,
        step: -(1),
        offset: 9,
    },
    convertStruct {
        rangeStart: 0x1fbe,
        rangeEnd: 0x1fbe,
        step: -(1),
        offset: -(7205),
    },
    convertStruct {
        rangeStart: 0x1fc3,
        rangeEnd: 0x1fc3,
        step: -(1),
        offset: 9,
    },
    convertStruct {
        rangeStart: 0x1fd0,
        rangeEnd: 0x1fd1,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1fe0,
        rangeEnd: 0x1fe1,
        step: 1,
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x1fe5,
        rangeEnd: 0x1fe5,
        step: -(1),
        offset: 7,
    },
    convertStruct {
        rangeStart: 0x1ff3,
        rangeEnd: 0x1ff3,
        step: -(1),
        offset: 9,
    },
    convertStruct {
        rangeStart: 0x214e,
        rangeEnd: 0x214e,
        step: -(1),
        offset: -(28),
    },
    convertStruct {
        rangeStart: 0x2170,
        rangeEnd: 0x217f,
        step: 1,
        offset: -(16),
    },
    convertStruct {
        rangeStart: 0x2184,
        rangeEnd: 0x2184,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x24d0,
        rangeEnd: 0x24e9,
        step: 1,
        offset: -(26),
    },
    convertStruct {
        rangeStart: 0x2c30,
        rangeEnd: 0x2c5e,
        step: 1,
        offset: -(48),
    },
    convertStruct {
        rangeStart: 0x2c61,
        rangeEnd: 0x2c61,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x2c65,
        rangeEnd: 0x2c65,
        step: -(1),
        offset: -(10795),
    },
    convertStruct {
        rangeStart: 0x2c66,
        rangeEnd: 0x2c66,
        step: -(1),
        offset: -(10792),
    },
    convertStruct {
        rangeStart: 0x2c68,
        rangeEnd: 0x2c6c,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x2c73,
        rangeEnd: 0x2c76,
        step: 3,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x2c81,
        rangeEnd: 0x2ce3,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x2cec,
        rangeEnd: 0x2cee,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x2cf3,
        rangeEnd: 0x2cf3,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0x2d00,
        rangeEnd: 0x2d25,
        step: 1,
        offset: -(7264),
    },
    convertStruct {
        rangeStart: 0x2d27,
        rangeEnd: 0x2d2d,
        step: 6,
        offset: -(7264),
    },
    convertStruct {
        rangeStart: 0xa641,
        rangeEnd: 0xa66d,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa681,
        rangeEnd: 0xa69b,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa723,
        rangeEnd: 0xa72f,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa733,
        rangeEnd: 0xa76f,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa77a,
        rangeEnd: 0xa77c,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa77f,
        rangeEnd: 0xa787,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa78c,
        rangeEnd: 0xa791,
        step: 5,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa793,
        rangeEnd: 0xa793,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa794,
        rangeEnd: 0xa794,
        step: -(1),
        offset: 48,
    },
    convertStruct {
        rangeStart: 0xa797,
        rangeEnd: 0xa7a9,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa7b5,
        rangeEnd: 0xa7bf,
        step: 2,
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xa7c3,
        rangeEnd: 0xa7c3,
        step: -(1),
        offset: -(1),
    },
    convertStruct {
        rangeStart: 0xab53,
        rangeEnd: 0xab53,
        step: -(1),
        offset: -(928),
    },
    convertStruct {
        rangeStart: 0xab70,
        rangeEnd: 0xabbf,
        step: 1,
        offset: -(38864),
    },
    convertStruct {
        rangeStart: 0xff41,
        rangeEnd: 0xff5a,
        step: 1,
        offset: -(32),
    },
    convertStruct {
        rangeStart: 0x10428,
        rangeEnd: 0x1044f,
        step: 1,
        offset: -(40),
    },
    convertStruct {
        rangeStart: 0x104d8,
        rangeEnd: 0x104fb,
        step: 1,
        offset: -(40),
    },
    convertStruct {
        rangeStart: 0x10cc0,
        rangeEnd: 0x10cf2,
        step: 1,
        offset: -(64),
    },
    convertStruct {
        rangeStart: 0x118c0,
        rangeEnd: 0x118df,
        step: 1,
        offset: -(32),
    },
    convertStruct {
        rangeStart: 0x16e60,
        rangeEnd: 0x16e7f,
        step: 1,
        offset: -(32),
    },
    convertStruct {
        rangeStart: 0x1e922,
        rangeEnd: 0x1e943,
        step: 1,
        offset: -(34),
    },
];
pub static mut combining: [interval; 280] = [
    interval {
        first: 0x300,
        last: 0x36f,
    },
    interval {
        first: 0x483,
        last: 0x489,
    },
    interval {
        first: 0x591,
        last: 0x5bd,
    },
    interval {
        first: 0x5bf,
        last: 0x5bf,
    },
    interval {
        first: 0x5c1,
        last: 0x5c2,
    },
    interval {
        first: 0x5c4,
        last: 0x5c5,
    },
    interval {
        first: 0x5c7,
        last: 0x5c7,
    },
    interval {
        first: 0x610,
        last: 0x61a,
    },
    interval {
        first: 0x64b,
        last: 0x65f,
    },
    interval {
        first: 0x670,
        last: 0x670,
    },
    interval {
        first: 0x6d6,
        last: 0x6dc,
    },
    interval {
        first: 0x6df,
        last: 0x6e4,
    },
    interval {
        first: 0x6e7,
        last: 0x6e8,
    },
    interval {
        first: 0x6ea,
        last: 0x6ed,
    },
    interval {
        first: 0x711,
        last: 0x711,
    },
    interval {
        first: 0x730,
        last: 0x74a,
    },
    interval {
        first: 0x7a6,
        last: 0x7b0,
    },
    interval {
        first: 0x7eb,
        last: 0x7f3,
    },
    interval {
        first: 0x7fd,
        last: 0x7fd,
    },
    interval {
        first: 0x816,
        last: 0x819,
    },
    interval {
        first: 0x81b,
        last: 0x823,
    },
    interval {
        first: 0x825,
        last: 0x827,
    },
    interval {
        first: 0x829,
        last: 0x82d,
    },
    interval {
        first: 0x859,
        last: 0x85b,
    },
    interval {
        first: 0x8d3,
        last: 0x8e1,
    },
    interval {
        first: 0x8e3,
        last: 0x903,
    },
    interval {
        first: 0x93a,
        last: 0x93c,
    },
    interval {
        first: 0x93e,
        last: 0x94f,
    },
    interval {
        first: 0x951,
        last: 0x957,
    },
    interval {
        first: 0x962,
        last: 0x963,
    },
    interval {
        first: 0x981,
        last: 0x983,
    },
    interval {
        first: 0x9bc,
        last: 0x9bc,
    },
    interval {
        first: 0x9be,
        last: 0x9c4,
    },
    interval {
        first: 0x9c7,
        last: 0x9c8,
    },
    interval {
        first: 0x9cb,
        last: 0x9cd,
    },
    interval {
        first: 0x9d7,
        last: 0x9d7,
    },
    interval {
        first: 0x9e2,
        last: 0x9e3,
    },
    interval {
        first: 0x9fe,
        last: 0x9fe,
    },
    interval {
        first: 0xa01,
        last: 0xa03,
    },
    interval {
        first: 0xa3c,
        last: 0xa3c,
    },
    interval {
        first: 0xa3e,
        last: 0xa42,
    },
    interval {
        first: 0xa47,
        last: 0xa48,
    },
    interval {
        first: 0xa4b,
        last: 0xa4d,
    },
    interval {
        first: 0xa51,
        last: 0xa51,
    },
    interval {
        first: 0xa70,
        last: 0xa71,
    },
    interval {
        first: 0xa75,
        last: 0xa75,
    },
    interval {
        first: 0xa81,
        last: 0xa83,
    },
    interval {
        first: 0xabc,
        last: 0xabc,
    },
    interval {
        first: 0xabe,
        last: 0xac5,
    },
    interval {
        first: 0xac7,
        last: 0xac9,
    },
    interval {
        first: 0xacb,
        last: 0xacd,
    },
    interval {
        first: 0xae2,
        last: 0xae3,
    },
    interval {
        first: 0xafa,
        last: 0xaff,
    },
    interval {
        first: 0xb01,
        last: 0xb03,
    },
    interval {
        first: 0xb3c,
        last: 0xb3c,
    },
    interval {
        first: 0xb3e,
        last: 0xb44,
    },
    interval {
        first: 0xb47,
        last: 0xb48,
    },
    interval {
        first: 0xb4b,
        last: 0xb4d,
    },
    interval {
        first: 0xb56,
        last: 0xb57,
    },
    interval {
        first: 0xb62,
        last: 0xb63,
    },
    interval {
        first: 0xb82,
        last: 0xb82,
    },
    interval {
        first: 0xbbe,
        last: 0xbc2,
    },
    interval {
        first: 0xbc6,
        last: 0xbc8,
    },
    interval {
        first: 0xbca,
        last: 0xbcd,
    },
    interval {
        first: 0xbd7,
        last: 0xbd7,
    },
    interval {
        first: 0xc00,
        last: 0xc04,
    },
    interval {
        first: 0xc3e,
        last: 0xc44,
    },
    interval {
        first: 0xc46,
        last: 0xc48,
    },
    interval {
        first: 0xc4a,
        last: 0xc4d,
    },
    interval {
        first: 0xc55,
        last: 0xc56,
    },
    interval {
        first: 0xc62,
        last: 0xc63,
    },
    interval {
        first: 0xc81,
        last: 0xc83,
    },
    interval {
        first: 0xcbc,
        last: 0xcbc,
    },
    interval {
        first: 0xcbe,
        last: 0xcc4,
    },
    interval {
        first: 0xcc6,
        last: 0xcc8,
    },
    interval {
        first: 0xcca,
        last: 0xccd,
    },
    interval {
        first: 0xcd5,
        last: 0xcd6,
    },
    interval {
        first: 0xce2,
        last: 0xce3,
    },
    interval {
        first: 0xd00,
        last: 0xd03,
    },
    interval {
        first: 0xd3b,
        last: 0xd3c,
    },
    interval {
        first: 0xd3e,
        last: 0xd44,
    },
    interval {
        first: 0xd46,
        last: 0xd48,
    },
    interval {
        first: 0xd4a,
        last: 0xd4d,
    },
    interval {
        first: 0xd57,
        last: 0xd57,
    },
    interval {
        first: 0xd62,
        last: 0xd63,
    },
    interval {
        first: 0xd82,
        last: 0xd83,
    },
    interval {
        first: 0xdca,
        last: 0xdca,
    },
    interval {
        first: 0xdcf,
        last: 0xdd4,
    },
    interval {
        first: 0xdd6,
        last: 0xdd6,
    },
    interval {
        first: 0xdd8,
        last: 0xddf,
    },
    interval {
        first: 0xdf2,
        last: 0xdf3,
    },
    interval {
        first: 0xe31,
        last: 0xe31,
    },
    interval {
        first: 0xe34,
        last: 0xe3a,
    },
    interval {
        first: 0xe47,
        last: 0xe4e,
    },
    interval {
        first: 0xeb1,
        last: 0xeb1,
    },
    interval {
        first: 0xeb4,
        last: 0xebc,
    },
    interval {
        first: 0xec8,
        last: 0xecd,
    },
    interval {
        first: 0xf18,
        last: 0xf19,
    },
    interval {
        first: 0xf35,
        last: 0xf35,
    },
    interval {
        first: 0xf37,
        last: 0xf37,
    },
    interval {
        first: 0xf39,
        last: 0xf39,
    },
    interval {
        first: 0xf3e,
        last: 0xf3f,
    },
    interval {
        first: 0xf71,
        last: 0xf84,
    },
    interval {
        first: 0xf86,
        last: 0xf87,
    },
    interval {
        first: 0xf8d,
        last: 0xf97,
    },
    interval {
        first: 0xf99,
        last: 0xfbc,
    },
    interval {
        first: 0xfc6,
        last: 0xfc6,
    },
    interval {
        first: 0x102b,
        last: 0x103e,
    },
    interval {
        first: 0x1056,
        last: 0x1059,
    },
    interval {
        first: 0x105e,
        last: 0x1060,
    },
    interval {
        first: 0x1062,
        last: 0x1064,
    },
    interval {
        first: 0x1067,
        last: 0x106d,
    },
    interval {
        first: 0x1071,
        last: 0x1074,
    },
    interval {
        first: 0x1082,
        last: 0x108d,
    },
    interval {
        first: 0x108f,
        last: 0x108f,
    },
    interval {
        first: 0x109a,
        last: 0x109d,
    },
    interval {
        first: 0x135d,
        last: 0x135f,
    },
    interval {
        first: 0x1712,
        last: 0x1714,
    },
    interval {
        first: 0x1732,
        last: 0x1734,
    },
    interval {
        first: 0x1752,
        last: 0x1753,
    },
    interval {
        first: 0x1772,
        last: 0x1773,
    },
    interval {
        first: 0x17b4,
        last: 0x17d3,
    },
    interval {
        first: 0x17dd,
        last: 0x17dd,
    },
    interval {
        first: 0x180b,
        last: 0x180d,
    },
    interval {
        first: 0x1885,
        last: 0x1886,
    },
    interval {
        first: 0x18a9,
        last: 0x18a9,
    },
    interval {
        first: 0x1920,
        last: 0x192b,
    },
    interval {
        first: 0x1930,
        last: 0x193b,
    },
    interval {
        first: 0x1a17,
        last: 0x1a1b,
    },
    interval {
        first: 0x1a55,
        last: 0x1a5e,
    },
    interval {
        first: 0x1a60,
        last: 0x1a7c,
    },
    interval {
        first: 0x1a7f,
        last: 0x1a7f,
    },
    interval {
        first: 0x1ab0,
        last: 0x1abe,
    },
    interval {
        first: 0x1b00,
        last: 0x1b04,
    },
    interval {
        first: 0x1b34,
        last: 0x1b44,
    },
    interval {
        first: 0x1b6b,
        last: 0x1b73,
    },
    interval {
        first: 0x1b80,
        last: 0x1b82,
    },
    interval {
        first: 0x1ba1,
        last: 0x1bad,
    },
    interval {
        first: 0x1be6,
        last: 0x1bf3,
    },
    interval {
        first: 0x1c24,
        last: 0x1c37,
    },
    interval {
        first: 0x1cd0,
        last: 0x1cd2,
    },
    interval {
        first: 0x1cd4,
        last: 0x1ce8,
    },
    interval {
        first: 0x1ced,
        last: 0x1ced,
    },
    interval {
        first: 0x1cf4,
        last: 0x1cf4,
    },
    interval {
        first: 0x1cf7,
        last: 0x1cf9,
    },
    interval {
        first: 0x1dc0,
        last: 0x1df9,
    },
    interval {
        first: 0x1dfb,
        last: 0x1dff,
    },
    interval {
        first: 0x20d0,
        last: 0x20f0,
    },
    interval {
        first: 0x2cef,
        last: 0x2cf1,
    },
    interval {
        first: 0x2d7f,
        last: 0x2d7f,
    },
    interval {
        first: 0x2de0,
        last: 0x2dff,
    },
    interval {
        first: 0x302a,
        last: 0x302f,
    },
    interval {
        first: 0x3099,
        last: 0x309a,
    },
    interval {
        first: 0xa66f,
        last: 0xa672,
    },
    interval {
        first: 0xa674,
        last: 0xa67d,
    },
    interval {
        first: 0xa69e,
        last: 0xa69f,
    },
    interval {
        first: 0xa6f0,
        last: 0xa6f1,
    },
    interval {
        first: 0xa802,
        last: 0xa802,
    },
    interval {
        first: 0xa806,
        last: 0xa806,
    },
    interval {
        first: 0xa80b,
        last: 0xa80b,
    },
    interval {
        first: 0xa823,
        last: 0xa827,
    },
    interval {
        first: 0xa880,
        last: 0xa881,
    },
    interval {
        first: 0xa8b4,
        last: 0xa8c5,
    },
    interval {
        first: 0xa8e0,
        last: 0xa8f1,
    },
    interval {
        first: 0xa8ff,
        last: 0xa8ff,
    },
    interval {
        first: 0xa926,
        last: 0xa92d,
    },
    interval {
        first: 0xa947,
        last: 0xa953,
    },
    interval {
        first: 0xa980,
        last: 0xa983,
    },
    interval {
        first: 0xa9b3,
        last: 0xa9c0,
    },
    interval {
        first: 0xa9e5,
        last: 0xa9e5,
    },
    interval {
        first: 0xaa29,
        last: 0xaa36,
    },
    interval {
        first: 0xaa43,
        last: 0xaa43,
    },
    interval {
        first: 0xaa4c,
        last: 0xaa4d,
    },
    interval {
        first: 0xaa7b,
        last: 0xaa7d,
    },
    interval {
        first: 0xaab0,
        last: 0xaab0,
    },
    interval {
        first: 0xaab2,
        last: 0xaab4,
    },
    interval {
        first: 0xaab7,
        last: 0xaab8,
    },
    interval {
        first: 0xaabe,
        last: 0xaabf,
    },
    interval {
        first: 0xaac1,
        last: 0xaac1,
    },
    interval {
        first: 0xaaeb,
        last: 0xaaef,
    },
    interval {
        first: 0xaaf5,
        last: 0xaaf6,
    },
    interval {
        first: 0xabe3,
        last: 0xabea,
    },
    interval {
        first: 0xabec,
        last: 0xabed,
    },
    interval {
        first: 0xfb1e,
        last: 0xfb1e,
    },
    interval {
        first: 0xfe00,
        last: 0xfe0f,
    },
    interval {
        first: 0xfe20,
        last: 0xfe2f,
    },
    interval {
        first: 0x101fd,
        last: 0x101fd,
    },
    interval {
        first: 0x102e0,
        last: 0x102e0,
    },
    interval {
        first: 0x10376,
        last: 0x1037a,
    },
    interval {
        first: 0x10a01,
        last: 0x10a03,
    },
    interval {
        first: 0x10a05,
        last: 0x10a06,
    },
    interval {
        first: 0x10a0c,
        last: 0x10a0f,
    },
    interval {
        first: 0x10a38,
        last: 0x10a3a,
    },
    interval {
        first: 0x10a3f,
        last: 0x10a3f,
    },
    interval {
        first: 0x10ae5,
        last: 0x10ae6,
    },
    interval {
        first: 0x10d24,
        last: 0x10d27,
    },
    interval {
        first: 0x10f46,
        last: 0x10f50,
    },
    interval {
        first: 0x11000,
        last: 0x11002,
    },
    interval {
        first: 0x11038,
        last: 0x11046,
    },
    interval {
        first: 0x1107f,
        last: 0x11082,
    },
    interval {
        first: 0x110b0,
        last: 0x110ba,
    },
    interval {
        first: 0x11100,
        last: 0x11102,
    },
    interval {
        first: 0x11127,
        last: 0x11134,
    },
    interval {
        first: 0x11145,
        last: 0x11146,
    },
    interval {
        first: 0x11173,
        last: 0x11173,
    },
    interval {
        first: 0x11180,
        last: 0x11182,
    },
    interval {
        first: 0x111b3,
        last: 0x111c0,
    },
    interval {
        first: 0x111c9,
        last: 0x111cc,
    },
    interval {
        first: 0x1122c,
        last: 0x11237,
    },
    interval {
        first: 0x1123e,
        last: 0x1123e,
    },
    interval {
        first: 0x112df,
        last: 0x112ea,
    },
    interval {
        first: 0x11300,
        last: 0x11303,
    },
    interval {
        first: 0x1133b,
        last: 0x1133c,
    },
    interval {
        first: 0x1133e,
        last: 0x11344,
    },
    interval {
        first: 0x11347,
        last: 0x11348,
    },
    interval {
        first: 0x1134b,
        last: 0x1134d,
    },
    interval {
        first: 0x11357,
        last: 0x11357,
    },
    interval {
        first: 0x11362,
        last: 0x11363,
    },
    interval {
        first: 0x11366,
        last: 0x1136c,
    },
    interval {
        first: 0x11370,
        last: 0x11374,
    },
    interval {
        first: 0x11435,
        last: 0x11446,
    },
    interval {
        first: 0x1145e,
        last: 0x1145e,
    },
    interval {
        first: 0x114b0,
        last: 0x114c3,
    },
    interval {
        first: 0x115af,
        last: 0x115b5,
    },
    interval {
        first: 0x115b8,
        last: 0x115c0,
    },
    interval {
        first: 0x115dc,
        last: 0x115dd,
    },
    interval {
        first: 0x11630,
        last: 0x11640,
    },
    interval {
        first: 0x116ab,
        last: 0x116b7,
    },
    interval {
        first: 0x1171d,
        last: 0x1172b,
    },
    interval {
        first: 0x1182c,
        last: 0x1183a,
    },
    interval {
        first: 0x119d1,
        last: 0x119d7,
    },
    interval {
        first: 0x119da,
        last: 0x119e0,
    },
    interval {
        first: 0x119e4,
        last: 0x119e4,
    },
    interval {
        first: 0x11a01,
        last: 0x11a0a,
    },
    interval {
        first: 0x11a33,
        last: 0x11a39,
    },
    interval {
        first: 0x11a3b,
        last: 0x11a3e,
    },
    interval {
        first: 0x11a47,
        last: 0x11a47,
    },
    interval {
        first: 0x11a51,
        last: 0x11a5b,
    },
    interval {
        first: 0x11a8a,
        last: 0x11a99,
    },
    interval {
        first: 0x11c2f,
        last: 0x11c36,
    },
    interval {
        first: 0x11c38,
        last: 0x11c3f,
    },
    interval {
        first: 0x11c92,
        last: 0x11ca7,
    },
    interval {
        first: 0x11ca9,
        last: 0x11cb6,
    },
    interval {
        first: 0x11d31,
        last: 0x11d36,
    },
    interval {
        first: 0x11d3a,
        last: 0x11d3a,
    },
    interval {
        first: 0x11d3c,
        last: 0x11d3d,
    },
    interval {
        first: 0x11d3f,
        last: 0x11d45,
    },
    interval {
        first: 0x11d47,
        last: 0x11d47,
    },
    interval {
        first: 0x11d8a,
        last: 0x11d8e,
    },
    interval {
        first: 0x11d90,
        last: 0x11d91,
    },
    interval {
        first: 0x11d93,
        last: 0x11d97,
    },
    interval {
        first: 0x11ef3,
        last: 0x11ef6,
    },
    interval {
        first: 0x16af0,
        last: 0x16af4,
    },
    interval {
        first: 0x16b30,
        last: 0x16b36,
    },
    interval {
        first: 0x16f4f,
        last: 0x16f4f,
    },
    interval {
        first: 0x16f51,
        last: 0x16f87,
    },
    interval {
        first: 0x16f8f,
        last: 0x16f92,
    },
    interval {
        first: 0x1bc9d,
        last: 0x1bc9e,
    },
    interval {
        first: 0x1d165,
        last: 0x1d169,
    },
    interval {
        first: 0x1d16d,
        last: 0x1d172,
    },
    interval {
        first: 0x1d17b,
        last: 0x1d182,
    },
    interval {
        first: 0x1d185,
        last: 0x1d18b,
    },
    interval {
        first: 0x1d1aa,
        last: 0x1d1ad,
    },
    interval {
        first: 0x1d242,
        last: 0x1d244,
    },
    interval {
        first: 0x1da00,
        last: 0x1da36,
    },
    interval {
        first: 0x1da3b,
        last: 0x1da6c,
    },
    interval {
        first: 0x1da75,
        last: 0x1da75,
    },
    interval {
        first: 0x1da84,
        last: 0x1da84,
    },
    interval {
        first: 0x1da9b,
        last: 0x1da9f,
    },
    interval {
        first: 0x1daa1,
        last: 0x1daaf,
    },
    interval {
        first: 0x1e000,
        last: 0x1e006,
    },
    interval {
        first: 0x1e008,
        last: 0x1e018,
    },
    interval {
        first: 0x1e01b,
        last: 0x1e021,
    },
    interval {
        first: 0x1e023,
        last: 0x1e024,
    },
    interval {
        first: 0x1e026,
        last: 0x1e02a,
    },
    interval {
        first: 0x1e130,
        last: 0x1e136,
    },
    interval {
        first: 0x1e2ec,
        last: 0x1e2ef,
    },
    interval {
        first: 0x1e8d0,
        last: 0x1e8d6,
    },
    interval {
        first: 0x1e944,
        last: 0x1e94a,
    },
    interval {
        first: 0xe0100,
        last: 0xe01ef,
    },
];
pub static mut foldCase: [convertStruct; 192] = [
    convertStruct {
        rangeStart: 0x41,
        rangeEnd: 0x5a,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0xb5,
        rangeEnd: 0xb5,
        step: -(1),
        offset: 775,
    },
    convertStruct {
        rangeStart: 0xc0,
        rangeEnd: 0xd6,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0xd8,
        rangeEnd: 0xde,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x100,
        rangeEnd: 0x12e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x132,
        rangeEnd: 0x136,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x139,
        rangeEnd: 0x147,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x14a,
        rangeEnd: 0x176,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x178,
        rangeEnd: 0x178,
        step: -(1),
        offset: -(121),
    },
    convertStruct {
        rangeStart: 0x179,
        rangeEnd: 0x17d,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x17f,
        rangeEnd: 0x17f,
        step: -(1),
        offset: -(268),
    },
    convertStruct {
        rangeStart: 0x181,
        rangeEnd: 0x181,
        step: -(1),
        offset: 210,
    },
    convertStruct {
        rangeStart: 0x182,
        rangeEnd: 0x184,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x186,
        rangeEnd: 0x186,
        step: -(1),
        offset: 206,
    },
    convertStruct {
        rangeStart: 0x187,
        rangeEnd: 0x187,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x189,
        rangeEnd: 0x18a,
        step: 1,
        offset: 205,
    },
    convertStruct {
        rangeStart: 0x18b,
        rangeEnd: 0x18b,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x18e,
        rangeEnd: 0x18e,
        step: -(1),
        offset: 79,
    },
    convertStruct {
        rangeStart: 0x18f,
        rangeEnd: 0x18f,
        step: -(1),
        offset: 202,
    },
    convertStruct {
        rangeStart: 0x190,
        rangeEnd: 0x190,
        step: -(1),
        offset: 203,
    },
    convertStruct {
        rangeStart: 0x191,
        rangeEnd: 0x191,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x193,
        rangeEnd: 0x193,
        step: -(1),
        offset: 205,
    },
    convertStruct {
        rangeStart: 0x194,
        rangeEnd: 0x194,
        step: -(1),
        offset: 207,
    },
    convertStruct {
        rangeStart: 0x196,
        rangeEnd: 0x196,
        step: -(1),
        offset: 211,
    },
    convertStruct {
        rangeStart: 0x197,
        rangeEnd: 0x197,
        step: -(1),
        offset: 209,
    },
    convertStruct {
        rangeStart: 0x198,
        rangeEnd: 0x198,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x19c,
        rangeEnd: 0x19c,
        step: -(1),
        offset: 211,
    },
    convertStruct {
        rangeStart: 0x19d,
        rangeEnd: 0x19d,
        step: -(1),
        offset: 213,
    },
    convertStruct {
        rangeStart: 0x19f,
        rangeEnd: 0x19f,
        step: -(1),
        offset: 214,
    },
    convertStruct {
        rangeStart: 0x1a0,
        rangeEnd: 0x1a4,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1a6,
        rangeEnd: 0x1a6,
        step: -(1),
        offset: 218,
    },
    convertStruct {
        rangeStart: 0x1a7,
        rangeEnd: 0x1a7,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1a9,
        rangeEnd: 0x1a9,
        step: -(1),
        offset: 218,
    },
    convertStruct {
        rangeStart: 0x1ac,
        rangeEnd: 0x1ac,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1ae,
        rangeEnd: 0x1ae,
        step: -(1),
        offset: 218,
    },
    convertStruct {
        rangeStart: 0x1af,
        rangeEnd: 0x1af,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1b1,
        rangeEnd: 0x1b2,
        step: 1,
        offset: 217,
    },
    convertStruct {
        rangeStart: 0x1b3,
        rangeEnd: 0x1b5,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1b7,
        rangeEnd: 0x1b7,
        step: -(1),
        offset: 219,
    },
    convertStruct {
        rangeStart: 0x1b8,
        rangeEnd: 0x1bc,
        step: 4,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1c4,
        rangeEnd: 0x1c4,
        step: -(1),
        offset: 2,
    },
    convertStruct {
        rangeStart: 0x1c5,
        rangeEnd: 0x1c5,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1c7,
        rangeEnd: 0x1c7,
        step: -(1),
        offset: 2,
    },
    convertStruct {
        rangeStart: 0x1c8,
        rangeEnd: 0x1c8,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1ca,
        rangeEnd: 0x1ca,
        step: -(1),
        offset: 2,
    },
    convertStruct {
        rangeStart: 0x1cb,
        rangeEnd: 0x1db,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1de,
        rangeEnd: 0x1ee,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1f1,
        rangeEnd: 0x1f1,
        step: -(1),
        offset: 2,
    },
    convertStruct {
        rangeStart: 0x1f2,
        rangeEnd: 0x1f4,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1f6,
        rangeEnd: 0x1f6,
        step: -(1),
        offset: -(97),
    },
    convertStruct {
        rangeStart: 0x1f7,
        rangeEnd: 0x1f7,
        step: -(1),
        offset: -(56),
    },
    convertStruct {
        rangeStart: 0x1f8,
        rangeEnd: 0x21e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x220,
        rangeEnd: 0x220,
        step: -(1),
        offset: -(130),
    },
    convertStruct {
        rangeStart: 0x222,
        rangeEnd: 0x232,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x23a,
        rangeEnd: 0x23a,
        step: -(1),
        offset: 10795,
    },
    convertStruct {
        rangeStart: 0x23b,
        rangeEnd: 0x23b,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x23d,
        rangeEnd: 0x23d,
        step: -(1),
        offset: -(163),
    },
    convertStruct {
        rangeStart: 0x23e,
        rangeEnd: 0x23e,
        step: -(1),
        offset: 10792,
    },
    convertStruct {
        rangeStart: 0x241,
        rangeEnd: 0x241,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x243,
        rangeEnd: 0x243,
        step: -(1),
        offset: -(195),
    },
    convertStruct {
        rangeStart: 0x244,
        rangeEnd: 0x244,
        step: -(1),
        offset: 69,
    },
    convertStruct {
        rangeStart: 0x245,
        rangeEnd: 0x245,
        step: -(1),
        offset: 71,
    },
    convertStruct {
        rangeStart: 0x246,
        rangeEnd: 0x24e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x345,
        rangeEnd: 0x345,
        step: -(1),
        offset: 116,
    },
    convertStruct {
        rangeStart: 0x370,
        rangeEnd: 0x372,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x376,
        rangeEnd: 0x376,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x37f,
        rangeEnd: 0x37f,
        step: -(1),
        offset: 116,
    },
    convertStruct {
        rangeStart: 0x386,
        rangeEnd: 0x386,
        step: -(1),
        offset: 38,
    },
    convertStruct {
        rangeStart: 0x388,
        rangeEnd: 0x38a,
        step: 1,
        offset: 37,
    },
    convertStruct {
        rangeStart: 0x38c,
        rangeEnd: 0x38c,
        step: -(1),
        offset: 64,
    },
    convertStruct {
        rangeStart: 0x38e,
        rangeEnd: 0x38f,
        step: 1,
        offset: 63,
    },
    convertStruct {
        rangeStart: 0x391,
        rangeEnd: 0x3a1,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x3a3,
        rangeEnd: 0x3ab,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x3c2,
        rangeEnd: 0x3c2,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x3cf,
        rangeEnd: 0x3cf,
        step: -(1),
        offset: 8,
    },
    convertStruct {
        rangeStart: 0x3d0,
        rangeEnd: 0x3d0,
        step: -(1),
        offset: -(30),
    },
    convertStruct {
        rangeStart: 0x3d1,
        rangeEnd: 0x3d1,
        step: -(1),
        offset: -(25),
    },
    convertStruct {
        rangeStart: 0x3d5,
        rangeEnd: 0x3d5,
        step: -(1),
        offset: -(15),
    },
    convertStruct {
        rangeStart: 0x3d6,
        rangeEnd: 0x3d6,
        step: -(1),
        offset: -(22),
    },
    convertStruct {
        rangeStart: 0x3d8,
        rangeEnd: 0x3ee,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x3f0,
        rangeEnd: 0x3f0,
        step: -(1),
        offset: -(54),
    },
    convertStruct {
        rangeStart: 0x3f1,
        rangeEnd: 0x3f1,
        step: -(1),
        offset: -(48),
    },
    convertStruct {
        rangeStart: 0x3f4,
        rangeEnd: 0x3f4,
        step: -(1),
        offset: -(60),
    },
    convertStruct {
        rangeStart: 0x3f5,
        rangeEnd: 0x3f5,
        step: -(1),
        offset: -(64),
    },
    convertStruct {
        rangeStart: 0x3f7,
        rangeEnd: 0x3f7,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x3f9,
        rangeEnd: 0x3f9,
        step: -(1),
        offset: -(7),
    },
    convertStruct {
        rangeStart: 0x3fa,
        rangeEnd: 0x3fa,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x3fd,
        rangeEnd: 0x3ff,
        step: 1,
        offset: -(130),
    },
    convertStruct {
        rangeStart: 0x400,
        rangeEnd: 0x40f,
        step: 1,
        offset: 80,
    },
    convertStruct {
        rangeStart: 0x410,
        rangeEnd: 0x42f,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x460,
        rangeEnd: 0x480,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x48a,
        rangeEnd: 0x4be,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x4c0,
        rangeEnd: 0x4c0,
        step: -(1),
        offset: 15,
    },
    convertStruct {
        rangeStart: 0x4c1,
        rangeEnd: 0x4cd,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x4d0,
        rangeEnd: 0x52e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x531,
        rangeEnd: 0x556,
        step: 1,
        offset: 48,
    },
    convertStruct {
        rangeStart: 0x10a0,
        rangeEnd: 0x10c5,
        step: 1,
        offset: 7264,
    },
    convertStruct {
        rangeStart: 0x10c7,
        rangeEnd: 0x10cd,
        step: 6,
        offset: 7264,
    },
    convertStruct {
        rangeStart: 0x13f8,
        rangeEnd: 0x13fd,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1c80,
        rangeEnd: 0x1c80,
        step: -(1),
        offset: -(6222),
    },
    convertStruct {
        rangeStart: 0x1c81,
        rangeEnd: 0x1c81,
        step: -(1),
        offset: -(6221),
    },
    convertStruct {
        rangeStart: 0x1c82,
        rangeEnd: 0x1c82,
        step: -(1),
        offset: -(6212),
    },
    convertStruct {
        rangeStart: 0x1c83,
        rangeEnd: 0x1c84,
        step: 1,
        offset: -(6210),
    },
    convertStruct {
        rangeStart: 0x1c85,
        rangeEnd: 0x1c85,
        step: -(1),
        offset: -(6211),
    },
    convertStruct {
        rangeStart: 0x1c86,
        rangeEnd: 0x1c86,
        step: -(1),
        offset: -(6204),
    },
    convertStruct {
        rangeStart: 0x1c87,
        rangeEnd: 0x1c87,
        step: -(1),
        offset: -(6180),
    },
    convertStruct {
        rangeStart: 0x1c88,
        rangeEnd: 0x1c88,
        step: -(1),
        offset: 35267,
    },
    convertStruct {
        rangeStart: 0x1c90,
        rangeEnd: 0x1cba,
        step: 1,
        offset: -(3008),
    },
    convertStruct {
        rangeStart: 0x1cbd,
        rangeEnd: 0x1cbf,
        step: 1,
        offset: -(3008),
    },
    convertStruct {
        rangeStart: 0x1e00,
        rangeEnd: 0x1e94,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1e9b,
        rangeEnd: 0x1e9b,
        step: -(1),
        offset: -(58),
    },
    convertStruct {
        rangeStart: 0x1e9e,
        rangeEnd: 0x1e9e,
        step: -(1),
        offset: -(7615),
    },
    convertStruct {
        rangeStart: 0x1ea0,
        rangeEnd: 0x1efe,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x1f08,
        rangeEnd: 0x1f0f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f18,
        rangeEnd: 0x1f1d,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f28,
        rangeEnd: 0x1f2f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f38,
        rangeEnd: 0x1f3f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f48,
        rangeEnd: 0x1f4d,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f59,
        rangeEnd: 0x1f5f,
        step: 2,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f68,
        rangeEnd: 0x1f6f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f88,
        rangeEnd: 0x1f8f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1f98,
        rangeEnd: 0x1f9f,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fa8,
        rangeEnd: 0x1faf,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fb8,
        rangeEnd: 0x1fb9,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fba,
        rangeEnd: 0x1fbb,
        step: 1,
        offset: -(74),
    },
    convertStruct {
        rangeStart: 0x1fbc,
        rangeEnd: 0x1fbc,
        step: -(1),
        offset: -(9),
    },
    convertStruct {
        rangeStart: 0x1fbe,
        rangeEnd: 0x1fbe,
        step: -(1),
        offset: -(7173),
    },
    convertStruct {
        rangeStart: 0x1fc8,
        rangeEnd: 0x1fcb,
        step: 1,
        offset: -(86),
    },
    convertStruct {
        rangeStart: 0x1fcc,
        rangeEnd: 0x1fcc,
        step: -(1),
        offset: -(9),
    },
    convertStruct {
        rangeStart: 0x1fd8,
        rangeEnd: 0x1fd9,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fda,
        rangeEnd: 0x1fdb,
        step: 1,
        offset: -(100),
    },
    convertStruct {
        rangeStart: 0x1fe8,
        rangeEnd: 0x1fe9,
        step: 1,
        offset: -(8),
    },
    convertStruct {
        rangeStart: 0x1fea,
        rangeEnd: 0x1feb,
        step: 1,
        offset: -(112),
    },
    convertStruct {
        rangeStart: 0x1fec,
        rangeEnd: 0x1fec,
        step: -(1),
        offset: -(7),
    },
    convertStruct {
        rangeStart: 0x1ff8,
        rangeEnd: 0x1ff9,
        step: 1,
        offset: -(128),
    },
    convertStruct {
        rangeStart: 0x1ffa,
        rangeEnd: 0x1ffb,
        step: 1,
        offset: -(126),
    },
    convertStruct {
        rangeStart: 0x1ffc,
        rangeEnd: 0x1ffc,
        step: -(1),
        offset: -(9),
    },
    convertStruct {
        rangeStart: 0x2126,
        rangeEnd: 0x2126,
        step: -(1),
        offset: -(7517),
    },
    convertStruct {
        rangeStart: 0x212a,
        rangeEnd: 0x212a,
        step: -(1),
        offset: -(8383),
    },
    convertStruct {
        rangeStart: 0x212b,
        rangeEnd: 0x212b,
        step: -(1),
        offset: -(8262),
    },
    convertStruct {
        rangeStart: 0x2132,
        rangeEnd: 0x2132,
        step: -(1),
        offset: 28,
    },
    convertStruct {
        rangeStart: 0x2160,
        rangeEnd: 0x216f,
        step: 1,
        offset: 16,
    },
    convertStruct {
        rangeStart: 0x2183,
        rangeEnd: 0x2183,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x24b6,
        rangeEnd: 0x24cf,
        step: 1,
        offset: 26,
    },
    convertStruct {
        rangeStart: 0x2c00,
        rangeEnd: 0x2c2e,
        step: 1,
        offset: 48,
    },
    convertStruct {
        rangeStart: 0x2c60,
        rangeEnd: 0x2c60,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2c62,
        rangeEnd: 0x2c62,
        step: -(1),
        offset: -(10743),
    },
    convertStruct {
        rangeStart: 0x2c63,
        rangeEnd: 0x2c63,
        step: -(1),
        offset: -(3814),
    },
    convertStruct {
        rangeStart: 0x2c64,
        rangeEnd: 0x2c64,
        step: -(1),
        offset: -(10727),
    },
    convertStruct {
        rangeStart: 0x2c67,
        rangeEnd: 0x2c6b,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2c6d,
        rangeEnd: 0x2c6d,
        step: -(1),
        offset: -(10780),
    },
    convertStruct {
        rangeStart: 0x2c6e,
        rangeEnd: 0x2c6e,
        step: -(1),
        offset: -(10749),
    },
    convertStruct {
        rangeStart: 0x2c6f,
        rangeEnd: 0x2c6f,
        step: -(1),
        offset: -(10783),
    },
    convertStruct {
        rangeStart: 0x2c70,
        rangeEnd: 0x2c70,
        step: -(1),
        offset: -(10782),
    },
    convertStruct {
        rangeStart: 0x2c72,
        rangeEnd: 0x2c75,
        step: 3,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2c7e,
        rangeEnd: 0x2c7f,
        step: 1,
        offset: -(10815),
    },
    convertStruct {
        rangeStart: 0x2c80,
        rangeEnd: 0x2ce2,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2ceb,
        rangeEnd: 0x2ced,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0x2cf2,
        rangeEnd: 0xa640,
        step: 31054,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa642,
        rangeEnd: 0xa66c,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa680,
        rangeEnd: 0xa69a,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa722,
        rangeEnd: 0xa72e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa732,
        rangeEnd: 0xa76e,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa779,
        rangeEnd: 0xa77b,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa77d,
        rangeEnd: 0xa77d,
        step: -(1),
        offset: -(35332),
    },
    convertStruct {
        rangeStart: 0xa77e,
        rangeEnd: 0xa786,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa78b,
        rangeEnd: 0xa78b,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa78d,
        rangeEnd: 0xa78d,
        step: -(1),
        offset: -(42280),
    },
    convertStruct {
        rangeStart: 0xa790,
        rangeEnd: 0xa792,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa796,
        rangeEnd: 0xa7a8,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa7aa,
        rangeEnd: 0xa7aa,
        step: -(1),
        offset: -(42308),
    },
    convertStruct {
        rangeStart: 0xa7ab,
        rangeEnd: 0xa7ab,
        step: -(1),
        offset: -(42319),
    },
    convertStruct {
        rangeStart: 0xa7ac,
        rangeEnd: 0xa7ac,
        step: -(1),
        offset: -(42315),
    },
    convertStruct {
        rangeStart: 0xa7ad,
        rangeEnd: 0xa7ad,
        step: -(1),
        offset: -(42305),
    },
    convertStruct {
        rangeStart: 0xa7ae,
        rangeEnd: 0xa7ae,
        step: -(1),
        offset: -(42308),
    },
    convertStruct {
        rangeStart: 0xa7b0,
        rangeEnd: 0xa7b0,
        step: -(1),
        offset: -(42258),
    },
    convertStruct {
        rangeStart: 0xa7b1,
        rangeEnd: 0xa7b1,
        step: -(1),
        offset: -(42282),
    },
    convertStruct {
        rangeStart: 0xa7b2,
        rangeEnd: 0xa7b2,
        step: -(1),
        offset: -(42261),
    },
    convertStruct {
        rangeStart: 0xa7b3,
        rangeEnd: 0xa7b3,
        step: -(1),
        offset: 928,
    },
    convertStruct {
        rangeStart: 0xa7b4,
        rangeEnd: 0xa7be,
        step: 2,
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa7c2,
        rangeEnd: 0xa7c2,
        step: -(1),
        offset: 1,
    },
    convertStruct {
        rangeStart: 0xa7c4,
        rangeEnd: 0xa7c4,
        step: -(1),
        offset: -(48),
    },
    convertStruct {
        rangeStart: 0xa7c5,
        rangeEnd: 0xa7c5,
        step: -(1),
        offset: -(42307),
    },
    convertStruct {
        rangeStart: 0xa7c6,
        rangeEnd: 0xa7c6,
        step: -(1),
        offset: -(35384),
    },
    convertStruct {
        rangeStart: 0xab70,
        rangeEnd: 0xabbf,
        step: 1,
        offset: -(38864),
    },
    convertStruct {
        rangeStart: 0xff21,
        rangeEnd: 0xff3a,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x10400,
        rangeEnd: 0x10427,
        step: 1,
        offset: 40,
    },
    convertStruct {
        rangeStart: 0x104b0,
        rangeEnd: 0x104d3,
        step: 1,
        offset: 40,
    },
    convertStruct {
        rangeStart: 0x10c80,
        rangeEnd: 0x10cb2,
        step: 1,
        offset: 64,
    },
    convertStruct {
        rangeStart: 0x118a0,
        rangeEnd: 0x118bf,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x16e40,
        rangeEnd: 0x16e5f,
        step: 1,
        offset: 32,
    },
    convertStruct {
        rangeStart: 0x1e900,
        rangeEnd: 0x1e921,
        step: 1,
        offset: 34,
    },
];
pub static mut doublewidth: [interval; 113] = [
    interval {
        first: 0x1100,
        last: 0x115f,
    },
    interval {
        first: 0x231a,
        last: 0x231b,
    },
    interval {
        first: 0x2329,
        last: 0x232a,
    },
    interval {
        first: 0x23e9,
        last: 0x23ec,
    },
    interval {
        first: 0x23f0,
        last: 0x23f0,
    },
    interval {
        first: 0x23f3,
        last: 0x23f3,
    },
    interval {
        first: 0x25fd,
        last: 0x25fe,
    },
    interval {
        first: 0x2614,
        last: 0x2615,
    },
    interval {
        first: 0x2648,
        last: 0x2653,
    },
    interval {
        first: 0x267f,
        last: 0x267f,
    },
    interval {
        first: 0x2693,
        last: 0x2693,
    },
    interval {
        first: 0x26a1,
        last: 0x26a1,
    },
    interval {
        first: 0x26aa,
        last: 0x26ab,
    },
    interval {
        first: 0x26bd,
        last: 0x26be,
    },
    interval {
        first: 0x26c4,
        last: 0x26c5,
    },
    interval {
        first: 0x26ce,
        last: 0x26ce,
    },
    interval {
        first: 0x26d4,
        last: 0x26d4,
    },
    interval {
        first: 0x26ea,
        last: 0x26ea,
    },
    interval {
        first: 0x26f2,
        last: 0x26f3,
    },
    interval {
        first: 0x26f5,
        last: 0x26f5,
    },
    interval {
        first: 0x26fa,
        last: 0x26fa,
    },
    interval {
        first: 0x26fd,
        last: 0x26fd,
    },
    interval {
        first: 0x2705,
        last: 0x2705,
    },
    interval {
        first: 0x270a,
        last: 0x270b,
    },
    interval {
        first: 0x2728,
        last: 0x2728,
    },
    interval {
        first: 0x274c,
        last: 0x274c,
    },
    interval {
        first: 0x274e,
        last: 0x274e,
    },
    interval {
        first: 0x2753,
        last: 0x2755,
    },
    interval {
        first: 0x2757,
        last: 0x2757,
    },
    interval {
        first: 0x2795,
        last: 0x2797,
    },
    interval {
        first: 0x27b0,
        last: 0x27b0,
    },
    interval {
        first: 0x27bf,
        last: 0x27bf,
    },
    interval {
        first: 0x2b1b,
        last: 0x2b1c,
    },
    interval {
        first: 0x2b50,
        last: 0x2b50,
    },
    interval {
        first: 0x2b55,
        last: 0x2b55,
    },
    interval {
        first: 0x2e80,
        last: 0x2e99,
    },
    interval {
        first: 0x2e9b,
        last: 0x2ef3,
    },
    interval {
        first: 0x2f00,
        last: 0x2fd5,
    },
    interval {
        first: 0x2ff0,
        last: 0x2ffb,
    },
    interval {
        first: 0x3000,
        last: 0x303e,
    },
    interval {
        first: 0x3041,
        last: 0x3096,
    },
    interval {
        first: 0x3099,
        last: 0x30ff,
    },
    interval {
        first: 0x3105,
        last: 0x312f,
    },
    interval {
        first: 0x3131,
        last: 0x318e,
    },
    interval {
        first: 0x3190,
        last: 0x31ba,
    },
    interval {
        first: 0x31c0,
        last: 0x31e3,
    },
    interval {
        first: 0x31f0,
        last: 0x321e,
    },
    interval {
        first: 0x3220,
        last: 0x3247,
    },
    interval {
        first: 0x3250,
        last: 0x4dbf,
    },
    interval {
        first: 0x4e00,
        last: 0xa48c,
    },
    interval {
        first: 0xa490,
        last: 0xa4c6,
    },
    interval {
        first: 0xa960,
        last: 0xa97c,
    },
    interval {
        first: 0xac00,
        last: 0xd7a3,
    },
    interval {
        first: 0xf900,
        last: 0xfaff,
    },
    interval {
        first: 0xfe10,
        last: 0xfe19,
    },
    interval {
        first: 0xfe30,
        last: 0xfe52,
    },
    interval {
        first: 0xfe54,
        last: 0xfe66,
    },
    interval {
        first: 0xfe68,
        last: 0xfe6b,
    },
    interval {
        first: 0xff01,
        last: 0xff60,
    },
    interval {
        first: 0xffe0,
        last: 0xffe6,
    },
    interval {
        first: 0x16fe0,
        last: 0x16fe3,
    },
    interval {
        first: 0x17000,
        last: 0x187f7,
    },
    interval {
        first: 0x18800,
        last: 0x18af2,
    },
    interval {
        first: 0x1b000,
        last: 0x1b11e,
    },
    interval {
        first: 0x1b150,
        last: 0x1b152,
    },
    interval {
        first: 0x1b164,
        last: 0x1b167,
    },
    interval {
        first: 0x1b170,
        last: 0x1b2fb,
    },
    interval {
        first: 0x1f004,
        last: 0x1f004,
    },
    interval {
        first: 0x1f0cf,
        last: 0x1f0cf,
    },
    interval {
        first: 0x1f18e,
        last: 0x1f18e,
    },
    interval {
        first: 0x1f191,
        last: 0x1f19a,
    },
    interval {
        first: 0x1f200,
        last: 0x1f202,
    },
    interval {
        first: 0x1f210,
        last: 0x1f23b,
    },
    interval {
        first: 0x1f240,
        last: 0x1f248,
    },
    interval {
        first: 0x1f250,
        last: 0x1f251,
    },
    interval {
        first: 0x1f260,
        last: 0x1f265,
    },
    interval {
        first: 0x1f300,
        last: 0x1f320,
    },
    interval {
        first: 0x1f32d,
        last: 0x1f335,
    },
    interval {
        first: 0x1f337,
        last: 0x1f37c,
    },
    interval {
        first: 0x1f37e,
        last: 0x1f393,
    },
    interval {
        first: 0x1f3a0,
        last: 0x1f3ca,
    },
    interval {
        first: 0x1f3cf,
        last: 0x1f3d3,
    },
    interval {
        first: 0x1f3e0,
        last: 0x1f3f0,
    },
    interval {
        first: 0x1f3f4,
        last: 0x1f3f4,
    },
    interval {
        first: 0x1f3f8,
        last: 0x1f43e,
    },
    interval {
        first: 0x1f440,
        last: 0x1f440,
    },
    interval {
        first: 0x1f442,
        last: 0x1f4fc,
    },
    interval {
        first: 0x1f4ff,
        last: 0x1f53d,
    },
    interval {
        first: 0x1f54b,
        last: 0x1f54e,
    },
    interval {
        first: 0x1f550,
        last: 0x1f567,
    },
    interval {
        first: 0x1f57a,
        last: 0x1f57a,
    },
    interval {
        first: 0x1f595,
        last: 0x1f596,
    },
    interval {
        first: 0x1f5a4,
        last: 0x1f5a4,
    },
    interval {
        first: 0x1f5fb,
        last: 0x1f64f,
    },
    interval {
        first: 0x1f680,
        last: 0x1f6c5,
    },
    interval {
        first: 0x1f6cc,
        last: 0x1f6cc,
    },
    interval {
        first: 0x1f6d0,
        last: 0x1f6d2,
    },
    interval {
        first: 0x1f6d5,
        last: 0x1f6d5,
    },
    interval {
        first: 0x1f6eb,
        last: 0x1f6ec,
    },
    interval {
        first: 0x1f6f4,
        last: 0x1f6fa,
    },
    interval {
        first: 0x1f7e0,
        last: 0x1f7eb,
    },
    interval {
        first: 0x1f90d,
        last: 0x1f971,
    },
    interval {
        first: 0x1f973,
        last: 0x1f976,
    },
    interval {
        first: 0x1f97a,
        last: 0x1f9a2,
    },
    interval {
        first: 0x1f9a5,
        last: 0x1f9aa,
    },
    interval {
        first: 0x1f9ae,
        last: 0x1f9ca,
    },
    interval {
        first: 0x1f9cd,
        last: 0x1f9ff,
    },
    interval {
        first: 0x1fa70,
        last: 0x1fa73,
    },
    interval {
        first: 0x1fa78,
        last: 0x1fa7a,
    },
    interval {
        first: 0x1fa80,
        last: 0x1fa82,
    },
    interval {
        first: 0x1fa90,
        last: 0x1fa95,
    },
    interval {
        first: 0x20000,
        last: 0x2fffd,
    },
    interval {
        first: 0x30000,
        last: 0x3fffd,
    },
];
pub static mut ambiguous: [interval; 179] = [
    interval {
        first: 0xa1,
        last: 0xa1,
    },
    interval {
        first: 0xa4,
        last: 0xa4,
    },
    interval {
        first: 0xa7,
        last: 0xa8,
    },
    interval {
        first: 0xaa,
        last: 0xaa,
    },
    interval {
        first: 0xad,
        last: 0xae,
    },
    interval {
        first: 0xb0,
        last: 0xb4,
    },
    interval {
        first: 0xb6,
        last: 0xba,
    },
    interval {
        first: 0xbc,
        last: 0xbf,
    },
    interval {
        first: 0xc6,
        last: 0xc6,
    },
    interval {
        first: 0xd0,
        last: 0xd0,
    },
    interval {
        first: 0xd7,
        last: 0xd8,
    },
    interval {
        first: 0xde,
        last: 0xe1,
    },
    interval {
        first: 0xe6,
        last: 0xe6,
    },
    interval {
        first: 0xe8,
        last: 0xea,
    },
    interval {
        first: 0xec,
        last: 0xed,
    },
    interval {
        first: 0xf0,
        last: 0xf0,
    },
    interval {
        first: 0xf2,
        last: 0xf3,
    },
    interval {
        first: 0xf7,
        last: 0xfa,
    },
    interval {
        first: 0xfc,
        last: 0xfc,
    },
    interval {
        first: 0xfe,
        last: 0xfe,
    },
    interval {
        first: 0x101,
        last: 0x101,
    },
    interval {
        first: 0x111,
        last: 0x111,
    },
    interval {
        first: 0x113,
        last: 0x113,
    },
    interval {
        first: 0x11b,
        last: 0x11b,
    },
    interval {
        first: 0x126,
        last: 0x127,
    },
    interval {
        first: 0x12b,
        last: 0x12b,
    },
    interval {
        first: 0x131,
        last: 0x133,
    },
    interval {
        first: 0x138,
        last: 0x138,
    },
    interval {
        first: 0x13f,
        last: 0x142,
    },
    interval {
        first: 0x144,
        last: 0x144,
    },
    interval {
        first: 0x148,
        last: 0x14b,
    },
    interval {
        first: 0x14d,
        last: 0x14d,
    },
    interval {
        first: 0x152,
        last: 0x153,
    },
    interval {
        first: 0x166,
        last: 0x167,
    },
    interval {
        first: 0x16b,
        last: 0x16b,
    },
    interval {
        first: 0x1ce,
        last: 0x1ce,
    },
    interval {
        first: 0x1d0,
        last: 0x1d0,
    },
    interval {
        first: 0x1d2,
        last: 0x1d2,
    },
    interval {
        first: 0x1d4,
        last: 0x1d4,
    },
    interval {
        first: 0x1d6,
        last: 0x1d6,
    },
    interval {
        first: 0x1d8,
        last: 0x1d8,
    },
    interval {
        first: 0x1da,
        last: 0x1da,
    },
    interval {
        first: 0x1dc,
        last: 0x1dc,
    },
    interval {
        first: 0x251,
        last: 0x251,
    },
    interval {
        first: 0x261,
        last: 0x261,
    },
    interval {
        first: 0x2c4,
        last: 0x2c4,
    },
    interval {
        first: 0x2c7,
        last: 0x2c7,
    },
    interval {
        first: 0x2c9,
        last: 0x2cb,
    },
    interval {
        first: 0x2cd,
        last: 0x2cd,
    },
    interval {
        first: 0x2d0,
        last: 0x2d0,
    },
    interval {
        first: 0x2d8,
        last: 0x2db,
    },
    interval {
        first: 0x2dd,
        last: 0x2dd,
    },
    interval {
        first: 0x2df,
        last: 0x2df,
    },
    interval {
        first: 0x300,
        last: 0x36f,
    },
    interval {
        first: 0x391,
        last: 0x3a1,
    },
    interval {
        first: 0x3a3,
        last: 0x3a9,
    },
    interval {
        first: 0x3b1,
        last: 0x3c1,
    },
    interval {
        first: 0x3c3,
        last: 0x3c9,
    },
    interval {
        first: 0x401,
        last: 0x401,
    },
    interval {
        first: 0x410,
        last: 0x44f,
    },
    interval {
        first: 0x451,
        last: 0x451,
    },
    interval {
        first: 0x2010,
        last: 0x2010,
    },
    interval {
        first: 0x2013,
        last: 0x2016,
    },
    interval {
        first: 0x2018,
        last: 0x2019,
    },
    interval {
        first: 0x201c,
        last: 0x201d,
    },
    interval {
        first: 0x2020,
        last: 0x2022,
    },
    interval {
        first: 0x2024,
        last: 0x2027,
    },
    interval {
        first: 0x2030,
        last: 0x2030,
    },
    interval {
        first: 0x2032,
        last: 0x2033,
    },
    interval {
        first: 0x2035,
        last: 0x2035,
    },
    interval {
        first: 0x203b,
        last: 0x203b,
    },
    interval {
        first: 0x203e,
        last: 0x203e,
    },
    interval {
        first: 0x2074,
        last: 0x2074,
    },
    interval {
        first: 0x207f,
        last: 0x207f,
    },
    interval {
        first: 0x2081,
        last: 0x2084,
    },
    interval {
        first: 0x20ac,
        last: 0x20ac,
    },
    interval {
        first: 0x2103,
        last: 0x2103,
    },
    interval {
        first: 0x2105,
        last: 0x2105,
    },
    interval {
        first: 0x2109,
        last: 0x2109,
    },
    interval {
        first: 0x2113,
        last: 0x2113,
    },
    interval {
        first: 0x2116,
        last: 0x2116,
    },
    interval {
        first: 0x2121,
        last: 0x2122,
    },
    interval {
        first: 0x2126,
        last: 0x2126,
    },
    interval {
        first: 0x212b,
        last: 0x212b,
    },
    interval {
        first: 0x2153,
        last: 0x2154,
    },
    interval {
        first: 0x215b,
        last: 0x215e,
    },
    interval {
        first: 0x2160,
        last: 0x216b,
    },
    interval {
        first: 0x2170,
        last: 0x2179,
    },
    interval {
        first: 0x2189,
        last: 0x2189,
    },
    interval {
        first: 0x2190,
        last: 0x2199,
    },
    interval {
        first: 0x21b8,
        last: 0x21b9,
    },
    interval {
        first: 0x21d2,
        last: 0x21d2,
    },
    interval {
        first: 0x21d4,
        last: 0x21d4,
    },
    interval {
        first: 0x21e7,
        last: 0x21e7,
    },
    interval {
        first: 0x2200,
        last: 0x2200,
    },
    interval {
        first: 0x2202,
        last: 0x2203,
    },
    interval {
        first: 0x2207,
        last: 0x2208,
    },
    interval {
        first: 0x220b,
        last: 0x220b,
    },
    interval {
        first: 0x220f,
        last: 0x220f,
    },
    interval {
        first: 0x2211,
        last: 0x2211,
    },
    interval {
        first: 0x2215,
        last: 0x2215,
    },
    interval {
        first: 0x221a,
        last: 0x221a,
    },
    interval {
        first: 0x221d,
        last: 0x2220,
    },
    interval {
        first: 0x2223,
        last: 0x2223,
    },
    interval {
        first: 0x2225,
        last: 0x2225,
    },
    interval {
        first: 0x2227,
        last: 0x222c,
    },
    interval {
        first: 0x222e,
        last: 0x222e,
    },
    interval {
        first: 0x2234,
        last: 0x2237,
    },
    interval {
        first: 0x223c,
        last: 0x223d,
    },
    interval {
        first: 0x2248,
        last: 0x2248,
    },
    interval {
        first: 0x224c,
        last: 0x224c,
    },
    interval {
        first: 0x2252,
        last: 0x2252,
    },
    interval {
        first: 0x2260,
        last: 0x2261,
    },
    interval {
        first: 0x2264,
        last: 0x2267,
    },
    interval {
        first: 0x226a,
        last: 0x226b,
    },
    interval {
        first: 0x226e,
        last: 0x226f,
    },
    interval {
        first: 0x2282,
        last: 0x2283,
    },
    interval {
        first: 0x2286,
        last: 0x2287,
    },
    interval {
        first: 0x2295,
        last: 0x2295,
    },
    interval {
        first: 0x2299,
        last: 0x2299,
    },
    interval {
        first: 0x22a5,
        last: 0x22a5,
    },
    interval {
        first: 0x22bf,
        last: 0x22bf,
    },
    interval {
        first: 0x2312,
        last: 0x2312,
    },
    interval {
        first: 0x2460,
        last: 0x24e9,
    },
    interval {
        first: 0x24eb,
        last: 0x254b,
    },
    interval {
        first: 0x2550,
        last: 0x2573,
    },
    interval {
        first: 0x2580,
        last: 0x258f,
    },
    interval {
        first: 0x2592,
        last: 0x2595,
    },
    interval {
        first: 0x25a0,
        last: 0x25a1,
    },
    interval {
        first: 0x25a3,
        last: 0x25a9,
    },
    interval {
        first: 0x25b2,
        last: 0x25b3,
    },
    interval {
        first: 0x25b6,
        last: 0x25b7,
    },
    interval {
        first: 0x25bc,
        last: 0x25bd,
    },
    interval {
        first: 0x25c0,
        last: 0x25c1,
    },
    interval {
        first: 0x25c6,
        last: 0x25c8,
    },
    interval {
        first: 0x25cb,
        last: 0x25cb,
    },
    interval {
        first: 0x25ce,
        last: 0x25d1,
    },
    interval {
        first: 0x25e2,
        last: 0x25e5,
    },
    interval {
        first: 0x25ef,
        last: 0x25ef,
    },
    interval {
        first: 0x2605,
        last: 0x2606,
    },
    interval {
        first: 0x2609,
        last: 0x2609,
    },
    interval {
        first: 0x260e,
        last: 0x260f,
    },
    interval {
        first: 0x261c,
        last: 0x261c,
    },
    interval {
        first: 0x261e,
        last: 0x261e,
    },
    interval {
        first: 0x2640,
        last: 0x2640,
    },
    interval {
        first: 0x2642,
        last: 0x2642,
    },
    interval {
        first: 0x2660,
        last: 0x2661,
    },
    interval {
        first: 0x2663,
        last: 0x2665,
    },
    interval {
        first: 0x2667,
        last: 0x266a,
    },
    interval {
        first: 0x266c,
        last: 0x266d,
    },
    interval {
        first: 0x266f,
        last: 0x266f,
    },
    interval {
        first: 0x269e,
        last: 0x269f,
    },
    interval {
        first: 0x26bf,
        last: 0x26bf,
    },
    interval {
        first: 0x26c6,
        last: 0x26cd,
    },
    interval {
        first: 0x26cf,
        last: 0x26d3,
    },
    interval {
        first: 0x26d5,
        last: 0x26e1,
    },
    interval {
        first: 0x26e3,
        last: 0x26e3,
    },
    interval {
        first: 0x26e8,
        last: 0x26e9,
    },
    interval {
        first: 0x26eb,
        last: 0x26f1,
    },
    interval {
        first: 0x26f4,
        last: 0x26f4,
    },
    interval {
        first: 0x26f6,
        last: 0x26f9,
    },
    interval {
        first: 0x26fb,
        last: 0x26fc,
    },
    interval {
        first: 0x26fe,
        last: 0x26ff,
    },
    interval {
        first: 0x273d,
        last: 0x273d,
    },
    interval {
        first: 0x2776,
        last: 0x277f,
    },
    interval {
        first: 0x2b56,
        last: 0x2b59,
    },
    interval {
        first: 0x3248,
        last: 0x324f,
    },
    interval {
        first: 0xe000,
        last: 0xf8ff,
    },
    interval {
        first: 0xfe00,
        last: 0xfe0f,
    },
    interval {
        first: 0xfffd,
        last: 0xfffd,
    },
    interval {
        first: 0x1f100,
        last: 0x1f10a,
    },
    interval {
        first: 0x1f110,
        last: 0x1f12d,
    },
    interval {
        first: 0x1f130,
        last: 0x1f169,
    },
    interval {
        first: 0x1f170,
        last: 0x1f18d,
    },
    interval {
        first: 0x1f18f,
        last: 0x1f190,
    },
    interval {
        first: 0x1f19b,
        last: 0x1f1ac,
    },
    interval {
        first: 0xe0100,
        last: 0xe01ef,
    },
    interval {
        first: 0xf0000,
        last: 0xffffd,
    },
    interval {
        first: 0x100000,
        last: 0x10fffd,
    },
];
pub static mut emoji_all: [interval; 151] = [
    interval {
        first: 0x23,
        last: 0x23,
    },
    interval {
        first: 0x2a,
        last: 0x2a,
    },
    interval {
        first: 0x30,
        last: 0x39,
    },
    interval {
        first: 0xa9,
        last: 0xa9,
    },
    interval {
        first: 0xae,
        last: 0xae,
    },
    interval {
        first: 0x203c,
        last: 0x203c,
    },
    interval {
        first: 0x2049,
        last: 0x2049,
    },
    interval {
        first: 0x2122,
        last: 0x2122,
    },
    interval {
        first: 0x2139,
        last: 0x2139,
    },
    interval {
        first: 0x2194,
        last: 0x2199,
    },
    interval {
        first: 0x21a9,
        last: 0x21aa,
    },
    interval {
        first: 0x231a,
        last: 0x231b,
    },
    interval {
        first: 0x2328,
        last: 0x2328,
    },
    interval {
        first: 0x23cf,
        last: 0x23cf,
    },
    interval {
        first: 0x23e9,
        last: 0x23f3,
    },
    interval {
        first: 0x23f8,
        last: 0x23fa,
    },
    interval {
        first: 0x24c2,
        last: 0x24c2,
    },
    interval {
        first: 0x25aa,
        last: 0x25ab,
    },
    interval {
        first: 0x25b6,
        last: 0x25b6,
    },
    interval {
        first: 0x25c0,
        last: 0x25c0,
    },
    interval {
        first: 0x25fb,
        last: 0x25fe,
    },
    interval {
        first: 0x2600,
        last: 0x2604,
    },
    interval {
        first: 0x260e,
        last: 0x260e,
    },
    interval {
        first: 0x2611,
        last: 0x2611,
    },
    interval {
        first: 0x2614,
        last: 0x2615,
    },
    interval {
        first: 0x2618,
        last: 0x2618,
    },
    interval {
        first: 0x261d,
        last: 0x261d,
    },
    interval {
        first: 0x2620,
        last: 0x2620,
    },
    interval {
        first: 0x2622,
        last: 0x2623,
    },
    interval {
        first: 0x2626,
        last: 0x2626,
    },
    interval {
        first: 0x262a,
        last: 0x262a,
    },
    interval {
        first: 0x262e,
        last: 0x262f,
    },
    interval {
        first: 0x2638,
        last: 0x263a,
    },
    interval {
        first: 0x2640,
        last: 0x2640,
    },
    interval {
        first: 0x2642,
        last: 0x2642,
    },
    interval {
        first: 0x2648,
        last: 0x2653,
    },
    interval {
        first: 0x265f,
        last: 0x2660,
    },
    interval {
        first: 0x2663,
        last: 0x2663,
    },
    interval {
        first: 0x2665,
        last: 0x2666,
    },
    interval {
        first: 0x2668,
        last: 0x2668,
    },
    interval {
        first: 0x267b,
        last: 0x267b,
    },
    interval {
        first: 0x267e,
        last: 0x267f,
    },
    interval {
        first: 0x2692,
        last: 0x2697,
    },
    interval {
        first: 0x2699,
        last: 0x2699,
    },
    interval {
        first: 0x269b,
        last: 0x269c,
    },
    interval {
        first: 0x26a0,
        last: 0x26a1,
    },
    interval {
        first: 0x26aa,
        last: 0x26ab,
    },
    interval {
        first: 0x26b0,
        last: 0x26b1,
    },
    interval {
        first: 0x26bd,
        last: 0x26be,
    },
    interval {
        first: 0x26c4,
        last: 0x26c5,
    },
    interval {
        first: 0x26c8,
        last: 0x26c8,
    },
    interval {
        first: 0x26ce,
        last: 0x26cf,
    },
    interval {
        first: 0x26d1,
        last: 0x26d1,
    },
    interval {
        first: 0x26d3,
        last: 0x26d4,
    },
    interval {
        first: 0x26e9,
        last: 0x26ea,
    },
    interval {
        first: 0x26f0,
        last: 0x26f5,
    },
    interval {
        first: 0x26f7,
        last: 0x26fa,
    },
    interval {
        first: 0x26fd,
        last: 0x26fd,
    },
    interval {
        first: 0x2702,
        last: 0x2702,
    },
    interval {
        first: 0x2705,
        last: 0x2705,
    },
    interval {
        first: 0x2708,
        last: 0x270d,
    },
    interval {
        first: 0x270f,
        last: 0x270f,
    },
    interval {
        first: 0x2712,
        last: 0x2712,
    },
    interval {
        first: 0x2714,
        last: 0x2714,
    },
    interval {
        first: 0x2716,
        last: 0x2716,
    },
    interval {
        first: 0x271d,
        last: 0x271d,
    },
    interval {
        first: 0x2721,
        last: 0x2721,
    },
    interval {
        first: 0x2728,
        last: 0x2728,
    },
    interval {
        first: 0x2733,
        last: 0x2734,
    },
    interval {
        first: 0x2744,
        last: 0x2744,
    },
    interval {
        first: 0x2747,
        last: 0x2747,
    },
    interval {
        first: 0x274c,
        last: 0x274c,
    },
    interval {
        first: 0x274e,
        last: 0x274e,
    },
    interval {
        first: 0x2753,
        last: 0x2755,
    },
    interval {
        first: 0x2757,
        last: 0x2757,
    },
    interval {
        first: 0x2763,
        last: 0x2764,
    },
    interval {
        first: 0x2795,
        last: 0x2797,
    },
    interval {
        first: 0x27a1,
        last: 0x27a1,
    },
    interval {
        first: 0x27b0,
        last: 0x27b0,
    },
    interval {
        first: 0x27bf,
        last: 0x27bf,
    },
    interval {
        first: 0x2934,
        last: 0x2935,
    },
    interval {
        first: 0x2b05,
        last: 0x2b07,
    },
    interval {
        first: 0x2b1b,
        last: 0x2b1c,
    },
    interval {
        first: 0x2b50,
        last: 0x2b50,
    },
    interval {
        first: 0x2b55,
        last: 0x2b55,
    },
    interval {
        first: 0x3030,
        last: 0x3030,
    },
    interval {
        first: 0x303d,
        last: 0x303d,
    },
    interval {
        first: 0x3297,
        last: 0x3297,
    },
    interval {
        first: 0x3299,
        last: 0x3299,
    },
    interval {
        first: 0x1f004,
        last: 0x1f004,
    },
    interval {
        first: 0x1f0cf,
        last: 0x1f0cf,
    },
    interval {
        first: 0x1f170,
        last: 0x1f171,
    },
    interval {
        first: 0x1f17e,
        last: 0x1f17f,
    },
    interval {
        first: 0x1f18e,
        last: 0x1f18e,
    },
    interval {
        first: 0x1f191,
        last: 0x1f19a,
    },
    interval {
        first: 0x1f1e6,
        last: 0x1f1ff,
    },
    interval {
        first: 0x1f201,
        last: 0x1f202,
    },
    interval {
        first: 0x1f21a,
        last: 0x1f21a,
    },
    interval {
        first: 0x1f22f,
        last: 0x1f22f,
    },
    interval {
        first: 0x1f232,
        last: 0x1f23a,
    },
    interval {
        first: 0x1f250,
        last: 0x1f251,
    },
    interval {
        first: 0x1f300,
        last: 0x1f321,
    },
    interval {
        first: 0x1f324,
        last: 0x1f393,
    },
    interval {
        first: 0x1f396,
        last: 0x1f397,
    },
    interval {
        first: 0x1f399,
        last: 0x1f39b,
    },
    interval {
        first: 0x1f39e,
        last: 0x1f3f0,
    },
    interval {
        first: 0x1f3f3,
        last: 0x1f3f5,
    },
    interval {
        first: 0x1f3f7,
        last: 0x1f4fd,
    },
    interval {
        first: 0x1f4ff,
        last: 0x1f53d,
    },
    interval {
        first: 0x1f549,
        last: 0x1f54e,
    },
    interval {
        first: 0x1f550,
        last: 0x1f567,
    },
    interval {
        first: 0x1f56f,
        last: 0x1f570,
    },
    interval {
        first: 0x1f573,
        last: 0x1f57a,
    },
    interval {
        first: 0x1f587,
        last: 0x1f587,
    },
    interval {
        first: 0x1f58a,
        last: 0x1f58d,
    },
    interval {
        first: 0x1f590,
        last: 0x1f590,
    },
    interval {
        first: 0x1f595,
        last: 0x1f596,
    },
    interval {
        first: 0x1f5a4,
        last: 0x1f5a5,
    },
    interval {
        first: 0x1f5a8,
        last: 0x1f5a8,
    },
    interval {
        first: 0x1f5b1,
        last: 0x1f5b2,
    },
    interval {
        first: 0x1f5bc,
        last: 0x1f5bc,
    },
    interval {
        first: 0x1f5c2,
        last: 0x1f5c4,
    },
    interval {
        first: 0x1f5d1,
        last: 0x1f5d3,
    },
    interval {
        first: 0x1f5dc,
        last: 0x1f5de,
    },
    interval {
        first: 0x1f5e1,
        last: 0x1f5e1,
    },
    interval {
        first: 0x1f5e3,
        last: 0x1f5e3,
    },
    interval {
        first: 0x1f5e8,
        last: 0x1f5e8,
    },
    interval {
        first: 0x1f5ef,
        last: 0x1f5ef,
    },
    interval {
        first: 0x1f5f3,
        last: 0x1f5f3,
    },
    interval {
        first: 0x1f5fa,
        last: 0x1f64f,
    },
    interval {
        first: 0x1f680,
        last: 0x1f6c5,
    },
    interval {
        first: 0x1f6cb,
        last: 0x1f6d2,
    },
    interval {
        first: 0x1f6d5,
        last: 0x1f6d5,
    },
    interval {
        first: 0x1f6e0,
        last: 0x1f6e5,
    },
    interval {
        first: 0x1f6e9,
        last: 0x1f6e9,
    },
    interval {
        first: 0x1f6eb,
        last: 0x1f6ec,
    },
    interval {
        first: 0x1f6f0,
        last: 0x1f6f0,
    },
    interval {
        first: 0x1f6f3,
        last: 0x1f6fa,
    },
    interval {
        first: 0x1f7e0,
        last: 0x1f7eb,
    },
    interval {
        first: 0x1f90d,
        last: 0x1f93a,
    },
    interval {
        first: 0x1f93c,
        last: 0x1f945,
    },
    interval {
        first: 0x1f947,
        last: 0x1f971,
    },
    interval {
        first: 0x1f973,
        last: 0x1f976,
    },
    interval {
        first: 0x1f97a,
        last: 0x1f9a2,
    },
    interval {
        first: 0x1f9a5,
        last: 0x1f9aa,
    },
    interval {
        first: 0x1f9ae,
        last: 0x1f9ca,
    },
    interval {
        first: 0x1f9cd,
        last: 0x1f9ff,
    },
    interval {
        first: 0x1fa70,
        last: 0x1fa73,
    },
    interval {
        first: 0x1fa78,
        last: 0x1fa7a,
    },
    interval {
        first: 0x1fa80,
        last: 0x1fa82,
    },
    interval {
        first: 0x1fa90,
        last: 0x1fa95,
    },
];
pub static mut emoji_width: [interval; 39] = [
    interval {
        first: 0x1f1e6,
        last: 0x1f1ff,
    },
    interval {
        first: 0x1f321,
        last: 0x1f321,
    },
    interval {
        first: 0x1f324,
        last: 0x1f32c,
    },
    interval {
        first: 0x1f336,
        last: 0x1f336,
    },
    interval {
        first: 0x1f37d,
        last: 0x1f37d,
    },
    interval {
        first: 0x1f396,
        last: 0x1f397,
    },
    interval {
        first: 0x1f399,
        last: 0x1f39b,
    },
    interval {
        first: 0x1f39e,
        last: 0x1f39f,
    },
    interval {
        first: 0x1f3cb,
        last: 0x1f3ce,
    },
    interval {
        first: 0x1f3d4,
        last: 0x1f3df,
    },
    interval {
        first: 0x1f3f3,
        last: 0x1f3f5,
    },
    interval {
        first: 0x1f3f7,
        last: 0x1f3f7,
    },
    interval {
        first: 0x1f43f,
        last: 0x1f43f,
    },
    interval {
        first: 0x1f441,
        last: 0x1f441,
    },
    interval {
        first: 0x1f4fd,
        last: 0x1f4fd,
    },
    interval {
        first: 0x1f549,
        last: 0x1f54a,
    },
    interval {
        first: 0x1f56f,
        last: 0x1f570,
    },
    interval {
        first: 0x1f573,
        last: 0x1f579,
    },
    interval {
        first: 0x1f587,
        last: 0x1f587,
    },
    interval {
        first: 0x1f58a,
        last: 0x1f58d,
    },
    interval {
        first: 0x1f590,
        last: 0x1f590,
    },
    interval {
        first: 0x1f5a5,
        last: 0x1f5a5,
    },
    interval {
        first: 0x1f5a8,
        last: 0x1f5a8,
    },
    interval {
        first: 0x1f5b1,
        last: 0x1f5b2,
    },
    interval {
        first: 0x1f5bc,
        last: 0x1f5bc,
    },
    interval {
        first: 0x1f5c2,
        last: 0x1f5c4,
    },
    interval {
        first: 0x1f5d1,
        last: 0x1f5d3,
    },
    interval {
        first: 0x1f5dc,
        last: 0x1f5de,
    },
    interval {
        first: 0x1f5e1,
        last: 0x1f5e1,
    },
    interval {
        first: 0x1f5e3,
        last: 0x1f5e3,
    },
    interval {
        first: 0x1f5e8,
        last: 0x1f5e8,
    },
    interval {
        first: 0x1f5ef,
        last: 0x1f5ef,
    },
    interval {
        first: 0x1f5f3,
        last: 0x1f5f3,
    },
    interval {
        first: 0x1f5fa,
        last: 0x1f5fa,
    },
    interval {
        first: 0x1f6cb,
        last: 0x1f6cf,
    },
    interval {
        first: 0x1f6e0,
        last: 0x1f6e5,
    },
    interval {
        first: 0x1f6e9,
        last: 0x1f6e9,
    },
    interval {
        first: 0x1f6f0,
        last: 0x1f6f0,
    },
    interval {
        first: 0x1f6f3,
        last: 0x1f6f3,
    },
];
