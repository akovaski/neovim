use crate::*;

pub type Window = handle_T;
pub type Boolean = bool;
pub type Integer = i64;
pub type Float = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct String_0 {
    pub data: *mut libc::c_char,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object {
    pub type_0: ObjectType,
    pub data: objectData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union objectData {
    pub boolean: Boolean,
    pub integer: Integer,
    pub floating: Float,
    pub string: String_0,
    pub array: Array,
    pub dictionary: Dictionary,
    pub luaref: LuaRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dictionary {
    pub items: *mut KeyValuePair,
    pub size: size_t,
    pub capacity: size_t,
}
pub type KeyValuePair = key_value_pair;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_value_pair {
    pub key: String_0,
    pub value: Object,
}
pub type Object = object;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Array {
    pub items: *mut Object,
    pub size: size_t,
    pub capacity: size_t,
}
pub type ObjectType = libc::c_uint;
pub const kObjectTypeTabpage: ObjectType = 10;
pub const kObjectTypeWindow: ObjectType = 9;
pub const kObjectTypeBuffer: ObjectType = 8;
pub const kObjectTypeLuaRef: ObjectType = 7;
pub const kObjectTypeDictionary: ObjectType = 6;
pub const kObjectTypeArray: ObjectType = 5;
pub const kObjectTypeString: ObjectType = 4;
pub const kObjectTypeFloat: ObjectType = 3;
pub const kObjectTypeInteger: ObjectType = 2;
pub const kObjectTypeBoolean: ObjectType = 1;
pub const kObjectTypeNil: ObjectType = 0;
#[inline(always)]
pub unsafe extern "C" fn is_internal_call(channel_id: u64) -> bool {
    return channel_id
        & (1 as libc::c_int as u64)
            << (::std::mem::size_of::<u64>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        != 0;
}
