//! Definitions for error types used throught the crate

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioInitError {
    #[error("RaylibAudio cannot be instantiated more then once at a time")]
    DoubleInit,
    #[error("failed to initialize audio device")]
    InitFailed,
}

#[derive(Error, Debug)]
pub enum ExportWaveError {
    #[error("wave data must be 16 bit per sample for QOA format export (actual: {0})")]
    QoaBadSamples(i32),
    #[error("failed to export wave data")]
    ExportFailed,
}

#[derive(Error, Debug)]
pub enum LoadSoundError {
    #[error("failed to load sound\npath: {path:?}")]
    LoadFailed { path: String },
    #[error("failed to load sound from wave")]
    LoadFromWaveFailed,
    #[error("cannot load wave\npath: {path:?}")]
    LoadWaveFromFileFailed { path: String },
    #[error("wave data is null, check provided buffer data")]
    Null,
    #[error("music could not be loaded from file\npath: {path:?}")]
    LoadMusicFromFileFailed { path: String },
    #[error("music's buffer data data is null, check provided buffer data")]
    MusicNull,
}

#[derive(Error, Debug)]
pub enum AllocationError {
    #[error("memory request does not produce a valid layout")]
    InvalidLayout,
    #[error("memory request exceeds capacity")]
    ExceedsCapacity,
    #[error("memory request exceeds unsigned integer maximum")]
    ExceedsUIntMax,
    #[error("cannot allocate less than 1 element")]
    SubMinSize,
}

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("could not compress data")]
    CompressionFailed,
}

#[derive(Error, Debug)]
pub enum LoadModelError {
    #[error("could not load model\npath: {path:?}")]
    LoadFromFileFailed { path: String },
    #[error("could not load model from mesh")]
    LoadFromMeshFailed,
}

#[derive(Error, Debug)]
pub enum LoadModelAnimError {
    #[error("no model animations loaded\npath: {path:?}")]
    NoAnimationsLoaded { path: String },
}

#[derive(Error, Debug)]
pub enum SetMaterialError {
    #[error("mesh_id greater than mesh count")]
    MeshIdOutOfBounds,
    #[error("material_id greater than material count")]
    MaterialIdOutOfBounds,
}

#[derive(Error, Debug)]
pub enum LoadMaterialError {
    #[error("no materials loaded\npath: {path:?}")]
    NoneLoaded { path: String },
}

#[derive(Error, Debug)]
pub enum LoadFontError {
    #[error("error loading font; check if the file exists and if it's the right type\npath: {path:?}")]
    LoadFromFileFailed { path: String },
    #[error("error loading font from image")]
    LoadFromImageFailed,
    #[error("error loading font from memory; check if the file's type is correct")]
    LoadFromMemoryFailed,
}

#[derive(Error, Debug)]
pub enum InvalidImageError {
    #[error("invalid image: width is 0")]
    ZeroWidth,
    #[error("invalid image: height is 0")]
    ZeroHeight,
    #[error("invalid image: data is null")]
    NullData,
    #[error("image data is null, either the file doesnt exist or the image type is unsupported")]
    NullDataFromFile,
    #[error("invalid file data")]
    InvalidFile,
    #[error("image data is null, check provided buffer data")]
    NullDataFromMemory,
    #[error("failed to retrieve pixel data")]
    NullDataFromTexture,
    #[error("unsupported format")]
    UnsupportedFormat,
    #[error("convolution kernel must be square to be applied")]
    NonSquareKernel,
}

#[derive(Error, Debug)]
pub enum UpdateTextureError {
    #[error("data is wrong size (expected {expect} bytes, got {actual})")]
    WrongDataSize { expect: usize, actual: usize },
    #[error("destination rectangle cannot exceed texture bounds")]
    OutOfBounds,
    #[error("destination rectangle cannot have negative extents")]
    NegativeSize,
}

#[derive(Error, Debug)]
pub enum LoadTextureError {
    #[error("failed to load the texture\npath: {path:?}")]
    TextureFromFileFailed { path: String },
    #[error("failed to load image as a texture cubemap")]
    CubemapFromImageFailed,
    #[error("failed to load image as a texture")]
    TextureFromImageFailed,
    #[error("failed to create render texture")]
    CreateRenderTextureFailed,
    #[error("data is not valid to load texture")]
    InvalidData,
}

#[derive(Error, Debug)]
pub enum RaylibError {
    #[error("audio initialization error")]
    AudioInit(#[from] AudioInitError),
    #[error("wave export error")]
    ExportWave(#[from] ExportWaveError),
    #[error("sound loading error")]
    LoadSound(#[from] LoadSoundError),
    #[error("allocation error")]
    Allocation(#[from] AllocationError),
    #[error("compression error")]
    Compression(#[from] CompressionError),
    #[error("model loading error")]
    LoadModel(#[from] LoadModelError),
    #[error("model animation loading error")]
    LoadModelAnim(#[from] LoadModelAnimError),
    #[error("material update error")]
    SetMaterial(#[from] SetMaterialError),
    #[error("material loading error")]
    LoadMaterial(#[from] LoadMaterialError),
    #[error("font loading error")]
    LoadFont(#[from] LoadFontError),
    #[error("image error")]
    InvalidImage(#[from] InvalidImageError),
    #[error("texture update error")]
    UpdateTexture(#[from] UpdateTextureError),
    #[error("texture loading error")]
    LoadTexture(#[from] LoadTextureError),
}
