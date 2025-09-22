//! Raylib-managed memory. Similar in function to [`Box`], but specifically
//! designed for `RL_MALLOC`/`RL_REALLOC`/`RL_FREE`.
//!
//! See [`DataBuf`]

#![warn(clippy::style, clippy::pedantic, clippy::perf)]
#![allow(
    clippy::missing_errors_doc,
    reason = "errors are documented at their defintion, not by the functions that use them"
)]
#![warn(
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    reason = "minimize confusion over what is and isn't safe"
)]
#![deny(
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block,
    reason = "DataBuf is delicate and assumptions must be clearly declared and scoped"
)]

//! Data manipulation functions. Compress and Decompress with DEFLATE
use std::{
    alloc::Layout,
    marker::PhantomData,
    mem::MaybeUninit,
    num::{NonZeroU32, NonZeroUsize},
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{error::AllocationError, ffi};

/// Calculate the number of bytes needed to allocate `layout`.
#[inline]
fn allocation_layout_size(layout: Layout) -> Result<NonZeroU32, AllocationError> {
    let bytes = layout
        .size()
        .try_into()
        .ok()
        .ok_or(AllocationError::IntoUIntFailed)?;

    NonZeroU32::new(bytes).ok_or(AllocationError::ZeroBytes)
}

/// Calculate the number of bytes needed to allocate `[T; count]`.
#[inline]
fn allocation_array_size<T>(count: usize) -> Result<NonZeroU32, AllocationError> {
    allocation_layout_size(Layout::array::<T>(count).map_err(|_| AllocationError::IntoUIntFailed)?)
}

/// Calculate the number of bytes needed to allocate a copy of `val`
#[inline]
fn allocation_val_size<T: ?Sized>(val: &T) -> Result<NonZeroU32, AllocationError> {
    allocation_layout_size(Layout::for_value(val))
}

mod rl_managed {
    use super::ffi;
    use std::{mem::MaybeUninit, num::NonZeroU32, ptr::NonNull};

    /// Raylib-managed [`NonNull`].
    ///
    /// # Safety
    ///
    /// [`RlManaged`] must be unique, not dangling, and allocated with `RL_ALLOC`/[`ffi::MemAlloc`]
    /// or `RL_REALLOC`/[`ffi::MemRealloc`].
    ///
    /// **However**, it is *not* guaranteed to be safe to dereference. It is the user's responsibility
    /// to ensure the pointer stored in [`RlManaged`] is safe to dereference as the type it claims to
    /// be, for the count it claims to be, before dereferencing.
    ///
    /// [`RlManaged`] does not guarantee the size, alignment, nor validity of its allocation -- **it
    /// only guarantees that its memory block is safe for Raylib to re/deallocate**.
    #[repr(transparent)]
    #[derive(Debug)]
    #[must_use]
    pub struct RlManaged<T: ?Sized>(/* unsafe */ NonNull<T>);

    /// Allocate `size` bytes of memory aligned to `T` using [`ffi::MemAlloc`].
    ///
    /// Returns [`None`] if [`ffi::MemAlloc`] returned null.
    #[inline]
    pub fn mem_alloc<T>(size: NonZeroU32) -> Option<RlManaged<MaybeUninit<T>>> {
        // SAFETY: `size` is not zero.
        let ptr = unsafe { ffi::MemAlloc(size.get()) }.cast();
        NonNull::new(ptr).map(RlManaged)
    }

    impl<T: ?Sized> RlManaged<T> {
        /// Mark memory as Raylib-managed.
        ///
        /// # Safety
        ///
        /// `data` must be unique, not dangling, and allocated with `RL_ALLOC`/[`ffi::MemAlloc`] or `RL_REALLOC`/[`ffi::MemRealloc`].
        #[inline]
        pub(crate) const unsafe fn new(data: NonNull<T>) -> Self {
            Self(data)
        }

        /// Returns a shared reference to the value.
        ///
        /// # Safety
        ///
        /// `self` must be safe to dereference as `T`.
        /// See [`RlManaged`] safety for more information.
        #[inline]
        #[must_use]
        pub const unsafe fn as_ref(&self) -> &T {
            // SAFETY: RlManaged must be unique and cannot dangle.
            // Taking `self` by reference guarantees aliasing rules are followed.
            // Caller must ensure `self` is safe to dereference.
            unsafe { self.0.as_ref() }
        }

        /// Returns a unique reference to the value.
        ///
        /// # Safety
        ///
        /// `self` must be safe to dereference as `T`.
        /// See [`RlManaged`] safety for more information.
        #[inline]
        #[must_use]
        pub const unsafe fn as_mut(&mut self) -> &mut T {
            // SAFETY: RlManaged must be unique and cannot dangle.
            // Taking `self` by mutable reference guarantees aliasing rules are followed.
            // Caller must ensure `self` is safe to dereference.
            unsafe { self.0.as_mut() }
        }

        /// Access the pointer of this allocation.
        #[inline]
        #[must_use]
        pub const fn into_inner(self) -> NonNull<T> {
            self.0
        }

        /// Reallocate `self` to have `size` bytes of memory aligned to `U` using [`ffi::MemRealloc`].
        ///
        /// Any elements that were initialized prior to calling will still be initialized after reallocating.
        /// Any elements that were not previously in the allocation are uninitialized.
        ///
        /// Returns the original memory block if [`ffi::MemRealloc`] returned null.
        ///
        /// **WARNING:** This method does not drop the contents of `self`.
        #[inline]
        pub fn mem_realloc<U>(self, size: NonZeroU32) -> Result<RlManaged<MaybeUninit<U>>, Self> {
            // SAFETY: `self` is non-null and is Raylib-allocated, and `size` is not zero.
            let new_ptr = unsafe { ffi::MemRealloc(self.0.as_ptr().cast(), size.get()) }.cast();
            NonNull::new(new_ptr).map(RlManaged).ok_or(self)
        }

        /// Free `self` using [`ffi::MemFree`].
        ///
        /// **WARNING:** This method does not drop the contents of `self`.
        #[inline]
        pub fn mem_free(self) {
            // SAFETY: `self` is non-null, not dangling, and Raylib-allocated.
            unsafe {
                ffi::MemFree(self.0.as_ptr().cast());
            }
        }
    }

    impl<T> RlManaged<[T]> {
        /// Create a Raylib-managed slice from a thin pointer and a length.
        ///
        /// The `len` argument is the number of **elements**, not the number of bytes.
        ///
        /// This function is safe, but dereferencing the return value is unsafe.
        /// See the documentation of [`std::slice::from_raw_parts`] for slice safety requirements.
        #[inline]
        #[allow(
            clippy::needless_pass_by_value,
            reason = "passing by reference would allow `data` to be duplicated because `data.0` is Copy"
        )]
        pub(crate) const fn slice_from_raw_parts(data: RlManaged<T>, len: usize) -> Self {
            Self(NonNull::slice_from_raw_parts(data.0, len))
        }
    }
}
pub use rl_managed::*;

