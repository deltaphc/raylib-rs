use crate::audio::{AudioSample, AudioStream};
use crate::error::UpdateAudioStreamError;
use crate::ffi;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::slice::from_raw_parts_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

struct AudioCallbackWrapper {
    channels: u16,
    bytes_per_sample: usize,
    callback_fn: Box<dyn FnMut(&mut [u8]) + Send>,
}

static AUDIO_STREAM_CALLBACK_SLOT: AtomicPtr<AudioCallbackWrapper> = AtomicPtr::new(null_mut());

/// Audio thread callback to request new data
pub fn set_audio_stream_callback<T, F>(
    stream: &AudioStream,
    cb: F,
) -> Result<(), UpdateAudioStreamError>
where
    T: AudioSample,
    F: FnMut(&mut [T]) + Send + 'static,
{
    if !AUDIO_STREAM_CALLBACK_SLOT.load(Ordering::Acquire).is_null() {
        return Err(UpdateAudioStreamError::CallbackSlotBusy);
    }
    let expected_sample_size =
        usize::try_from(stream.sample_size()).expect("sampleSize should be 8, 16, or 32");
    let provided_sample_size_bits = size_of::<T>() * u8::BITS as usize;
    if provided_sample_size_bits != expected_sample_size {
        return Err(UpdateAudioStreamError::SampleSizeMismatch {
            expected: expected_sample_size,
            provided: provided_sample_size_bits,
        });
    }
    let channels = u16::try_from(stream.channels()).expect("channels should fit in u16");
    let bytes_per_sample = size_of::<T>();
    let callback_fn: Box<dyn FnMut(&mut [u8]) + Send> = Box::new({
        let mut closure_captured = cb;
        move |bytes: &mut [u8]| {
            let byte_len = bytes.len() / bytes_per_sample;
            let buffer_ptr = bytes.as_mut_ptr() as *mut T;
            let buffer_slice = unsafe { from_raw_parts_mut(buffer_ptr, byte_len) };
            closure_captured(buffer_slice);
        }
    });
    let callback_wrapper = Box::new(AudioCallbackWrapper {
        channels,
        bytes_per_sample,
        callback_fn,
    });

    let raw_ptr_for_callback = Box::into_raw(callback_wrapper);
    AUDIO_STREAM_CALLBACK_SLOT.store(raw_ptr_for_callback, Ordering::Release);
    unsafe { ffi::SetAudioStreamCallback(stream.0, Some(custom_audio_stream_callback)) }
    Ok(())
}

pub fn unset_audio_stream_callback(stream: &AudioStream) {
    unsafe { ffi::SetAudioStreamCallback(stream.0, None) }
    let raw_ptr_for_callback = AUDIO_STREAM_CALLBACK_SLOT.swap(null_mut(), Ordering::AcqRel);
    if !raw_ptr_for_callback.is_null() {
        unsafe {
            drop(Box::from_raw(raw_ptr_for_callback));
        }
    }
}

extern "C" fn custom_audio_stream_callback(data: *mut c_void, frame_count: u32) {
    let raw_ptr_to_callback = AUDIO_STREAM_CALLBACK_SLOT.load(Ordering::Acquire);
    if raw_ptr_to_callback.is_null() {
        // this should never happen, i imagine there is a more elegant way to make sure...
        return;
    }
    let callback_wrapper = unsafe { &mut *raw_ptr_to_callback };
    let channels = callback_wrapper.channels as usize;
    let byte_len = frame_count as usize * channels * callback_wrapper.bytes_per_sample;
    let buffer_slice = unsafe { from_raw_parts_mut(data as *mut u8, byte_len) };
    (callback_wrapper.callback_fn)(buffer_slice);
}
