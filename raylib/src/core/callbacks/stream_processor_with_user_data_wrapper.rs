use paste::paste;
use raylib_sys::{AttachAudioStreamProcessor, AudioStream};
use seq_macro::seq;
use std::sync::{LazyLock, Mutex};

// region: -- AudioCallbackWithUserData --

/// This is the callback we wish to get from raylib:
/// It contains `user_data` in order to plug in our
/// context (e.g. our closure).
type RawAudioCallbackWithUserData = extern "C" fn(
    user_data: *mut ::std::os::raw::c_void,
    data_ptr: *mut ::std::os::raw::c_void,
    frames: u32,
) -> ();

/// This is a tuple of `user_data` which represents
/// our context (see RawAudioCallbackWithUserData)
/// and the callback we wish to pass to our raylib
/// abstraction layer (wrapping the real raylib
/// callback to plug in our context).
pub struct AudioCallbackWithUserData {
    user_data: *mut ::std::os::raw::c_void,
    callback: Option<RawAudioCallbackWithUserData>,
}

unsafe impl Send for AudioCallbackWithUserData {} //??

impl AudioCallbackWithUserData {
    pub fn new(
        user_data: *mut ::std::os::raw::c_void,
        raw_callback: RawAudioCallbackWithUserData,
    ) -> Self {
        AudioCallbackWithUserData {
            user_data: user_data,
            callback: Some(raw_callback),
        }
    }
}

impl Default for AudioCallbackWithUserData {
    fn default() -> Self {
        AudioCallbackWithUserData {
            user_data: std::ptr::null_mut(),
            callback: None,
        }
    }
}

// endregion: -- AudioCallbackWithUserData --

// region: -- raw callbacks and linkage
// raw callback and linkage to AudioCallbackWithUserData
// we only support a limited amount of callbacks - since
// we need a dedicated callback function for each
// callback or closure we plug in. This is caused by the
// absence of a `user_data` context in the callbacks
// supported by raylib.

macro_rules! generate_functions {
  ( $( $n:literal ),* ) => {
    paste! {
        $(
            /// For each supported callback the data for our context.
            /// (here we have N "slots" with context data)
            static [< CLOSURE_ $n >]:  LazyLock<Mutex<AudioCallbackWithUserData>> = LazyLock::new(|| Mutex::new(AudioCallbackWithUserData::default()));
        )*

          /// Function to set our context
          /// and returns the slot used to store the context.
          fn set_context(audio_callback: AudioCallbackWithUserData) -> usize {
              $(
                  {
                      let mut guard = [< CLOSURE_ $n >].lock().unwrap();
                      if (*guard).callback == None {
                        *guard = audio_callback;
                        return $n;
                      }
                  }
              )*
              panic!("index out of bounds");
          }

          /// Function to clear our context given the slot of the context.
          fn clear_context(index: usize) {
              $(
                  if index == $n {
                      let mut guard = [< CLOSURE_ $n >].lock().unwrap();
                      if (*guard).callback == None {
                          panic!(
                              "No callbacks registered under this number ({}).",
                              index
                          );
                      }
                      *guard = AudioCallbackWithUserData::default();
                      return;
                  }
              )*
              panic!("clear_context: index {} out of bounds", index);
          }

          $(
            /// The real callback passed to raylib.
            /// Each callback has a fixed association with
            /// a given context "slot".
            #[no_mangle]
            pub extern "C" fn [< callback_ $n >](data_ptr: *mut ::std::os::raw::c_void, frames: u32) -> () {
              let guard = [< CLOSURE_ $n >].lock().unwrap();
              let audio_callback = &(*guard);
              if let Some(callback) = audio_callback.callback {
                (callback)(audio_callback.user_data, data_ptr, frames);
              } else {
                  panic!("unexpected: no callback $n set")
              }
            }
          )*

          /// Function to get the callback for a given context
          /// given the slot of the context.
          fn get_callback(index: usize) -> extern "C" fn(data_ptr: *mut ::std::os::raw::c_void, frames: u32) {
            $(
                if index == $n {
                    return [< callback_ $n >];
                }
            )*
            panic!("get_callback: index out of bounds");
          }
        }
  }
}

// here, you can control how many callbacks are supported
seq!(I in 1..30 {
    generate_functions!( 0#(,I)* );
});

// endregion: -- raw callbacks and linkage

/// Here, we c
pub fn attach_audio_stream_processor_with_user_data(
    stream: AudioStream,
    callback: AudioCallbackWithUserData,
) -> usize {
    let idx = set_context(callback);
    unsafe {
        AttachAudioStreamProcessor(stream, Some(get_callback(idx)));
    }
    idx
}

pub fn detach_audio_stream_processor_with_user_data(index: usize) {
    clear_context(index);
}
