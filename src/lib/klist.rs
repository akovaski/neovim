/* The MIT License

   Copyright (c) 2008-2009, by Attractive Chaos <attractor@live.co.uk>

   Permission is hereby granted, free of charge, to any person obtaining
   a copy of this software and associated documentation files (the
   "Software"), to deal in the Software without restriction, including
   without limitation the rights to use, copy, modify, merge, publish,
   distribute, sublicense, and/or sell copies of the Software, and to
   permit persons to whom the Software is furnished to do so, subject to
   the following conditions:

   The above copyright notice and this permission notice shall be
   included in all copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
   EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
   NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
   BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
   ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
   CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
   SOFTWARE.
*/

use crate::memory::*;
use std::ptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kl1<T> {
    pub data: T,
    pub next: *mut kl1<T>,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct kmp_t<T> {
    cnt: libc::size_t,
    n: libc::size_t,
    max: libc::size_t,
    buf: *mut *mut kl1<T>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kl_t<T> {
    pub head: *mut kl1<T>,
    pub tail: *mut kl1<T>,
    mp: *mut kmp_t<T>,
    pub size: libc::size_t,
}

unsafe fn kmp_init<T>() -> *mut kmp_t<T> {
    return xcalloc(1, std::mem::size_of::<kmp_t<T>>());
}
unsafe fn kmp_destroy<T>(mut mp: *mut kmp_t<T>) {
    for k in 0..(*mp).n {
        XFREE_CLEAR(&mut *(*mp).buf.offset(k as isize));
    }
    XFREE_CLEAR(&mut (*mp).buf);
    XFREE_CLEAR(&mut mp);
}
unsafe fn kmp_alloc<T>(mut mp: *mut kmp_t<T>) -> *mut kl1<T> {
    (*mp).cnt += 1;
    if (*mp).n == 0 {
        return xcalloc(1, std::mem::size_of::<kl1<T>>());
    }
    (*mp).n -= 1;
    return *(*mp).buf.offset((*mp).n as isize);
}
unsafe fn kmp_free<T>(mut mp: *mut kmp_t<T>, p: *mut kl1<T>) {
    (*mp).cnt -= 1;
    if (*mp).n == (*mp).max {
        (*mp).max = if (*mp).max != 0 { ((*mp).max) << 1 } else { 16 };
        (*mp).buf = xrealloc((*mp).buf, std::mem::size_of::<*mut kl1<T>>() * (*mp).max);
    }
    let fresh1 = &mut *(*mp).buf.offset((*mp).n as isize);
    *fresh1 = p;
    (*mp).n += 1
}

pub unsafe fn kl_init<T>() -> *mut kl_t<T> {
    let mut kl: *mut kl_t<T> = xcalloc(1, std::mem::size_of::<kl_t<T>>());
    (*kl).mp = kmp_init();
    (*kl).tail = kmp_alloc((*kl).mp);
    (*kl).head = (*kl).tail;
    (*(*kl).head).next = ptr::null_mut();
    return kl;
}
pub unsafe fn kl_destroy<T>(mut kl: *mut kl_t<T>) {
    let mut p: *mut kl1<T> = (*kl).head;
    while p != (*kl).tail {
        kmp_free((*kl).mp, p);
        p = (*p).next
    }
    kmp_free((*kl).mp, p);
    kmp_destroy((*kl).mp);
    XFREE_CLEAR(&mut kl);
}
pub unsafe extern "C" fn kl_push<T, D: Into<T>>(mut kl: *mut kl_t<T>, d: D) {
    let mut p: *mut kl1<T> = kmp_alloc((*kl).mp);
    let mut q: *mut kl1<T> = (*kl).tail;
    (*p).next = ptr::null_mut();
    (*(*kl).tail).next = p;
    (*kl).tail = p;
    (*kl).size = (*kl).size.wrapping_add(1);
    (*q).data = d.into();
}

pub unsafe extern "C" fn kl_shift_at<T: Copy>(mut kl: *mut kl_t<T>, n: *mut *mut kl1<T>) -> T {
    c_assert!(!(**n).next.is_null());
    (*kl).size = (*kl).size.wrapping_sub(1);
    let p: *mut kl1<T> = *n;
    *n = (**n).next;
    if p == (*kl).head {
        (*kl).head = *n
    }
    let d: T = (*p).data;
    kmp_free((*kl).mp, p);
    return d;
}

pub unsafe fn kl_empty<T>(kl: *mut kl_t<T>) -> bool {
    (*kl).size == 0
}

impl<T> kl_t<T> {
    pub fn iter(&self) -> klIterator<T> {
        klIterator {
            kl: self,
            p: &(self.head),
        }
    }
}

pub struct klIterator<'a, T> {
    kl: &'a kl_t<T>,
    p: &'a *mut kl1<T>,
}

impl<'a, T> Iterator for klIterator<'a, T> {
    type Item = *mut *mut kl1<T>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if *self.p == (*self.kl).tail {
                None
            } else {
                let ret = self.p;
                self.p = &(**self.p).next;
                Some(ret as *const _ as *mut _)
            }
        }
    }
}