/// A wrapper acting as an owned buffer for Raylib-allocated memory.
/// Automatically releases the memory with [`ffi::MemFree()`] when dropped.
///
/// # Example
/// ```
/// use raylib::prelude::*;
/// let buf: DataBuf<[u8]> = compress_data(b"11111").unwrap();
/// // Use this how you used to use the return of `compress_data()`.
/// // It will live until `buf` goes out of scope or gets dropped.
/// let data: &[u8] = buf.as_ref();
/// let expected: &[u8] = &[1, 5, 0, 250, 255, 49, 49, 49, 49, 49];
/// assert_eq!(data, expected);
/// ```
///
/// # Safety
///
/// - `buf` must not be dangling.
/// - `buf` must be safe to dereference.
/// - `buf` must be a **unique, owned** pointer (not to static or local memory, and the memory must not be
///   accessible through any pointers/references not derived from the returned [`DataBuf`]).
/// - `buf` must point to [valid](https://doc.rust-lang.org/std/ptr/index.html#safety), intialized data.
/// - `buf` must be [convertible to a reference](std::ptr#pointer-to-reference-conversion).
/// - `buf` must have been created with `RL_MALLOC`/[`ffi::MemAlloc`] or `RL_REALLOC`/[`ffi::MemRealloc`].
///
/// This structure is only intended for use with pointers given by Raylib with the expectation that you
/// would manually deallocate them with [`ffi::MemFree`]. DO NOT use this structure to hold arbitrary
/// or un-owned pointers.
///
/// If the pointer is expected to be conditionally deallocated by Raylib,
/// (i.e. conditionally passing the buffer to a Raylib function that will certainly deallocatate it)
/// use [`DataBuf::leak`] to prevent [`DataBuf::drop`] from causing a double-free.
#[derive(Debug)]
#[repr(transparent)]
#[must_use]
pub struct DataBuf<T: ?Sized> {
    buf: RlManaged<T>,
    /// Tell the compiler that this instance logically owns a `T`.
    _marker: PhantomData<T>,
}

