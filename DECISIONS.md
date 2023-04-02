Document where I put all the little design decisions that go into this library.

1. Allocations
   A few functiosn in raylib return a buffer that should be deallocated by the user. Previously, we copied this data into a Vector and then freed with libc::free.

If the user had a custom alocator or some other strange linking strategy, this would free an invalid pointer.

Now we cast buffers to `ManuallyDrop<Box<[T]>>`
This allows us to created a box slice and have all the crazy iterator shenanigans users love, without invoking a copy allocation.

We use `Box::leak` and `ManuallyDrop::take` to get the slice and then cast that to a `* void` for raylibs various `UnloadX` functions. If an `UnloadX` function doesn't exist, we use the `MemFree` function to return memory using the same allocator as raylib.


2. impl Into

Where posible, have the library call `.into` on function params instead of relying on the user to make the call.
