use std::{marker::PhantomData, ops::Index, ptr::NonNull, slice};

use crate::ffi;

#[repr(transparent)]
pub(crate) struct ContigousBuffer<'a, T>(NonNull<T>, PhantomData<&'a T>);

impl<'a, T> ContigousBuffer<'a, T> {
    pub fn new(buffer: *mut T) -> Option<Self> {
        NonNull::new(buffer).and_then(|p| Some(ContigousBuffer(p, PhantomData)))
    }

    pub unsafe fn get_slice(&'a self, len: usize) -> &'a [T] {
        slice::from_raw_parts(self.0.as_ptr(), len)
    }

    pub unsafe fn get_mut_slice(&'a mut self, len: usize) -> &'a mut [T] {
        slice::from_raw_parts_mut(self.0.as_ptr(), len)
    }

    pub fn get_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
}

pub struct RaylibBuffer<'a, T>(&'a mut [T], ContigousBuffer<'a, T>);

impl<'a, T> RaylibBuffer<'a, T> {
    pub unsafe fn new(data: *mut T, len: usize) -> Option<Self> {
        Some(RaylibBuffer(
            slice::from_raw_parts_mut(data, len),
            ContigousBuffer::new(data)?,
        ))
    }

    pub fn get_slice(&'a self) -> &'a [T] {
        self.0
    }

    pub fn get_mut_slice(&'a mut self) -> &'a mut [T] {
        self.0
    }

    pub fn get_ptr(&self) -> *mut T {
        self.1.get_ptr()
    }
}

impl<'a, T> Index<usize> for RaylibBuffer<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.get_ptr().add(index) }
    }
}

impl<'a, T> Drop for RaylibBuffer<'a, T> {
    fn drop(&mut self) {
        unsafe { ffi::MemFree(self.1.get_ptr() as _) }
    }
}
