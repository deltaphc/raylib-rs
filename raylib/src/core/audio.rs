//! Contains code related to audio. [`RaylibAudio`] plays sounds and music.

use crate::ffi;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::path::Path;

make_thin_wrapper_lifetime!(Wave, ffi::Wave, RaylibAudio, ffi::UnloadWave);

make_thin_wrapper_lifetime!(Sound, ffi::Sound, RaylibAudio, (ffi::UnloadSound), true);
make_thin_wrapper_lifetime!(Music, ffi::Music, RaylibAudio, ffi::UnloadMusicStream);
make_thin_wrapper_lifetime!(
    AudioStream,
    ffi::AudioStream,
    RaylibAudio,
    ffi::UnloadAudioStream
);

make_rslice!(WaveSamples, f32, ffi::UnloadWaveSamples);

/// A marker trait specifying an audio sample (`u8`, `i16`, or `f32`).
pub trait AudioSample {}
impl AudioSample for u8 {}
impl AudioSample for i16 {}
impl AudioSample for f32 {}

pub struct RaylibAudioInitError;

impl std::fmt::Debug for RaylibAudioInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RaylibAudio cannot be instantiated more then once at a time.")
    }
}
impl std::fmt::Display for RaylibAudioInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RaylibAudio cannot be instantiated more then once at a time.")
    }
}

impl std::error::Error for RaylibAudioInitError {}

/// This token is used to indicate audio is initialized. It's also used to create [`Wave`], [`Sound`], [`Music`], [`AudioStream`], and [`SoundAlias`].
/// All of those have a lifetime that is bound to RaylibAudio. The compiler will disallow you from using them without ensuring that the [`RaylibAudio`] is present while doing so.
#[derive(Debug, Clone)]
pub struct RaylibAudio(PhantomData<()>);

impl RaylibAudio {
    /// Initializes audio device and context.
    #[inline]
    pub fn init_audio_device() -> Result<RaylibAudio, RaylibAudioInitError> {
        unsafe {
            let t = ffi::IsAudioDeviceReady();
            if t {
                return Err(RaylibAudioInitError);
            }
            ffi::InitAudioDevice();
        }
        Ok(RaylibAudio(PhantomData))
    }

    /// Checks if audio device is ready.
    #[inline]
    pub fn is_audio_device_ready(&self) -> bool {
        unsafe { ffi::IsAudioDeviceReady() }
    }

    /// Get master volume (listener)
    pub fn get_master_volume(&mut self) -> f32 {
        unsafe { ffi::GetMasterVolume() }
    }

    /// Sets master volume (listener).
    #[inline]
    pub fn set_master_volume(&self, volume: f32) {
        unsafe {
            ffi::SetMasterVolume(volume);
        }
    }

    /// Loads a new sound from file.
    pub fn new_sound<'aud>(&'aud self, filename: &str) -> Result<Sound<'aud>, String> {
        let c_filename = CString::new(filename).unwrap();
        let s = unsafe { ffi::LoadSound(c_filename.as_ptr()) };
        if s.stream.buffer.is_null() {
            return Err(format!("failed to load sound {}", filename));
        }

        Ok(Sound(s, self))
    }

    /// Loads sound from wave data.
    pub fn new_sound_from_wave<'aud>(&'aud self, wave: &Wave) -> Result<Sound<'aud>, String> {
        let s = unsafe { ffi::LoadSoundFromWave(wave.0) };
        if s.stream.buffer.is_null() {
            return Err(format!("failed to load sound from wave"));
        }
        Ok(Sound(s, self))
    }
    /// Loads wave data from file into RAM.
    #[inline]
    pub fn new_wave<'aud>(&'aud self, filename: &str) -> Result<Wave<'aud>, String> {
        let c_filename = CString::new(filename).unwrap();
        let w = unsafe { ffi::LoadWave(c_filename.as_ptr()) };
        if w.data.is_null() {
            return Err(format!("Cannot load wave {}", filename));
        }
        Ok(Wave(w, self))
    }

    /// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
    pub fn new_wave_from_memory<'aud>(
        &'aud self,
        filetype: &str,
        bytes: &[u8],
    ) -> Result<Wave<'aud>, String> {
        let c_filetype = CString::new(filetype).unwrap();
        let c_bytes = bytes.as_ptr();
        let w =
            unsafe { ffi::LoadWaveFromMemory(c_filetype.as_ptr(), c_bytes, bytes.len() as i32) };
        if w.data.is_null() {
            return Err(format!("Wave data is null. Check provided buffer data"));
        };
        Ok(Wave(w, self))
    }

    /// Loads music stream from file.
    // #[inline]
    pub fn new_music<'aud>(&'aud self, filename: &str) -> Result<Music<'aud>, String> {
        let c_filename = CString::new(filename).unwrap();
        let m = unsafe { ffi::LoadMusicStream(c_filename.as_ptr()) };
        if m.stream.buffer.is_null() {
            return Err(format!("music could not be loaded from file {}", filename));
        }
        Ok(Music(m, self))
    }

    /// Load music stream from data
    pub fn new_music_from_memory<'aud>(
        &'aud self,
        filetype: &str,
        bytes: &Vec<u8>,
    ) -> Result<Music<'aud>, String> {
        let c_filetype = CString::new(filetype).unwrap();
        let c_bytes = bytes.as_ptr();
        let w = unsafe {
            ffi::LoadMusicStreamFromMemory(c_filetype.as_ptr(), c_bytes, bytes.len() as i32)
        };
        if w.stream.buffer.is_null() {
            return Err(format!(
                "Music's buffer data data is null. Check provided buffer data"
            ));
        };
        Ok(Music(w, self))
    }

    /// Initializes audio stream (to stream raw PCM data).
    #[inline]
    pub fn new_audio_stream<'aud>(
        &'aud self,
        sample_rate: u32,
        sample_size: u32,
        channels: u32,
    ) -> AudioStream<'aud> {
        unsafe {
            AudioStream(
                ffi::LoadAudioStream(sample_rate, sample_size, channels),
                self,
            )
        }
    }
}

