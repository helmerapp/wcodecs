use crate::{codec::AudioDecoderConfig, core::work_queue::WorkQueue};
use std::sync::Arc;

/// Outcome returned after processing a control message.
#[derive(Debug)]
pub enum Outcome {
    Processed,
    NotProcessed,
}

/// A trait representing a control message.
pub trait ControlMessageTrait {
    fn process(&mut self) -> Outcome;
}

/// A control message that can be either a decode or an encode message.
pub enum ControlMessage {
    Config(AudioConfigMessage),
    Decode(DecodeMessage),
    Encode(EncodeMessage),
}

/// A decode control message which offloads work to the work queue.
pub struct AudioConfigMessage {
    pub data: AudioDecoderConfig,
    pub work_queue: Arc<WorkQueue>,
}

/// A decode control message which offloads work to the work queue.
pub struct DecodeMessage {
    pub data: String,
    pub work_queue: Arc<WorkQueue>,
}

/// An encode control message which offloads work to the work queue.
pub struct EncodeMessage {
    pub data: String,
    pub work_queue: Arc<WorkQueue>,
}

impl ControlMessageTrait for DecodeMessage {
    fn process(&mut self) -> Outcome {
        let data = self.data.clone();
        let work_queue = self.work_queue.clone();
        // Enqueue the "decoding" work.
        work_queue.enqueue(Box::new(move || {
            println!("Decoding in parallel: {}", data);
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("decoded: {}", data);
        }));
        Outcome::Processed
    }
}

impl ControlMessageTrait for EncodeMessage {
    fn process(&mut self) -> Outcome {
        let data = self.data.clone();
        let work_queue = self.work_queue.clone();
        // Enqueue the "encoding" work.
        work_queue.enqueue(Box::new(move || {
            println!("Encoding in parallel: {}", data);
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("encoded: {}", data);
        }));
        Outcome::Processed
    }
}

impl ControlMessageTrait for AudioConfigMessage {
    fn process(&mut self) -> Outcome {
        let data = self.data.clone();
        let work_queue = self.work_queue.clone();
        work_queue.enqueue(Box::new(move || {
            println!("Actual configuring of the decoder");
            std::thread::sleep(std::time::Duration::from_secs(2));
            println!("configured: {:?}", data);
        }));
        Outcome::Processed
    }
}
