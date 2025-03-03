/// Decodes `EncodedAudioChunk` objects.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder
pub struct AudioDecoder {}

impl AudioDecoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_config_supported() {}

    pub fn configure() {}

    pub fn decode() {}

    pub fn flush() {}

    pub fn reset() {}

    pub fn close() {}
}

/// Encodes `AudioData` objects.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder
pub struct AudioEncoder {}

impl AudioEncoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_config_supported() {}

    pub fn configure() {}

    pub fn encode() {}

    pub fn flush() {}

    pub fn reset() {}

    pub fn close() {}
}

/// Represents codec-specific encoded audio bytes.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk
pub struct EncodedAudioChunk {}

/// Represents unencoded audio data.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/AudioData
pub struct AudioData {
    pub duration: f64,
    pub format: String,
    pub number_of_channels: u32,
    pub number_of_frames: u32,
    pub sample_rate: u32,
    pub timestamp: f64,
}
