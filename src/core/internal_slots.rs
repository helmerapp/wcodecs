use std::{collections::VecDeque, sync::Arc};

use crate::codec::ConfigMessage;

use super::{
    control::{
        self, ControlMessage, ControlMessageTrait, DecodeMessage, EncodeMessage, FlushMessage,
        Outcome,
    },
    work_queue::WorkQueue,
};

/// Internal slots shared by codec instances.
pub struct CodecInternalSlots {
    pub control_message_queue: VecDeque<control::ControlMessage>,
    pub message_queue_blocked: bool,
    pub work_queue: Arc<WorkQueue>,
}

impl CodecInternalSlots {
    pub fn new(num_threads: usize) -> Self {
        CodecInternalSlots {
            control_message_queue: VecDeque::new(),
            message_queue_blocked: false,
            work_queue: Arc::new(WorkQueue::new(num_threads)),
        }
    }

    /// Enqueue a control message and process the control message queue.
    pub fn enqueue_control_message(&mut self, msg: control::ControlMessage) {
        self.control_message_queue.push_back(msg);
        self.process_control_message_queue();
    }

    /// Sequential processing
    pub fn process_control_message_queue(&mut self) {
        while !self.message_queue_blocked && !self.control_message_queue.is_empty() {
            if let Some(front_msg) = self.control_message_queue.front_mut() {
                // TODO: refact this
                match front_msg {
                    ControlMessage::Decode(decode_msg) => match decode_msg {
                        DecodeMessage::AudioDecode(dcd_msg) => {
                            let outcome = dcd_msg.process();
                            match outcome {
                                Outcome::NotProcessed => break,
                                Outcome::Processed => {
                                    self.control_message_queue.pop_front();
                                }
                            }
                        }
                    },
                    ControlMessage::Encode(encode_msg) => match encode_msg {
                        EncodeMessage::AudioEncode(enc_msg) => {
                            let outcome = enc_msg.process();
                            match outcome {
                                Outcome::NotProcessed => break,
                                Outcome::Processed => {
                                    self.control_message_queue.pop_front();
                                }
                            }
                        }
                    },
                    ControlMessage::Config(config_msg) => match config_msg {
                        ConfigMessage::AudioConfig(cfg_msg) => {
                            let outcome = cfg_msg.process();
                            match outcome {
                                Outcome::NotProcessed => break,
                                Outcome::Processed => {
                                    self.control_message_queue.pop_front();
                                }
                            }
                        }
                    },
                    ControlMessage::Flush(flush_msg) => match flush_msg {
                        FlushMessage::AudioFlush(fls_msg) => {
                            let outcome = fls_msg.process();
                            match outcome {
                                Outcome::NotProcessed => break,
                                Outcome::Processed => {
                                    self.control_message_queue.pop_front();
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
