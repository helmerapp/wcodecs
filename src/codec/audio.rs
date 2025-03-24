use std::sync::{Arc, Mutex};

use crate::{
    core::{
        control::{AudioDecodeMessage, AudioEncodeMessage, ControlMessage, DecodeMessage},
        internal_slots::CodecInternalSlots,
        work_queue::MAX_WORKERS,
    },
    data::audio_data::{AudioData, EncodedAudioChunk},
};

use super::{AudioConfigMessage, AudioDecoderConfig, ConfigMessage, Exception, State};

/// Decodes `EncodedAudioChunk` objects.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder
pub struct AudioDecoder {
    internal_slots: CodecInternalSlots,
    decode_queue_size: u32,
    state: State,
    codec_impl: Arc<Mutex<Option<ffmpeg_next::decoder::Audio>>>,
    output_callback: Arc<dyn Fn(AudioData) + Send + Sync>,
    error_callback: Arc<dyn Fn(Exception) + Send + Sync>,
    key_chunk_required: bool,
}

impl AudioDecoder {
    pub fn new(
        output_callback: impl Fn(AudioData) + Send + Sync + 'static,
        error_callback: impl Fn(Exception) + Send + Sync + 'static,
    ) -> Self {
        let internal_slots = CodecInternalSlots::new(MAX_WORKERS);
        Self {
            internal_slots,
            decode_queue_size: 0,
            state: State::Unconfigured,
            codec_impl: Arc::new(Mutex::new(None)),
            output_callback: Arc::new(output_callback),
            error_callback: Arc::new(error_callback),
            key_chunk_required: true,
        }
    }

    pub fn is_config_supported(&self, config: &AudioDecoderConfig) -> bool {
        config.is_valid()
    }

    // Initialises the underlying decoder with given config.
    pub fn configure(&mut self, config: AudioDecoderConfig) -> Result<(), Exception> {
        if !self.is_config_supported(&config) {
            return Err(Exception::TypeError);
        }
        if self.state == State::Closed {
            return Err(Exception::InvalidStateError);
        }

        self.state = State::Configured;
        self.key_chunk_required = true;

        let config_message = AudioConfigMessage {
            data: config,
            work_queue: self.internal_slots.work_queue.clone(),
            error_callback: self.error_callback.clone(),
            codec_impl: self.codec_impl.clone(),
        };
        self.internal_slots
            .enqueue_control_message(ControlMessage::Config(ConfigMessage::AudioConfig(
                config_message,
            )));
        self.internal_slots.process_control_message_queue();

        Ok(())
    }

    /// Decodes an encoded audio chunk.
    pub fn decode(&mut self, chunk: EncodedAudioChunk) -> Result<(), Exception> {
        if self.state != State::Configured {
            (self.error_callback)(Exception::InvalidStateError);
            return Err(Exception::InvalidStateError);
        }
        if self.key_chunk_required && !chunk.is_key {
            (self.error_callback)(Exception::DecodeError);
            return Err(Exception::DecodeError);
        }
        self.key_chunk_required = false;
        self.decode_queue_size += 1;

        let decode_message = AudioDecodeMessage {
            chunk,
            work_queue: self.internal_slots.work_queue.clone(),
            output_callback: self.output_callback.clone(),
            error_callback: self.error_callback.clone(),
            codec_impl: self.codec_impl.clone(),
        };

        self.internal_slots
            .enqueue_control_message(ControlMessage::Decode(DecodeMessage::AudioDecode(
                decode_message,
            )));
        self.internal_slots.process_control_message_queue();

        self.decode_queue_size -= 1;
        Ok(())
    }

    // Flush the decoder and drain remaining frames.
    pub fn flush(&mut self) -> Result<(), Exception> {
        if self.state != State::Configured {
            return Err(Exception::InvalidStateError);
        }
        {
            let mut dec_lock = self.codec_impl.lock().unwrap();
            if let Some(decoder) = dec_lock.as_mut() {
                decoder.send_eof().map_err(|_| Exception::DecodeError)?;
                //TODO:
                //     let mut frame = ffmpeg_next::frame::Audio::empty();
                //     while decoder.receive_frame(&mut frame).is_ok() {
                //         let audio_data = AudioData::new(
                //             "f32-planar".to_string(),
                //             frame.rate() as f64,
                //             frame.channels() as u32,
                //             frame.samples() as u32,
                //             frame.timestamp().unwrap_or(0) as f64,
                //             frame.data(channel),
                //         );
                //         (self.output_callback)(audio_data);
                //     }
            }
        }
        Ok(())
    }

    /// Resets the decoder and clears the queue.
    pub fn reset(&mut self) {
        if self.state == State::Closed {
            (self.error_callback)(Exception::InvalidStateError);
            return;
        }
        self.state = State::Unconfigured;
        self.decode_queue_size = 0;
        self.internal_slots.control_message_queue.clear();
        {
            let mut dec_lock = self.codec_impl.lock().unwrap();
            *dec_lock = None;
        }
    }

    /// Closes the decoder; aborts any pending work.
    pub fn close(&mut self) {
        self.reset();
        self.state = State::Closed;
    }
}

/// Encodes `AudioData` objects.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/AudioEncoder
pub struct AudioEncoder {
    internal_slots: CodecInternalSlots,
}

impl AudioEncoder {
    pub fn new() -> Self {
        let internal_slots = CodecInternalSlots::new(MAX_WORKERS);
        Self { internal_slots }
    }

    pub fn is_config_supported() {}

    pub fn configure() {}

    pub fn encode(&mut self, data: String) {
        let message = AudioEncodeMessage {
            data,
            work_queue: self.internal_slots.work_queue.clone(),
        };
        self.internal_slots
            .enqueue_control_message(ControlMessage::Encode(
                crate::core::control::EncodeMessage::AudioEncode(message),
            ));
        self.internal_slots.process_control_message_queue();
    }

    pub fn flush() {}

    pub fn reset() {}

    pub fn close() {}
}
