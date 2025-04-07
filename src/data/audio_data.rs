/// Represents unencoded audio data.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/AudioData
#[derive(Debug)]
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
    pub data: Vec<u8>,
}

impl AudioData {
    pub fn new(
        format: String,
        sample_rate: f64,
        number_of_channels: u32,
        number_of_frames: u32,
        timestamp: f64,
        data: Vec<u8>,
    ) -> Self {
        let duration = (number_of_frames as f64 / sample_rate) * 1_000_000.0;
        AudioData {
            format,
            sample_rate,
            number_of_channels,
            number_of_frames,
            duration,
            timestamp,
            data,
        }
    }
}

/// Represents codec-specific encoded audio bytes.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk
#[derive(Debug, Clone)]
pub struct EncodedAudioChunk {
    pub data: Vec<u8>,
    pub timestamp: i64,
    pub is_key: bool,
}
