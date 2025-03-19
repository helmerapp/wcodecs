/// Decodes `EncodedVideoChunk` objects.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder
pub struct VideoDecoder {}

impl VideoDecoder {
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

/// Encodes `VideoFrame` objects.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame
pub struct VideoEncoder {}

impl VideoEncoder {
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

/// Represents codec-specific encoded video bytes.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk
pub struct EncodedVideoChunk {}

/// Represents a frame of unencoded video data.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame
pub struct VideoFrame {}

/// Represents the color space of a video frame.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/colorSpace
pub struct VideoColourSpace {
    pub primaries: String,
    pub transfer: String,
    pub matrix: String,
    pub full_range: bool,
}

impl VideoColourSpace {}
