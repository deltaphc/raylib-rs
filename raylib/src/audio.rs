//! Contains code related to audio. [`RaylibAudio`] plays sounds and music.
use crate::{buffer::RaylibBuffer, core::RaylibThread};
use crate::{ffi, make_bound_thin_wrapper, make_thin_wrapper};
use std::ffi::CString;

make_thin_wrapper!(Wave, ffi::Wave, ffi::UnloadWave);
make_bound_thin_wrapper!(Sound, ffi::Sound, ffi::UnloadSound, RaylibAudio);
make_bound_thin_wrapper!(Music, ffi::Music, ffi::UnloadMusicStream, RaylibAudio);
make_bound_thin_wrapper!(
    AudioStream,
    ffi::AudioStream,
    ffi::UnloadAudioStream,
    RaylibAudio
);

/// A marker trait specifying an audio sample (`u8`, `i16`, or `f32`).
pub trait AudioSample {}
impl AudioSample for u8 {}
impl AudioSample for i16 {}
impl AudioSample for f32 {}

/// This token is used to indicate audio is initialized
#[derive(Debug)]
pub struct RaylibAudio(());

impl RaylibAudio {
    /// Initializes audio device and context.
    #[inline]
    pub fn init_audio_device() -> RaylibAudio {
        unsafe {
            ffi::InitAudioDevice();
        }
        RaylibAudio(())
    }

    /// Checks if audio device is ready.
    #[inline]
    pub fn is_audio_device_ready(&self) -> bool {
        unsafe { ffi::IsAudioDeviceReady() }
    }

    /// Sets master volume (listener).
    #[inline]
    pub fn set_master_volume(&self, volume: f32) {
        unsafe {
            ffi::SetMasterVolume(volume);
        }
    }

    /// Plays a sound.
    #[inline]
    pub fn play_sound(&self, sound: &Sound) {
        unsafe {
            ffi::PlaySound(sound.0);
        }
    }

    /// Pauses a sound.
    #[inline]
    pub fn pause_sound(&self, sound: &Sound) {
        unsafe {
            ffi::PauseSound(sound.0);
        }
    }

    /// Resumes a paused sound.
    #[inline]
    pub fn resume_sound(&self, sound: &Sound) {
        unsafe {
            ffi::ResumeSound(sound.0);
        }
    }

    /// Stops playing a sound.
    #[inline]
    pub fn stop_sound(&self, sound: &Sound) {
        unsafe {
            ffi::StopSound(sound.0);
        }
    }

    /// Checks if a sound is currently playing.
    #[inline]
    pub fn is_sound_playing(&self, sound: &Sound) -> bool {
        unsafe { ffi::IsSoundPlaying(sound.0) }
    }

    /// Sets volume for a sound (`1.0` is max level).
    #[inline]
    pub fn set_sound_volume(&self, sound: &Sound, volume: f32) {
        unsafe {
            ffi::SetSoundVolume(sound.0, volume);
        }
    }

    /// Sets pitch for a sound (`1.0` is base level).
    #[inline]
    pub fn set_sound_pitch(&self, sound: &Sound, pitch: f32) {
        unsafe {
            ffi::SetSoundPitch(sound.0, pitch);
        }
    }

    /// Starts music playing.
    #[inline]
    pub fn play_music_stream(&self, music: &mut Music) {
        unsafe {
            ffi::PlayMusicStream(music.0);
        }
    }

    /// Updates buffers for music streaming.
    #[inline]
    pub fn update_music_stream(&self, music: &mut Music) {
        unsafe {
            ffi::UpdateMusicStream(music.0);
        }
    }

    /// Stops music playing.
    #[inline]
    pub fn stop_music_stream(&self, music: &mut Music) {
        unsafe {
            ffi::StopMusicStream(music.0);
        }
    }

    /// Pauses music playing.
    #[inline]
    pub fn pause_music_stream(&self, music: &mut Music) {
        unsafe {
            ffi::PauseMusicStream(music.0);
        }
    }

    /// Resumes playing paused music.
    #[inline]
    pub fn resume_music_stream(&self, music: &mut Music) {
        unsafe {
            ffi::ResumeMusicStream(music.0);
        }
    }

    /// Checks if music is playing.
    #[inline]
    pub fn is_music_stream_playing(&self, music: &Music) -> bool {
        unsafe { ffi::IsMusicStreamPlaying(music.0) }
    }

