use crate::core::{
    control::{AudioConfigMessage, ControlMessage},
    work_queue::{CodecInternalSlots, MAX_WORKERS},
};

use super::{state, AudioDecoderConfig, DecodeMessage, EncodeMessage, Exception, State};

/// Decodes `EncodedAudioChunk` objects.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/AudioDecoder
pub struct AudioDecoder {
    internal_slots: CodecInternalSlots,
    decode_queue_size: u32,
    state: Option<State>,
}

impl Default for AudioDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioDecoder {
    pub fn new() -> Self {
        let internal_slots = CodecInternalSlots::new(MAX_WORKERS);
        Self {
            internal_slots,
            decode_queue_size: 0,
            state: None,
        }
    }

    pub fn is_config_supported(&self, config: &AudioDecoderConfig) -> bool {
        config.is_valid()
    }

    pub fn configure(&mut self, config: AudioDecoderConfig) -> Result<(), Exception> {
        if !self.is_config_supported(&config) {
            return Err(Exception::TypeError);
        }
        if let Some(state) = &self.state {
            if *state == State::Closed {
                return Err(Exception::InvalidStateError);
            }
        }
        self.state = Some(State::Configured);

        let config_message = AudioConfigMessage {
            data: config,
            work_queue: self.internal_slots.work_queue.clone(),
        };

        self.internal_slots
            .enqueue_control_message(ControlMessage::Config(config_message));
        self.internal_slots.process_control_message_queue();
        Ok(())
    }

    pub fn decode(&mut self, data: String) {
        let message = DecodeMessage {
            data,
            work_queue: self.internal_slots.work_queue.clone(),
        };
        self.internal_slots
            .enqueue_control_message(ControlMessage::Decode(message));
        self.internal_slots.process_control_message_queue();
    }

    pub fn flush() {
        todo!()
    }

    pub fn reset() {
        todo!()
    }

    pub fn close() {
        todo!()
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
        let message = EncodeMessage {
            data,
            work_queue: self.internal_slots.work_queue.clone(),
        };
        self.internal_slots
            .enqueue_control_message(ControlMessage::Encode(message));
        self.internal_slots.process_control_message_queue();
    }

    pub fn flush() {}

    pub fn reset() {}

    pub fn close() {}
}

/// Represents codec-specific encoded audio bytes.
///
/// https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk
pub struct EncodedAudioChunk {}
