/// Decodes `EncodedAudioChunk` objects.
pub struct AudioDecoder {}

/// Decodes `EncodedVideoChunk` objects.
pub struct VideoDecoder {}

/// Encodes `AudioData` objects.
pub struct AudioEncoder {}

/// Encodes `VideoFrame` objects.
pub struct VideoEncoder {}

/// Represents codec-specific encoded audio bytes.
pub struct EncodedAudioChunk {}

/// Represents codec-specific encoded video bytes.
pub struct EncodedVideoChunk {}

/// Represents unencoded audio data.
pub struct AudioData {
    duration: f64,
    format: String,
    number_of_channels: u32,
    number_of_frames: u32,
    sample_rate: u32,
    timestamp: f64,
}

/// Represents a frame of unencoded video data.
pub struct VideoFrame {}

/// Represents the color space of a video frame.
pub struct VideoColourSpace {}

/// Unpacks and decodes image data, giving access to the sequence of frames in an animated image.
pub struct ImageDecoder {}

/// Represents an individual image track.
pub struct ImageTrack {}

/// Represents the list of tracks available in the image.
pub struct ImageTrackList {}