    /// Sets volume for music (`1.0` is max level).
    #[inline]
    pub fn set_music_volume(&self, music: &mut Music, volume: f32) {
        unsafe {
            ffi::SetMusicVolume(music.0, volume);
        }
    }

    /// Sets pitch for music (`1.0` is base level).
    #[inline]
    pub fn set_music_pitch(&self, music: &mut Music, pitch: f32) {
        unsafe {
            ffi::SetMusicPitch(music.0, pitch);
        }
    }

    /// Gets music time length in seconds.
    #[inline]
    pub fn get_music_time_length(&self, music: &Music) -> f32 {
        unsafe { ffi::GetMusicTimeLength(music.0) }
    }

    /// Gets current music time played in seconds.
    #[inline]
    pub fn get_music_time_played(&self, music: &Music) -> f32 {
        unsafe { ffi::GetMusicTimePlayed(music.0) }
    }

    /// Plays audio stream.
    #[inline]
    pub fn play_audio_stream(&self, stream: &mut AudioStream) {
        unsafe {
            ffi::PlayAudioStream(stream.0);
        }
    }

    /// Pauses audio stream.
    #[inline]
    pub fn pause_audio_stream(&self, stream: &mut AudioStream) {
        unsafe {
            ffi::PauseAudioStream(stream.0);
        }
    }

    /// Resumes audio stream.
    #[inline]
    pub fn resume_audio_stream(&self, stream: &mut AudioStream) {
        unsafe {
            ffi::ResumeAudioStream(stream.0);
        }
    }

    /// Checks if audio stream is currently playing.
    #[inline]
    pub fn is_audio_stream_playing(&self, stream: &AudioStream) -> bool {
        unsafe { ffi::IsAudioStreamPlaying(stream.0) }
    }

    /// Stops audio stream.
    #[inline]
    pub fn stop_audio_stream(&self, stream: &mut AudioStream) {
        unsafe {
            ffi::StopAudioStream(stream.0);
        }
    }

    /// Sets volume for audio stream (`1.0` is max level).
    #[inline]
    pub fn set_audio_stream_volume(&self, stream: &mut AudioStream, volume: f32) {
        unsafe {
            ffi::SetAudioStreamVolume(stream.0, volume);
        }
    }

    /// Sets pitch for audio stream (`1.0` is base level).
    #[inline]
    pub fn set_audio_stream_pitch(&self, stream: &mut AudioStream, pitch: f32) {
        unsafe {
            ffi::SetAudioStreamPitch(stream.0, pitch);
        }
    }

    /// Sets pitch for audio stream (`1.0` is base level).
    #[inline]
    pub fn is_audio_stream_processed(&self, stream: &AudioStream) -> bool {
        unsafe { ffi::IsAudioStreamProcessed(stream.0) }
    }
}

impl Drop for RaylibAudio {
    fn drop(&mut self) {
        unsafe {
            ffi::CloseAudioDevice();
        }
    }
}

impl Wave {
    pub fn frame_count(&self) -> u32 {
        self.0.frameCount
    }
    pub fn smaple_rate(&self) -> u32 {
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
    /// Loads wave data from file into RAM.
    #[inline]
    pub fn load_wave(filename: &str) -> Result<Wave, String> {
        let c_filename = CString::new(filename).unwrap();
        let w = unsafe { ffi::LoadWave(c_filename.as_ptr()) };
        if w.data.is_null() {
            return Err(format!("Cannot load wave {}", filename));
        }
        Ok(Wave(w))
    }

    /// Export wave file. Extension must be .wav or .raw
    #[inline]
    pub fn export_wave(&self, filename: &str) -> bool {
        let c_filename = CString::new(filename).unwrap();
        unsafe { ffi::ExportWave(self.0, c_filename.as_ptr()) }
    }

    /// Export wave sample data to code (.h)
    #[inline]
    pub fn export_wave_as_code(&self, filename: &str) -> bool {
        let c_filename = CString::new(filename).unwrap();
        unsafe { ffi::ExportWaveAsCode(self.0, c_filename.as_ptr()) }
    }

    /// Converts wave data to desired format.
    #[inline]
    pub fn wave_format(&mut self, sample_rate: i32, sample_size: i32, channels: i32) {
        unsafe {
            ffi::WaveFormat(&mut self.0, sample_rate, sample_size, channels);
        }
    }

    /// Copies a wave to a new wave.
    #[inline]
    pub fn wave_copy(&self) -> Wave {
        unsafe { Wave(ffi::WaveCopy(self.0)) }
    }

    /// Crops a wave to defined sample range.
    #[inline]
    pub fn wave_crop(&mut self, init_sample: i32, final_sample: i32) {
        unsafe {
            ffi::WaveCrop(&mut self.0, init_sample, final_sample);
        }
    }

    /// Load samples data from wave as a floats array
    /// NOTE 1: Returned sample values are normalized to range [-1..1]
    /// NOTE 2: Sample data allocated should be freed with UnloadWaveSamples()
    #[inline]
    pub fn load_wave_samples(&self) -> RaylibBuffer<'static, f32> {
        unsafe {
            RaylibBuffer::new(ffi::LoadWaveSamples(self.0), self.frame_count() as usize).unwrap()
        }
    }
}

