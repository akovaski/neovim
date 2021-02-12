if [ "$#" -ne 1 ]; then
    echo "Accepts one file as argument. Goodbye."
    exit 1
fi

# rustfmt --edition 2018 --config max_width=600 $1

echo "hash thingies"
sed -i '/^\s*#\[c2rust::src_loc .*\]$/d' $1
sed -i '/^\s*#\[c2rust::header_src .*\]$/d' $1

echo "integers"
#sed -i '/^\s*$/d' $1
sed -i 's/\<char_u\>/u8/g' $1
sed -i '/^\s*use super::nvim_types_h::u8;$/d' $1

sed -i 's/\<uint8_t\>/u8/g' $1
sed -i 's/\<uint32_t\>/u32/g' $1
sed -i 's/\<uint64_t\>/u64/g' $1
sed -i 's/\<libc::size_t\>/size_t/g' $1
sed -i '/^\s*use super::stdint_uintn_h::u8;$/d' $1
sed -i '/^\s*use super::stdint_uintn_h::u32;$/d' $1
sed -i '/^\s*use super::stdint_uintn_h::u64;$/d' $1
sed -i 's/\<libc::c_char\>/i8/g' $1
sed -i 's/\<libc::c_short\>/i16/g' $1
sed -i 's/\<libc::c_ushort\>/u16/g' $1
sed -i 's/\<libc::c_int\>/i32/g' $1
sed -i 's/\<libc::c_uint\>/u32/g' $1
sed -i 's/\<libc::c_long\>/i64/g' $1
sed -i 's/\<libc::c_ulong\>/u64/g' $1

sed -i 's/\<int8_t\>/i8/g' $1
sed -i 's/\<int32_t\>/i32/g' $1
sed -i 's/\<int64_t\>/i64/g' $1
sed -i '/^\s*use super::stdint_intn_h::i8;$/d' $1
sed -i '/^\s*use super::stdint_intn_h::i32;$/d' $1
sed -i '/^\s*use super::stdint_intn_h::i64;$/d' $1

sed -i '/^\s*use super::regexp_defs_h::regmatch_T;$/d' $1

echo "null pointers"
sed -i 's/NULL as \(\*mut \)\+[a-zA-Z0-9_:]\+/ptr::null_mut()/g' $1
sed -i 's/NULL as \(\*const \)\+[a-zA-Z0-9_:]\+/ptr::null_mut()/g' $1

echo "simple logic"
sed -i 's/wrapping_offset_from/offset_from/g' $1
sed -i 's/false_0 != 0/false/g' $1
sed -i 's/true_0 != 0/true/g' $1

echo "intricate replacements"
perl -i -0pe 's/gettext\(b"([^"]*)\\x00"(\s+as\s+\*\S+\s+[a-zA-Z0-9:_]+)+\)/gettext\("$1"\)/g' $1
perl -i -0pe 's/(size_of::<[^>]+>\(\))\s+as\s+libc::c_ulong/$1/g' $1
perl -i -0pe 's/([(,\s]\d+)(\s+as\s+[a-zA-Z0-9:_]+)+/$1/g' $1
perl -i -0pe 's/__assert_fail\(b"(.*)\\x00"[^)]*.*\.as_ptr\(\)\)/assert!(false, "$1")/g' $1

echo "remove ported mods"
sed -i '/^pub mod log_h_generated_h/,/^}/d' $1
sed -i '/^pub mod mbyte_h_generated_h/,/^}/d' $1
sed -i '/^pub mod memory_h_generated_h/,/^}/d' $1
sed -i '/^pub mod message_h_generated_h/,/^}/d' $1
sed -i '/^pub mod env_h_generated_h/,/^}/d' $1
sed -i '/^pub mod fs_h_generated_h/,/^}/d' $1
sed -i '/^pub mod stdpaths_h_generated_h/,/^}/d' $1
sed -i '/^pub mod buffer_h_generated_h/,/^}/d' $1
sed -i '/^pub mod map_h_generated_h/,/^}/d' $1
sed -i '/^pub mod process_h_generated_h/,/^}/d' $1
sed -i '/^pub mod signal_h_generated_h/,/^}/d' $1

sed -i '/^\(pub \)\?use self::.*{[^}]*$/,/;/d' $1
sed -i '/^ *\(pub \)\?use .*;/d' $1
echo "insert use crate::\*"; sed -i '1iuse crate::*;' $1


# rustfmt --edition 2018 $1