impl<T: ?Sized> Drop for DataBuf<T> {
    #[inline]
    fn drop(&mut self) {
        let mut ptr = MaybeUninit::uninit();
        // SAFETY: Both `self.buf` and `ptr` are non-null and valid for 1 element.
        // Taking `self` by mutable reference ensures aliasing rules are upheld
        // outside of the method; and `self` is not used again after this line.
        // Because `drop` is the end of `self`'s lifetime, `buf` is guaranteed not
        // to be accessed again after the function returns.
        unsafe {
            std::ptr::copy_nonoverlapping(std::ptr::from_ref(&self.buf), ptr.as_mut_ptr(), 1);
        }
        // SAFETY: Just written to with a valid value.
        let data = unsafe { ptr.assume_init() }.into_inner();
        // SAFETY: DataBuf `buf` is guaranteed to be unique, owned, non-null, valid, and not dangling.
        // DataBuf and RlManaged do not implement Clone, and `drop` is called *at most once*, so `data`
        // is guaranteed not to have been dropped for `T` yet so long as DataBuf's safety contract has
        // been upheld.
        unsafe {
            data.drop_in_place();
        }
        // SAFETY: `data` taken from `RlManaged` is still managed by Raylib after contents is dropped.
        // Any copies of `data` go out of scope, preventing double-free.
        unsafe { RlManaged::new(data) }.mem_free();
    }
}

impl<T: ?Sized> Deref for DataBuf<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> DerefMut for DataBuf<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: ?Sized> AsRef<T> for DataBuf<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ?Sized> AsMut<T> for DataBuf<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> DataBuf<MaybeUninit<T>> {
    /// Initialize the buffer with valid memory.
    #[inline]
    pub const fn write(mut self, val: T) -> DataBuf<T> {
        // SAFETY: DataBuf guarantees `buf` is safe to dereference, and
        // ownership of `self` ensures aliasing rules are followed.
        let buf = unsafe { self.buf.as_mut() };
        MaybeUninit::write(buf, val);
        // SAFETY: We just initialized this value.
        unsafe { self.assume_init() }
    }

    /// Mark that the data pointed to by `self` is initialized.
    ///
    /// # Safety
    ///
    /// The data pointed to by `self` must actually be initialized.
    #[inline]
    pub const unsafe fn assume_init(self) -> DataBuf<T> {
        // SAFETY: `DataBuf<MaybeUninit<T>>` and `DataBuf<T>` have the same layout
        unsafe { std::mem::transmute::<DataBuf<MaybeUninit<T>>, DataBuf<T>>(self) }
    }
}

impl<T> DataBuf<[MaybeUninit<T>]> {
    /// Mark that the data pointed to by `self` is initialized.
    ///
    /// # Safety
    ///
    /// The data pointed to by `self` must actually be initialized.
    #[inline]
    pub const unsafe fn assume_init(self) -> DataBuf<[T]> {
        // SAFETY: `DataBuf<[MaybeUninit<T>]>` and `DataBuf<[T]>` have the same layout
        unsafe { std::mem::transmute::<DataBuf<[MaybeUninit<T>]>, DataBuf<[T]>>(self) }
    }
}

impl<T: ?Sized> DataBuf<T> {
    /// Returns a shared reference to the value.
    #[inline]
    #[must_use]
    pub const fn as_ref(&self) -> &T {
        // SAFETY: DataBuf guarantees `buf` is safe to dereference, and taking
        // `self` by reference ensures aliasing rules are followed.
        unsafe { self.buf.as_ref() }
    }

    /// Returns a unique reference to the value.
    #[inline]
    #[must_use]
    pub const fn as_mut(&mut self) -> &mut T {
        // SAFETY: DataBuf guarantees `buf` is safe to dereference, and taking
        // `self` by mutable reference ensures aliasing rules are followed.
        unsafe { self.buf.as_mut() }
    }