impl AsRef<ffi::AudioStream> for Sound<'_, '_> {
    fn as_ref(&self) -> &ffi::AudioStream {
        &self.0.stream
    }
}

impl AsMut<ffi::AudioStream> for Sound<'_, '_> {
    fn as_mut(&mut self) -> &mut ffi::AudioStream {
        &mut self.0.stream
    }
}

impl<'bind, 'a> Sound<'bind, 'a> {
    pub fn frame_count(&self) -> u32 {
        self.0.frameCount
    }
    pub unsafe fn inner(self) -> ffi::Sound {
        let inner = self.0;
        std::mem::forget(self);
        inner
    }
    /// Loads sound from file.
    pub fn load_sound(_: &'bind RaylibAudio, filename: &str) -> Result<Sound<'bind, 'a>, String> {
        let c_filename = CString::new(filename).unwrap();
        let s = unsafe { ffi::LoadSound(c_filename.as_ptr()) };
        if s.stream.buffer.is_null() {
            return Err(format!("failed to load sound {}", filename));
        }
        Ok(unsafe { Sound::from_raw(s) })
    }

    /// Loads sound from wave data.
    pub fn load_sound_from_wave(
        _: &'bind RaylibAudio,
        wave: &Wave,
    ) -> Result<Sound<'bind, 'a>, String> {
        let s = unsafe { ffi::LoadSoundFromWave(wave.0) };
        if s.stream.buffer.is_null() {
            return Err("failed to load sound from wave".to_string());
        }
        Ok(unsafe { Sound::from_raw(s) })
    }

    // Figure out how to make this safe
    // /// Updates sound buffer with new data.
    // #[inline]
    // pub fn update_sound(&mut self, data: &[impl AudioSample]) {
    //     unsafe {
    //         ffi::UpdateSound(
    //             self.0,
    //             data.as_ptr() as *const std::os::raw::c_void,
    //             data.len() as i32,
    //         );
    //     }
    // }
}

impl<'bind, 'a> Music<'bind, 'a> {
    /// Loads music stream from file.
    // #[inline]
    pub fn load_music_stream(
        _: &'bind RaylibAudio,
        _: &RaylibThread,
        filename: &str,
    ) -> Result<Music<'bind, 'a>, String> {
        let c_filename = CString::new(filename).unwrap();
        let m = unsafe { ffi::LoadMusicStream(c_filename.as_ptr()) };
        if m.stream.buffer.is_null() {
            return Err(format!("music could not be loaded from file {}", filename));
        }
        Ok(unsafe { Music::from_raw(m) })
    }
}

impl<'bind, 'a> AudioStream<'bind, 'a> {
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
    /// Initializes audio stream (to stream raw PCM data).
    #[inline]
    pub fn load_audio_stream(
        _: &'bind RaylibAudio,
        _: &RaylibThread,
        sample_rate: u32,
        sample_size: u32,
        channels: u32,
    ) -> AudioStream<'bind, 'a> {
        unsafe { AudioStream::from_raw(ffi::LoadAudioStream(sample_rate, sample_size, channels)) }
    }

    /// Updates audio stream buffers with data.
    #[inline]
    pub fn update_audio_stream<T: AudioSample>(&mut self, data: &[T]) {
        assert_eq!(self.0.sampleSize as usize, core::mem::size_of::<T>() * 8);

        unsafe { ffi::UpdateAudioStream(self.0, data.as_ptr() as _, data.len() as i32) }
    }
}
