/// Decodes `EncodedAudioChunk` objects.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder
pub struct AudioDecoder {
    decode_queue_size: u32,
    state: String,
}

impl AudioDecoder {
    pub fn new() -> Self {
        unimplemented!()
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
    /// The sample format of the audio.
    pub format: String,
    /// The sample rate of the audio in Hz.
    pub sample_rate: f64,
    /// The number of audio channels.
    pub number_of_channels: u32,
    /// The number of frames.
    pub number_of_frames: u32,
    /// The duration of the audio in microseconds.
    pub duration: f64,
    /// The timestamp of the audio in microseconds.
    pub timestamp: f64,
}

impl AudioData {
    pub fn new() {
        unimplemented!()
    }

    pub fn allocation_size() {}

    pub fn clone() {}

    pub fn close() {}
}