impl<'aud> Drop for RaylibAudio {
    fn drop(&mut self) {
        unsafe {
            ffi::CloseAudioDevice();
        }
    }
}

impl<'aud> Wave<'aud> {
    pub fn frame_count(&self) -> u32 {
        self.0.frameCount
    }
    pub fn sample_rate(&self) -> u32 {
        self.0.sampleRate
    }
    pub fn sample_size(&self) -> u32 {
        self.0.sampleSize
    }
    pub fn channels(&self) -> u32 {
        self.0.channels
    }
    pub unsafe fn inner(self) -> ffi::Wave {
        let inner = self.0;
        std::mem::forget(self);
        inner
    }

    pub fn is_wave_ready(&self) -> bool {
        unsafe { ffi::IsWaveReady(self.0) }
    }

    /// Export wave file. Extension must be .wav or .raw
    #[inline]
    pub fn export(&self, filename: impl AsRef<Path>) -> bool {
        let c_filename = CString::new(filename.as_ref().to_string_lossy().as_bytes()).unwrap();
        unsafe { ffi::ExportWave(self.0, c_filename.as_ptr()) }
    }

    /// Export wave sample data to code (.h)
    /*#[inline]
    pub fn export_wave_as_code(&self, filename: &str) -> bool {
        let c_filename = CString::new(filename).unwrap();
        unsafe { ffi::ExportWaveAsCode(self.0, c_filename.as_ptr()) }
    }*/

    /// Copies a wave to a new wave.
    #[inline]
    pub(crate) fn copy(&self) -> Wave {
        unsafe { Wave(ffi::WaveCopy(self.0), self.1) }
    }

    /// Converts wave data to desired format.
    #[inline]
    pub fn format(&mut self, sample_rate: i32, sample_size: i32, channels: i32) {
        unsafe {
            ffi::WaveFormat(&mut self.0, sample_rate, sample_size, channels);
        }
    }

    /// Crops a wave to defined sample range.
    #[inline]
    pub fn crop(&mut self, init_sample: i32, final_sample: i32) {
        unsafe {
            ffi::WaveCrop(&mut self.0, init_sample, final_sample);
        }
    }

    /// Load samples data from wave as a floats array
    /// NOTE 1: Returned sample values are normalized to range [-1..1]
    /// NOTE 2: Sample data allocated should be freed with UnloadWaveSamples()
    #[inline]
    pub fn load_samples(&self) -> WaveSamples {
        let as_slice = unsafe {
            let data = ffi::LoadWaveSamples(self.0);
            Box::from_raw(std::slice::from_raw_parts_mut(
                data,
                self.frame_count() as usize,
            ))
        };
        WaveSamples(ManuallyDrop::new(as_slice))
    }
}

impl<'aud> AsRef<ffi::AudioStream> for Sound<'aud> {
    fn as_ref(&self) -> &ffi::AudioStream {
        &self.0.stream
    }
}

impl<'aud> AsMut<ffi::AudioStream> for Sound<'aud> {
    fn as_mut(&mut self) -> &mut ffi::AudioStream {
        &mut self.0.stream
    }
}

