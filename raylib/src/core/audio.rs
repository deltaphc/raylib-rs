//! Contains code related to audio. [`RaylibAudio`] plays sounds and music.

use crate::core::RaylibThread;
use crate::ffi;
use std::ffi::CString;

make_thin_wrapper!(Wave, ffi::Wave, ffi::UnloadWave);
make_thin_wrapper!(Sound, ffi::Sound, ffi::UnloadSound);
make_thin_wrapper!(Music, ffi::Music, ffi::UnloadMusicStream);
make_thin_wrapper!(AudioStream, ffi::AudioStream, ffi::CloseAudioStream);

/// A marker trait specifying an audio sample (`u8`, `i16`, or `f32`).
pub trait AudioSample {}
impl AudioSample for u8 {}
impl AudioSample for i16 {}
impl AudioSample for f32 {}

/// This token is used to indicate VR is initialized
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
    pub fn play_sound(&mut self, sound: &Sound) {
        unsafe {
            ffi::PlaySound(sound.0);
        }
    }

    /// Pauses a sound.
    #[inline]
    pub fn pause_sound(&mut self, sound: &Sound) {
        unsafe {
            ffi::PauseSound(sound.0);
        }
    }

    /// Resumes a paused sound.
    #[inline]
    pub fn resume_sound(&mut self, sound: &Sound) {
        unsafe {
            ffi::ResumeSound(sound.0);
        }
    }

    /// Stops playing a sound.
    #[inline]
    pub fn stop_sound(&mut self, sound: &Sound) {
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
    pub fn set_sound_volume(&mut self, sound: &Sound, volume: f32) {
        unsafe {
            ffi::SetSoundVolume(sound.0, volume);
        }
    }

    /// Sets pitch for a sound (`1.0` is base level).
    #[inline]
    pub fn set_sound_pitch(&mut self, sound: &Sound, pitch: f32) {
        unsafe {
            ffi::SetSoundPitch(sound.0, pitch);
        }
    }

    /// Starts music playing.
    #[inline]
    pub fn play_music_stream(&mut self, music: &mut Music) {
        unsafe {
            ffi::PlayMusicStream(music.0);
        }
    }

    /// Updates buffers for music streaming.
    #[inline]
    pub fn update_music_stream(&mut self, music: &mut Music) {
        unsafe {
            ffi::UpdateMusicStream(music.0);
        }
    }

    /// Stops music playing.
    #[inline]
    pub fn stop_music_stream(&mut self, music: &mut Music) {
        unsafe {
            ffi::StopMusicStream(music.0);
        }
    }

    /// Pauses music playing.
    #[inline]
    pub fn pause_music_stream(&mut self, music: &mut Music) {
        unsafe {
            ffi::PauseMusicStream(music.0);
        }
    }

    /// Resumes playing paused music.
    #[inline]
    pub fn resume_music_stream(&mut self, music: &mut Music) {
        unsafe {
            ffi::ResumeMusicStream(music.0);
        }
    }

    /// Checks if music is playing.
    #[inline]
    pub fn is_music_playing(&self, music: &Music) -> bool {
        unsafe { ffi::IsMusicPlaying(music.0) }
    }

    /// Sets volume for music (`1.0` is max level).
    #[inline]
    pub fn set_music_volume(&mut self, music: &mut Music, volume: f32) {
        unsafe {
            ffi::SetMusicVolume(music.0, volume);
        }
    }

    /// Sets pitch for music (`1.0` is base level).
    #[inline]
    pub fn set_music_pitch(&mut self, music: &mut Music, pitch: f32) {
        unsafe {
            ffi::SetMusicPitch(music.0, pitch);
        }
    }

    /// Sets music loop count (loop repeats).
    #[inline]
    pub fn set_music_loop_count(&mut self, music: &mut Music, count: i32) {
        unsafe {
            ffi::SetMusicLoopCount(music.0, count);
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
    pub fn play_audio_stream(&mut self, stream: &mut AudioStream) {
        unsafe {
            ffi::PlayAudioStream(stream.0);
        }
    }

    /// Pauses audio stream.
    #[inline]
    pub fn pause_audio_stream(&mut self, stream: &mut AudioStream) {
        unsafe {
            ffi::PauseAudioStream(stream.0);
        }
    }

    /// Resumes audio stream.
    #[inline]
    pub fn resume_audio_stream(&mut self, stream: &mut AudioStream) {
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
    pub fn stop_audio_stream(&mut self, stream: &mut AudioStream) {
        unsafe {
            ffi::StopAudioStream(stream.0);
        }
    }

    /// Sets volume for audio stream (`1.0` is max level).
    #[inline]
    pub fn set_audio_stream_volume(&mut self, stream: &mut AudioStream, volume: f32) {
        unsafe {
            ffi::SetAudioStreamVolume(stream.0, volume);
        }
    }

    /// Sets pitch for audio stream (`1.0` is base level).
    #[inline]
    pub fn set_audio_stream_pitch(&mut self, stream: &mut AudioStream, pitch: f32) {
        unsafe {
            ffi::SetAudioStreamPitch(stream.0, pitch);
        }
    }
}

impl Drop for RaylibAudio {
    fn drop(&mut self) {
        unsafe { ffi::CloseAudioDevice() }
    }
}

impl Wave {
    pub fn sample_count(&self) -> u32 {
        self.0.sampleCount
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
    pub fn export_wave(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe { ffi::ExportWave(self.0, c_filename.as_ptr()) }
    }

    /// Export wave sample data to code (.h)
    #[inline]
    pub fn export_wave_as_code(&self, filename: &str) {
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

    /// Gets sample data from wave as an `f32` array.
    #[inline]
    pub fn get_wave_data(&self) -> Vec<f32> {
        unsafe {
            let data = ffi::GetWaveData(self.0);
            let data_size = (self.sampleCount * self.channels) as usize;
            let mut samples = Vec::with_capacity(data_size);
            samples.set_len(data_size);
            std::ptr::copy(data, samples.as_mut_ptr(), data_size);
            libc::free(data as *mut libc::c_void);
            samples
        }
    }
}

impl Sound {
    pub fn sample_count(&self) -> u32 {
        self.0.sampleCount
    }
    pub unsafe fn inner(self) -> ffi::Sound {
        let inner = self.0;
        std::mem::forget(self);
        inner
    }
    /// Loads sound from file.
    pub fn load_sound(filename: &str) -> Result<Sound, String> {
        let c_filename = CString::new(filename).unwrap();
        let s = unsafe { ffi::LoadSound(c_filename.as_ptr()) };
        if s.stream.buffer.is_null() {
            return Err(format!("failed to load sound {}", filename));
        }
        Ok(Sound(s))
    }

    /// Loads sound from wave data.
    pub fn load_sound_from_wave(wave: &Wave) -> Result<Sound, String> {
        let s = unsafe { ffi::LoadSoundFromWave(wave.0) };
        if s.stream.buffer.is_null() {
            return Err(format!("failed to load sound from wave"));
        }
        Ok(Sound(s))
    }

    /// Updates sound buffer with new data.
    #[inline]
    pub fn update_sound(&mut self, data: &[impl AudioSample]) {
        unsafe {
            ffi::UpdateSound(
                self.0,
                data.as_ptr() as *const std::os::raw::c_void,
                data.len() as i32,
            );
        }
    }
}

impl std::convert::AsRef<ffi::AudioStream> for Sound {
    fn as_ref(&self) -> &ffi::AudioStream {
        return &self.stream;
    }
}

impl std::convert::AsMut<ffi::AudioStream> for Sound {
    fn as_mut(&mut self) -> &mut ffi::AudioStream {
        return &mut self.stream;
    }
}

impl Music {
    /// Loads music stream from file.
    // #[inline]
    pub fn load_music_stream(_: &RaylibThread, filename: &str) -> Result<Music, String> {
        let c_filename = CString::new(filename).unwrap();
        let m = unsafe { ffi::LoadMusicStream(c_filename.as_ptr()) };
        if m.ctxData.is_null() {
            return Err(format!("music could not be loaded from file {}", filename));
        }
        Ok(Music(m))
    }
}

impl AudioStream {
    pub fn sample_rate(&self) -> u32 {
        self.0.sampleRate
    }
    pub fn sample_size(&self) -> u32 {
        self.0.sampleSize
    }
    pub fn channels(&self) -> u32 {
        self.0.channels
    }
    pub fn buffer(&self) -> &ffi::rAudioBuffer {
        unsafe { &*self.0.buffer }
    }
    pub fn buffer_mut(&mut self) -> &mut ffi::rAudioBuffer {
        unsafe { &mut *self.0.buffer }
    }
    pub unsafe fn inner(self) -> ffi::AudioStream {
        let inner = self.0;
        std::mem::forget(self);
        inner
    }
    /// Initializes audio stream (to stream raw PCM data).
    #[inline]
    pub fn init_audio_stream(
        _: &RaylibThread,
        sample_rate: u32,
        sample_size: u32,
        channels: u32,
    ) -> AudioStream {
        unsafe { AudioStream(ffi::InitAudioStream(sample_rate, sample_size, channels)) }
    }

    /// Updates audio stream buffers with data.
    #[inline]
    pub fn update_audio_stream(&mut self, data: &[impl AudioSample]) {
        unsafe {
            ffi::UpdateAudioStream(
                self.0,
                data.as_ptr() as *const std::os::raw::c_void,
                data.len() as i32,
            );
        }
    }
}