    /// Wrap an already allocated, non-null, Raylib-managed pointer in a [`DataBuf`].
    #[inline]
    pub(crate) const fn from_rlmanaged(buf: RlManaged<T>) -> Self {
        Self {
            buf,
            _marker: PhantomData,
        }
    }

    /// Wrap an already allocated, non-null pointer in a [`DataBuf`].
    ///
    /// # Safety
    ///
    /// See the [`DataBuf`] safety requirements.
    #[inline]
    pub(crate) const unsafe fn from_nonnull(data: NonNull<T>) -> Self {
        // SAFETY: Caller must ensure `data` is Raylib-managed.
        let buf = unsafe { RlManaged::new(data) };
        Self::from_rlmanaged(buf)
    }

    /// Wrap an already allocated pointer in a [`DataBuf`].
    /// Returns [`None`] if `buf` is null.
    ///
    /// # Safety
    ///
    /// See the [`DataBuf`] safety requirements.
    #[inline]
    pub(crate) const unsafe fn from_raw(ptr: *mut T) -> Option<Self> {
        if let Some(buf) = NonNull::new(ptr) {
            // SAFETY: Caller must uphold safety contract.
            Some(unsafe { Self::from_nonnull(buf) })
        } else {
            None
        }
    }

    /// Extract the pointer without freeing it, for the purpose of transferring ownership.
    ///
    /// **WARNING:** The returned pointer must be unloaded manually to avoid a memory leak.
    #[inline]
    pub const fn into_inner(self) -> RlManaged<T> {
        let mut buf = MaybeUninit::uninit();
        // SAFETY: Both `self.buf` and `ptr` are non-null and valid for 1 element.
        unsafe {
            std::ptr::copy_nonoverlapping(std::ptr::from_ref(&self.buf), buf.as_mut_ptr(), 1);
        }
        // SAFETY: Just written to with a valid value
        let buf = unsafe { buf.assume_init() };
        std::mem::forget(self); // Prevent `self` from causing double-free
        buf
    }
}

impl<T> DataBuf<T> {
    /// Allocate new memory managed by Raylib.
    #[inline]
    pub fn alloc() -> Result<DataBuf<MaybeUninit<T>>, AllocationError> {
        let bytes = allocation_array_size::<T>(1)?;
        let buf = mem_alloc::<T>(bytes).ok_or(AllocationError::NullAlloc)?;
        Ok(DataBuf::from_rlmanaged(buf))
    }

    /// Allocate new memory managed by Raylib and move `val` into it.
    #[inline]
    pub fn alloc_from(val: T) -> Result<Self, (AllocationError, T)> {
        match Self::alloc() {
            Ok(buf) => Ok(buf.write(val)),
            Err(e) => Err((e, val)),
        }
    }

    /// Allocate new memory managed by Raylib and clone `val` into it.
    #[inline]
    pub fn alloc_from_clone(src: &T) -> Result<Self, AllocationError>
    where
        T: Clone,
    {
        Ok(Self::alloc()?.write(src.clone()))
    }

    /// Allocate new memory managed by Raylib and copy `val` into it.
    #[inline]
    pub fn alloc_from_copy(src: &T) -> Result<Self, AllocationError>
    where
        T: Copy,
    {
        Ok(Self::alloc()?.write(*src))
    }
}

impl<T> DataBuf<[T]> {
    /// Wrap an already allocated pointer in a [`DataBuf`].
    ///
    /// # Safety
    ///
    /// **In addition** to the [`DataBuf`] and [`from_raw_parts_mut`](std::slice::from_raw_parts_mut)
    /// safety requirements, this function also requires:
    /// - `buf` must point to an array of as many valid, initialized elements as defined by `count`.
    #[inline]
    pub(crate) const unsafe fn slice_from_nonnull(buf: NonNull<T>, len: NonZeroUsize) -> Self {
        // SAFETY: Caller must uphold `from_raw_parts_mut` safety contract
        let slice = unsafe { std::slice::from_raw_parts_mut(buf.as_ptr(), len.get()) };
        // SAFETY: A mutable reference cannot be null.
        let buf = unsafe { NonNull::new_unchecked(slice) };
        // SAFETY: Calller must uphold `DataBuf` safety contract
        unsafe { Self::from_nonnull(buf) }
    }