impl<'aud> Sound<'aud> {
    pub fn is_sound_ready(&self) -> bool {
        unsafe { ffi::IsSoundReady(self.0) }
    }

    pub fn frame_count(&self) -> u32 {
        self.0.frameCount
    }
    pub unsafe fn inner(self) -> ffi::Sound {
        let inner = self.0;
        std::mem::forget(self);
        inner
    }

    /// Plays a sound.
    #[inline]
    pub fn play(&self) {
        unsafe {
            ffi::PlaySound(self.0);
        }
    }

    /// Pauses a sound.
    #[inline]
    pub fn pause(&mut self) {
        unsafe {
            ffi::PauseSound(self.0);
        }
    }

    /// Resumes a paused sound.
    #[inline]
    pub fn resume(&mut self) {
        unsafe {
            ffi::ResumeSound(self.0);
        }
    }

    /// Stops playing a sound.
    #[inline]
    pub fn stop(&mut self) {
        unsafe {
            ffi::StopSound(self.0);
        }
    }

    /// Checks if a sound is currently playing.
    #[inline]
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::IsSoundPlaying(self.0) }
    }

    /// Sets volume for a sound (`1.0` is max level).
    #[inline]
    pub fn set_volume(&mut self, volume: f32) {
        unsafe {
            ffi::SetSoundVolume(self.0, volume);
        }
    }

    /// Sets pitch for a sound (`1.0` is base level).
    #[inline]
    pub fn set_pitch(&mut self, pitch: f32) {
        unsafe {
            ffi::SetSoundPitch(self.0, pitch);
        }
    }
    pub fn set_pan(&mut self, pan: f32) {
        unsafe {
            ffi::SetSoundPan(self.0, pan);
        }
    }

    // Uncomment this when Raylib fulfills the todo comment within the original function to make the function safe.
    // /// Updates sound buffer with new data.
    // #[inline]
    // pub fn update<T: AudioSample>(&mut self, data: &[T]) {
    //     unsafe {
    //         ffi::UpdateSound(
    //             self.0,
    //             data.as_ptr() as *const std::os::raw::c_void,
    //             (data.len() * std::mem::size_of::<T>()) as i32,
    //         );
    //     }
    // }}
}

impl<'aud, 'bind> SoundAlias<'aud, 'bind> {
    pub fn is_sound_ready(&self) -> bool {
        unsafe { ffi::IsSoundReady(self.0) }
    }

    pub fn frame_count(&self) -> u32 {
        self.0.frameCount
    }
    pub unsafe fn inner(self) -> ffi::Sound {
        let inner = self.0;
        std::mem::forget(self);
        inner
    }

    /// Plays a sound.
    #[inline]
    pub fn play(&self) {
        unsafe {
            ffi::PlaySound(self.0);
        }
    }

    /// Pauses a sound.
    #[inline]
    pub fn pause(&mut self) {
        unsafe {
            ffi::PauseSound(self.0);
        }
    }

    /// Resumes a paused sound.
    #[inline]
    pub fn resume(&mut self) {
        unsafe {
            ffi::ResumeSound(self.0);
        }
    }

    /// Stops playing a sound.
    #[inline]
    pub fn stop(&mut self) {
        unsafe {
            ffi::StopSound(self.0);
        }
    }

    /// Checks if a sound is currently playing.
    #[inline]
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::IsSoundPlaying(self.0) }
    }

    /// Sets volume for a sound (`1.0` is max level).
    #[inline]
    pub fn set_volume(&mut self, volume: f32) {
        unsafe {
            ffi::SetSoundVolume(self.0, volume);
        }
    }

    /// Sets pitch for a sound (`1.0` is base level).
    #[inline]
    pub fn set_pitch(&mut self, pitch: f32) {
        unsafe {
            ffi::SetSoundPitch(self.0, pitch);
        }
    }
    pub fn set_pan(&mut self, pan: f32) {
        unsafe {
            ffi::SetSoundPan(self.0, pan);
        }
    }
}

impl<'aud> Music<'aud> {
    /// Starts music playing.
    #[inline]
    pub fn play_stream(&mut self) {
        unsafe {
            ffi::PlayMusicStream(self.0);
        }
    }

    /// Updates buffers for music streaming.
    #[inline]
    pub fn update_stream(&mut self) {
        unsafe {
            ffi::UpdateMusicStream(self.0);
        }
    }

    /// Stops music playing.
    #[inline]
    pub fn stop_stream(&mut self) {
        unsafe {
            ffi::StopMusicStream(self.0);
        }
    }

    /// Pauses music playing.
    #[inline]
    pub fn pause_stream(&mut self) {
        unsafe {
            ffi::PauseMusicStream(self.0);
        }
    }

