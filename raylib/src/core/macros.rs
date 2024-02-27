macro_rules! make_thin_wrapper {
    ($name:ident, $t:ty, $dropfunc:expr) => {
        make_thin_wrapper!($name, $t, $dropfunc, true);
    };
    ($name:ident, $t:ty, $dropfunc:expr, false) => {
        #[repr(transparent)]
        #[derive(Debug)]
        pub struct $name(pub(crate) $t);

        impl_wrapper!($name, $t, $dropfunc, 0);
        gen_from_raw_wrapper!($name, $t, $dropfunc, 0);
    };
    ($name:ident, $t:ty, $dropfunc:expr, true) => {
        #[repr(transparent)]
        #[derive(Debug)]
        pub struct $name(pub(crate) $t);

        impl_wrapper!($name, $t, $dropfunc, 0);
        deref_impl_wrapper!($name, $t, $dropfunc, 0);
        gen_from_raw_wrapper!($name, $t, $dropfunc, 0);
    };
}

macro_rules! make_thin_wrapper_lifetime {
    ($name:ident, $t1:ty, $t2:ty, $dropfunc:expr) => {
        make_thin_wrapper_lifetime!($name, $t1, $t2, $dropfunc, true);
    };
    ($name:ident, $t1:ty, $t2:ty,$dropfunc:expr, false) => {
        #[derive(Debug)]
        pub struct $name<'a>(pub(crate) $t1, &'a $t2);

        impl_wrapper!($name, $t1, $dropfunc, 0);
    };
    ($name:ident, $t1:ty, $t2:ty, $dropfunc:expr, true) => {
        #[derive(Debug)]
        pub struct $name<'a>(pub(crate) $t1, &'a $t2);

        impl_wrapper!($name<'a>, $t1, $dropfunc, 0);
        deref_impl_wrapper!($name<'a>, $t1, $dropfunc, 0);
    };
}

macro_rules! impl_wrapper {
    ($name:ident$(<$lifetime:tt>)?, $t:ty, $dropfunc:expr, $rawfield:tt) => {
        impl$(<$lifetime>)? $name$(<$lifetime>)? {
            /// Take the raw ffi type. Must manually free memory by calling the proper unload function
            pub unsafe fn unwrap(self) -> $t {
                let inner = self.$rawfield;
                std::mem::forget(self);
                inner
            }
        }

        impl$(<$lifetime>)? Drop for $name$(<$lifetime>)? {
            #[allow(unused_unsafe)]
            fn drop(&mut self) {
                unsafe {
                    ($dropfunc)(self.$rawfield);
                }
            }
        }


    };
}

macro_rules! gen_from_raw_wrapper {
    ($name:ident$(<$lifetime:tt>)?, $t:ty, $dropfunc:expr, $rawfield:tt) => {
        impl$(<$lifetime>)? $name$(<$lifetime>)? {
            /// returns the unwrapped raylib-sys object
            pub fn to_raw(self) -> $t {
                let raw = self.$rawfield;
                std::mem::forget(self);
                raw
            }

            /// converts raylib-sys object to a "safe"
            /// version. Make sure to call this function
            /// from the thread the resource was created.
            pub unsafe fn from_raw(raw: $t) -> Self {
                Self(raw)
            }
        }
    };
}

macro_rules! deref_impl_wrapper {
    ($name:ident$(<$lifetime:tt>)?, $t:ty, $dropfunc:expr, $rawfield:tt) => {
        impl$(<$lifetime>)? std::convert::AsRef<$t> for $name$(<$lifetime>)? {
            fn as_ref(&self) -> &$t {
                &self.$rawfield
            }
        }

        impl$(<$lifetime>)? std::convert::AsMut<$t> for $name$(<$lifetime>)? {
            fn as_mut(&mut self) -> &mut $t {
                &mut self.$rawfield
            }
        }

        impl$(<$lifetime>)? std::ops::Deref for $name$(<$lifetime>)? {
            type Target = $t;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$rawfield
            }
        }

        impl$(<$lifetime>)? std::ops::DerefMut for $name$(<$lifetime>)? {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$rawfield
            }
        }
    };
}
macro_rules! make_rslice {
    ($name:ident, $t:ty, $dropfunc:expr) => {
        #[repr(transparent)]
        #[derive(Debug)]
        pub struct $name(pub(crate) std::mem::ManuallyDrop<std::boxed::Box<[$t]>>);

        impl_rslice!($name, std::boxed::Box<[$t]>, $dropfunc, 0);
    };
}

macro_rules! impl_rslice {
    ($name:ident, $t:ty, $dropfunc:expr, $rawfield:tt) => {
        impl Drop for $name {
            #[allow(unused_unsafe)]
            fn drop(&mut self) {
                unsafe {
                    let inner = std::mem::ManuallyDrop::take(&mut self.0);
                    ($dropfunc)(std::boxed::Box::leak(inner).as_mut_ptr() as *mut _);
                }
            }
        }

        impl std::convert::AsRef<$t> for $name {
            fn as_ref(&self) -> &$t {
                &self.$rawfield
            }
        }

        impl std::convert::AsMut<$t> for $name {
            fn as_mut(&mut self) -> &mut $t {
                &mut self.$rawfield
            }
        }

        impl std::ops::Deref for $name {
            type Target = $t;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$rawfield
            }
        }

        impl std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$rawfield
            }
        }
    };
}