    /// Wrap an already allocated pointer in a [`DataBuf`].
    /// Returns [`None`] if `buf` is null.
    ///
    /// Takes `count` as a [`MaybeUninit<i32>`] for convenience, as most Raylib functions returning an array
    /// buffer provide the length of the buffer as an [`i32`] out param.
    ///
    /// # Safety
    ///
    /// **In addition** to the [`DataBuf`] and [`from_raw_parts_mut`](std::slice::from_raw_parts_mut) safety requirements,
    /// this function also requires:
    /// - `count` must be initialized if `buf` is non-null.
    /// - `buf` must point to an array of as many valid, initialized elements as defined by `count`.
    ///
    /// # Panics
    ///
    /// This method may panic if `count` is less than 1 or greater than [`usize::MAX`] while `buf` is non-null.
    #[inline]
    pub(crate) const unsafe fn slice_from_raw(
        ptr: *mut T,
        count: MaybeUninit<i32>,
    ) -> Option<Self> {
        if let Some(buf) = NonNull::new(ptr) {
            // SAFETY: Caller must ensure `count` is initialized if `buf` is non-null.
            let count = unsafe { count.assume_init() };
            assert!(count >= 1, "`count` should be positive");
            // confirm `as usize` will not overflow
            #[cfg(target_pointer_width = "16")]
            {
                assert!(
                    count <= usize::MAX as i32,
                    "`count` should fit within usize"
                );
            }
            #[allow(clippy::cast_sign_loss, reason = "intentional")]
            // SAFETY: Just checked that count is non-zero and positive.
            let len = unsafe { NonZeroUsize::new_unchecked(count as usize) };
            // SAFETY: Caller must uphold `DataBuf` and `slice` safety contracts
            Some(unsafe { Self::slice_from_nonnull(buf, len) })
        } else {
            None
        }
    }

    /// Allocate new memory managed by Raylib.
    ///
    /// # Example
    /// ```
    /// # use raylib::prelude::DataBuf;
    /// let mut data_buf = DataBuf::<[i32]>::alloc(5).unwrap();
    /// data_buf[0].write(4);
    /// data_buf[1].write(8);
    /// data_buf[2].write(-23);
    /// data_buf[3].write(9);
    /// data_buf[4].write(0);
    /// // SAFETY: Just initialized all elements
    /// let data_buf = unsafe { data_buf.assume_init() };
    /// assert_eq!(data_buf.as_ref(), &[4, 8, -23, 9, 0]);
    /// ```
    /// (See also: [`DataBuf::alloc_from_copy`])
    #[inline]
    pub fn alloc(count: usize) -> Result<DataBuf<[MaybeUninit<T>]>, AllocationError> {
        let bytes = allocation_array_size::<T>(count)?;
        let buf = mem_alloc::<T>(bytes).ok_or(AllocationError::NullAlloc)?;
        Ok(DataBuf {
            buf: RlManaged::slice_from_raw_parts(buf, count),
            _marker: PhantomData,
        })
    }

    /// Allocate memory managed by Raylib and initialize by copying.
    ///
    /// # Panics
    ///
    /// This method may panic in debug if the pointer returned by [`ffi::MemAlloc`] is unaligned.
    ///
    /// # Example
    /// ```
    /// # use raylib::prelude::DataBuf;
    /// let src = [4, 8, -23, 9, 0];
    /// let mut data_buf = DataBuf::<[i32]>::alloc_from_copy(&src).unwrap();
    /// assert_eq!(data_buf.as_ref(), &src);
    /// ```
    pub fn alloc_from_clone(src: &[T]) -> Result<Self, AllocationError>
    where
        T: Copy,
    {
        let mut buf = Self::alloc(src.len())?;
        // SAFETY: `&[T]` and `&[MaybeUninit<T>]` have the same layout. Reference to is non-null.
        let uninit_src = unsafe { &*(std::ptr::from_ref::<[T]>(src) as *const [MaybeUninit<T>]) };
        buf.copy_from_slice(uninit_src);
        // SAFETY: Valid elements have just been copied into `self` so it is initialized.
        Ok(unsafe { buf.assume_init() })
    }

