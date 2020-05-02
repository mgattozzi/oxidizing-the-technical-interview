#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(const_panic)]
#![feature(const_if_match)]
#![feature(const_raw_ptr_deref)]
#![feature(lang_items)]
#![feature(no_core)]
#![feature(optin_builtin_traits)]
#![feature(allow_internal_unstable)]
#![feature(intrinsics)]
#![feature(untagged_unions)]
#![no_core]
#[link(name = "System")]
extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
}
#[allow(non_camel_case_types)]
type c_char = i8;
#[allow(non_camel_case_types)]
type c_int = i32;
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    }
}
macro_rules! copy_clone_eq_impls {
  ($($t:ty)*) => {
    $(
    impl Copy for $t {}
    impl Clone for $t {
      fn clone(&self) -> Self {
         *self
      }
    }
    impl PartialEq for $t {
        fn eq(&self, other: &$t) -> bool {
            (*self) == (*other)
        }
    }
    )*
  }
}
#[lang = "sized"]
pub trait Sized {}
#[lang = "freeze"]
auto trait Freeze {}
extern "rust-intrinsic" {
    fn offset<T>(dst: *const T, offset: isize) -> *const T;
}
#[lang = "receiver"]
trait Receiver {}
#[lang = "index"]
trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}
impl<T, I> Index<I> for [T]
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;
    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}
trait SliceIndex<T: ?Sized> {
    type Output: ?Sized;
    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked(self, slice: &T) -> &Self::Output;
    unsafe fn get_unchecked_mut(self, slice: &mut T) -> &mut Self::Output;
    fn index(self, slice: &T) -> &Self::Output;
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}
#[lang = "copy"]
trait Copy: Clone {}
trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
#[lang = "eq"]
trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}
#[lang = "partial_ord"]
trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }
    fn le(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less) | Some(Equal))
    }
    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }
    fn ge(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater) | Some(Equal))
    }
}
#[lang = "not"]
trait Not {
    type Output;

    fn not(self) -> Self::Output;
}
#[lang = "neg"]
trait Neg {
    type Output;
    fn neg(self) -> Self::Output;
}
#[lang = "sub"]
trait Sub<Rhs = Self> {
    type Output;
    fn sub(self, rhs: Rhs) -> Self::Output;
}
trait Termination {
    fn report(self) -> i32;
}
#[lang = "slice"]
impl<T> [T] {
    #[allow(unused_attributes)]
    #[allow_internal_unstable(const_fn_union)]
    const fn len(&self) -> usize {
        unsafe { Repr { rust: self }.raw.len }
    }

    fn as_ptr(&self) -> *const T {
        self as *const [T] as *const T
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut [T] as *mut T
    }
}

#[lang = "const_ptr"]
impl<T: ?Sized> *const T {
    unsafe fn add(self, count: usize) -> Self
    where
        T: Sized,
    {
        self.offset(count as isize)
    }

    unsafe fn offset(self, count: isize) -> *const T
    where
        T: Sized,
    {
        offset(self, count)
    }
}
#[lang = "mut_ptr"]
impl<T: ?Sized> *mut T {
    unsafe fn add(self, count: usize) -> *mut T
    where
        T: Sized,
    {
        self.offset(count as isize)
    }
    unsafe fn offset(self, count: isize) -> *mut T
    where
        T: Sized,
    {
        offset(self, count) as *mut T
    }
}
impl<T> SliceIndex<[T]> for usize {
    type Output = T;
    fn get(self, slice: &[T]) -> Option<&T> {
        if self < slice.len() {
            unsafe { Some(self.get_unchecked(slice)) }
        } else {
            None
        }
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut T> {
        if self < slice.len() {
            unsafe { Some(self.get_unchecked_mut(slice)) }
        } else {
            None
        }
    }
    unsafe fn get_unchecked(self, slice: &[T]) -> &T {
        &*slice.as_ptr().add(self)
    }
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut T {
        &mut *slice.as_mut_ptr().add(self)
    }
    fn index(self, slice: &[T]) -> &T {
        &(*slice)[self]
    }
    fn index_mut(self, slice: &mut [T]) -> &mut T {
        &mut (*slice)[self]
    }
}
#[allow(unconditional_recursion)]
impl PartialOrd for usize {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.partial_cmp(other)
    }
}
impl Not for bool {
    type Output = bool;
    fn not(self) -> bool {
        !self
    }
}
impl Neg for isize {
    type Output = isize;

    fn neg(self) -> isize {
        -self
    }
}
impl Sub for usize {
    type Output = usize;
    fn sub(self, rhs: usize) -> Self::Output {
        self - rhs
    }
}
impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}
impl<T: ?Sized> Receiver for &T {}
impl<T: ?Sized> Receiver for &mut T {}
copy_clone_eq_impls!(usize u64 bool);

#[allow(dead_code)] // It is actually needed! Zombie Ordering
enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
use Ordering::*;
enum Option<T> {
    Some(T),
    None,
}
use crate::Option::*;
#[repr(C)]
pub(crate) union Repr<T> {
    pub(crate) rust: *const [T],
    rust_mut: *mut [T],
    pub(crate) raw: FatPtr<T>,
}
#[repr(C)]
pub(crate) struct FatPtr<T> {
    data: *const T,
    pub(crate) len: usize,
}

#[lang = "start"]
fn start<T: Termination + 'static>(main: fn() -> T, _: isize, _: *const *const u8) -> isize {
    main().report() as isize
}

fn main() {
    unsafe {
        printf(
            "The value is: %d\n\0" as *const str as *const u8 as *const c_char,
            DUPLICATE,
        );
    }
}

const DUPLICATE: u64 = {
    const LIST: &'static [u64] = {
        unsafe {
            &*Repr {
                raw: FatPtr {
                    data: &[
                        1u64, 2u64, 3u64, 7u64, 4u64, 5u64, 6u64, 7u64, 8u64, 9u64, 10u64,
                    ] as *const u64,
                    len: 11,
                },
            }
            .rust
        }
    };

    const fn recurse(item: usize, compare: usize) -> u64 {
        if compare == 0usize && item != 0usize {
            recurse(item - 1, 10)
        }
        // else if compare == 0 && item == 0 {
        //     // Pigeon Hole principle but panics bloat the lib
        // }
        else if compare == item {
            recurse(item, compare - 1)
        } else {
            if LIST[compare] == LIST[item] {
                return LIST[compare];
            } else {
                recurse(item, compare - 1)
            }
        }
    }
    recurse(10, 10)
};
