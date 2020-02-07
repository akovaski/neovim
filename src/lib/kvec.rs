use crate::*;
use std::mem;
use std::ptr;

/// Type of a vector with a few first members allocated on stack
///
/// Is compatible with #kv_A, #kv_pop, #kv_size, #kv_max, #kv_last.
/// Is not compatible with #kv_resize, #kv_resize_full, #kv_copy, #kv_push,
/// #kv_pushp, #kv_a, #kv_destroy.
///
/// @param[in]  type  Type of vector elements.
/// @param[in]  init_size  Number of the elements in the initial array.
macro_rules! kvec_withinit_t {
    ($type: ty, $init_size: expr) => {
        kvec_i<$type, [$type; $init_size]>
    };
    ($type: ty, $init_size: expr) => {
        kvec_i<$type, [$type; $init_size]>
    };
}
#[repr(C)]
#[allow(dead_code)]
pub struct kvec_i<T, A> {
    size: libc::size_t,
    capacity: libc::size_t,
    pub items: *mut T,
    init_array: A,
}
pub trait EmptyArray<A> {
    fn empty_array() -> A;
    fn capacity() -> libc::size_t;
}
macro_rules! define_ea {
    ($num: literal) => {
        impl<T: Copy> EmptyArray<[T; $num]> for [T; $num] {
            fn empty_array() -> [T; $num] {
                [unsafe { mem::zeroed() }; $num]
            }
            fn capacity() -> libc::size_t {
                $num
            }
        }
    };
}
define_ea!(4);
define_ea!(10);
define_ea!(16);
#[allow(dead_code)]
impl<T, A> kvec_i<T, A>
where
    T: Copy,
    A: AsRef<[T]> + AsMut<[T]> + EmptyArray<A>,
{
    fn sizeof_item(&self) -> usize {
        mem::size_of::<T>() as usize
    }

    /// Initialize vector with preallocated array
    pub fn init() -> Box<Self> {
        let ret = Self {
            size: 0,
            capacity: A::capacity(),
            items: std::ptr::null_mut(),
            init_array: A::empty_array(),
        };
        let mut ret = Box::new(ret);
        ret.items = ret.init_array.as_mut().as_mut_ptr();
        ret
    }
    pub unsafe fn A(&mut self, i: usize) -> &mut T {
        &mut *self.items.offset(i as isize)
    }
    pub unsafe fn pop(&mut self) -> T {
        self.size -= 1;
        *self.items.offset(self.size as isize)
    }
    pub fn size(&self) -> libc::size_t {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub unsafe fn Z(&mut self, i: usize) -> &mut T {
        self.A(self.size() - i - 1)
    }
    pub unsafe fn last(&mut self) -> &mut T {
        self.Z(0)
    }

    /// Drop last n items from kvec without resizing
    ///
    /// Previously spelled as `(void)kv_pop(v)`, repeated n times.
    ///
    /// @param[out]  v  Kvec to drop items from.
    /// @param[in]  n  Number of elements to drop.
    pub unsafe fn lop(&mut self, n: usize) {
        self.size -= n;
    }

    /// Resize vector with preallocated array
    ///
    /// @note May not resize to an array smaller then init_array: if requested,
    ///       init_array will be used.
    ///
    /// @param[out]  v  Vector to resize.
    /// @param[in]  s  New size.
    unsafe fn resize(&mut self, size: libc::size_t) {
        self.capacity = std::cmp::max(size, self.init_array.as_ref().len());
        self.items = if self.capacity == self.init_array.as_ref().len() {
            if self.items == self.init_array.as_mut().as_mut_ptr() {
                self.items as *mut libc::c_void
            } else {
                _memcpy_free(
                    self.init_array.as_mut().as_mut_ptr() as *mut libc::c_void,
                    self.items as *mut libc::c_void,
                    self.size * self.sizeof_item(),
                )
            }
        } else {
            if self.items == self.init_array.as_mut().as_mut_ptr() {
                memcpy::<libc::c_void, _>(
                    xmalloc(self.capacity * self.sizeof_item()),
                    self.items as *const libc::c_void,
                    self.size * self.sizeof_item(),
                )
            } else {
                xrealloc(
                    self.items as *mut libc::c_void,
                    self.capacity * self.sizeof_item(),
                )
            }
        } as *mut _;
    }
    /// Resize vector with preallocated array when it is full
    ///
    /// @param[out]  v  Vector to resize.
    unsafe fn resize_full(&mut self) {
        // ARRAY_SIZE((v).init_array) is the minimal capacity of this vector.
        // Thus when vector is full capacity may not be zero and it is safe
        // not to bother with checking whether (v).capacity is 0. But now
        // capacity is not guaranteed to have size that is a power of 2, it is
        // hard to fix this here and is not very necessary if users will use
        // 2^x initial array size. */ \
        self.resize(self.capacity << 1);
    }

    /// Push value to a vector with preallocated array
    ///
    /// @param[out]  v  Vector to push to.
    /// @param[in]  x  Value to push.
    pub unsafe fn push(&mut self, x: T) {
        if self.size == self.capacity {
            self.resize_full();
        }
        let ref mut fresh = *self.items.offset(self.size as isize);
        self.size = self.size.wrapping_add(1);
        *fresh = x;
    }

    /// Free array of elements of a vector with preallocated array if needed
    ///
    /// @param[out]  v  Vector to free.
    pub unsafe fn destroy(&mut self) {
        if self.items != self.init_array.as_mut().as_mut_ptr() {
            XFREE_CLEAR(&mut self.items);
        };
    }
}

/// Move data to a new destination and free source
#[inline]
unsafe extern "C" fn _memcpy_free(
    dest: *mut libc::c_void,
    src: *mut libc::c_void,
    size: libc::size_t,
) -> *mut libc::c_void {
    memcpy(dest, src, size);
    let ptr_ = &src as *const *mut libc::c_void as *mut *mut libc::c_void;
    xfree(*ptr_);
    *ptr_ = ptr::null_mut();
    return dest;
}