    /// Resumes playing paused music.
    #[inline]
    pub fn resume_stream(&mut self) {
        unsafe {
            ffi::ResumeMusicStream(self.0);
        }
    }

    /// Checks if music is playing.
    #[inline]
    pub fn is_stream_playing(&self) -> bool {
        unsafe { ffi::IsMusicStreamPlaying(self.0) }
    }

    /// Sets volume for music (`1.0` is max level).
    #[inline]
    pub fn set_volume(&mut self, volume: f32) {
        unsafe {
            ffi::SetMusicVolume(self.0, volume);
        }
    }

    /// Sets pitch for music (`1.0` is base level).
    #[inline]
    pub fn set_pitch(&mut self, pitch: f32) {
        unsafe {
            ffi::SetMusicPitch(self.0, pitch);
        }
    }

    /// Gets music time length in seconds.
    #[inline]
    pub fn get_time_length(&self) -> f32 {
        unsafe { ffi::GetMusicTimeLength(self.0) }
    }

    /// Gets current music time played in seconds.
    #[inline]
    pub fn get_time_played(&self) -> f32 {
        unsafe { ffi::GetMusicTimePlayed(self.0) }
    }

    pub fn seek_stream(&mut self, position: f32) {
        unsafe {
            ffi::SeekMusicStream(self.0, position);
        }
    }

    pub fn set_pan(&mut self, pan: f32) {
        unsafe {
            ffi::SetMusicPan(self.0, pan);
        }
    }
}

impl<'aud> AudioStream<'aud> {
    pub fn is_audio_stream_ready(&self) -> bool {
        unsafe { ffi::IsAudioStreamReady(self.0) }
    }
    pub fn sample_rate(&self) -> u32 {
        self.0.sampleRate
    }
    pub fn sample_size(&self) -> u32 {
        self.0.sampleSize
    }
    pub fn channels(&self) -> u32 {
        self.0.channels
    }

    pub unsafe fn inner(self) -> ffi::AudioStream {
        let inner = self.0;
        std::mem::forget(self);
        inner
    }

    /// Updates audio stream buffers with data.
    #[inline]
    pub fn update<T: AudioSample>(&mut self, data: &[T]) {
        unsafe {
            ffi::UpdateAudioStream(
                self.0,
                data.as_ptr() as *const std::os::raw::c_void,
                (data.len() * std::mem::size_of::<T>()) as i32,
            );
        }
    }

    /// Plays audio stream.
    #[inline]
    pub fn play(&mut self) {
        unsafe {
            ffi::PlayAudioStream(self.0);
        }
    }

    /// Pauses audio stream.
    #[inline]
    pub fn pause(&mut self) {
        unsafe {
            ffi::PauseAudioStream(self.0);
        }
    }

    /// Resumes audio stream.
    #[inline]
    pub fn resume(&mut self) {
        unsafe {
            ffi::ResumeAudioStream(self.0);
        }
    }

    /// Checks if audio stream is currently playing.
    #[inline]
    pub fn is_playing(&self) -> bool {
        unsafe { ffi::IsAudioStreamPlaying(self.0) }
    }

    /// Stops audio stream.
    #[inline]
    pub fn stop(&mut self) {
        unsafe {
            ffi::StopAudioStream(self.0);
        }
    }

    /// Sets volume for audio stream (`1.0` is max level).
    #[inline]
    pub fn set_volume(&mut self, volume: f32) {
        unsafe {
            ffi::SetAudioStreamVolume(self.0, volume);
        }
    }

    /// Sets pitch for audio stream (`1.0` is base level).
    #[inline]
    pub fn set_pitch(&mut self, pitch: f32) {
        unsafe {
            ffi::SetAudioStreamPitch(self.0, pitch);
        }
    }

    /// Sets pitch for audio stream (`1.0` is base level).
    #[inline]
    pub fn is_processed(&mut self) -> bool {
        unsafe { ffi::IsAudioStreamProcessed(self.0) }
    }

    pub fn set_pan(&mut self, pan: f32) {
        unsafe {
            ffi::SetAudioStreamPan(self.0, pan);
        }
    }
}

impl<'bind> Sound<'_> {
    pub fn alias<'snd>(&'snd self) -> Result<SoundAlias<'bind, 'snd>, String> {
        let s = unsafe { ffi::LoadSoundAlias(self.0) };
        if s.stream.buffer.is_null() {
            return Err("failed to load sound from wave".to_string());
        }
        Ok(SoundAlias(s, PhantomData))
    }
}

pub struct SoundAlias<'snd, 'bind>(ffi::Sound, PhantomData<&'snd Sound<'bind>>);
