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