    /// Allocate memory managed by Raylib and initialize by copying.
    ///
    /// # Panics
    ///
    /// This method may panic in debug if the pointer returned by [`ffi::MemAlloc`] is unaligned.
    ///
    /// # Example
    /// ```
    /// # use raylib::prelude::DataBuf;
    /// let src = [4, 8, -23, 9, 0];
    /// let mut data_buf = DataBuf::<[i32]>::alloc_from_copy(&src).unwrap();
    /// assert_eq!(data_buf.as_ref(), &src);
    /// ```
    pub fn alloc_from_copy(src: &[T]) -> Result<Self, AllocationError>
    where
        T: Copy,
    {
        let mut buf = Self::alloc(src.len())?;
        // SAFETY: `&[T]` and `&[MaybeUninit<T>]` have the same layout. Reference to is non-null.
        let uninit_src = unsafe { &*(std::ptr::from_ref::<[T]>(src) as *const [MaybeUninit<T>]) };
        buf.copy_from_slice(uninit_src);
        // SAFETY: Valid elements have just been copied into `self` so it is initialized.
        Ok(unsafe { buf.assume_init() })
    }

    /// Reallocate memory already managed by Raylib.
    ///
    /// # Panics
    ///
    /// This method may panic in debug if the pointer returned by [`ffi::MemAlloc`] is unaligned.
    pub fn realloc(
        self,
        new_count: usize,
    ) -> Result<DataBuf<[MaybeUninit<T>]>, (AllocationError, Self)> {
        match allocation_array_size::<T>(new_count) {
            Err(e) => Err((e, self)),
            Ok(bytes) => {
                let new_buf = self.into_inner().mem_realloc(bytes).map_err(|old_buf| {
                    (AllocationError::NullAlloc, Self::from_rlmanaged(old_buf))
                })?;
                let new_buf = RlManaged::slice_from_raw_parts(new_buf, new_count);
                Ok(DataBuf::from_rlmanaged(new_buf))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_value() {
        struct DropTest<F: FnMut()>(F);
        impl<F: FnMut()> Drop for DropTest<F> {
            fn drop(&mut self) {
                (self.0)();
            }
        }
        impl<F: FnMut()> std::fmt::Debug for DropTest<F> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple("DropTest").finish()
            }
        }

        let mut times_dropped = 0;
        let buf = DataBuf::alloc_from(DropTest(|| times_dropped += 1)).unwrap();
        drop(buf);
        assert_eq!(
            times_dropped, 1,
            "DataBuf should drop contents exactly once"
        );
    }

    #[test]
    fn test_from_raw() {
        type ExpectTy = [i32; 5];
        const EXPECT: [i32; 5] = [64, 264, -57, 653, -153];
        let bytes @ 1.. = (std::mem::size_of::<i32>() * EXPECT.len())
            .try_into()
            .unwrap()
        else {
            unreachable!()
        };
        // SAFETY: `bytes` is non-zero
        let ptr = unsafe { ffi::MemAlloc(bytes) }.cast::<ExpectTy>();
        assert!(!ptr.is_null(), "should be able to allocate");
        // SAFETY: `ptr` is not null and `MemAlloc` returns owned memory.
        unsafe {
            ptr.write(EXPECT);
        };
        // SAFETY: `ptr` is unique, non-dangling, valid, and allocated by Raylib
        let buf = unsafe { DataBuf::from_raw(ptr) }.expect("ptr should be convertible to DataBuf");
        assert_eq!(&*buf, &EXPECT);
    }

    #[test]
    fn test_slice_from_raw() {
        type ExpectTy = [i32; 5];
        const EXPECT: ExpectTy = [6, -453, 364, 45632, -1233];
        let bytes @ 1.. = (std::mem::size_of::<i32>() * EXPECT.len())
            .try_into()
            .unwrap()
        else {
            unreachable!()
        };
        // SAFETY: `bytes` is non-zero
        let ptr = unsafe { ffi::MemAlloc(bytes) }.cast::<i32>();
        assert!(!ptr.is_null(), "should be able to allocate");
        // SAFETY: `ptr` is not null and `MemAlloc` returns owned memory.
        unsafe {
            ptr.cast::<ExpectTy>().write(EXPECT);
        };
        // SAFETY: `ptr` is unique, non-dangling, valid, and allocated by Raylib
        let buf = unsafe {
            DataBuf::slice_from_raw(ptr, MaybeUninit::new(EXPECT.len().try_into().unwrap()))
        }
        .expect("ptr should be convertible to DataBuf");
        assert_eq!(&*buf, &EXPECT);
    }
}
