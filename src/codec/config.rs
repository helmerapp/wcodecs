use std::sync::{Arc, Mutex};

use crate::core::work_queue::WorkQueue;

use super::Exception;

#[derive(Clone)]
pub enum ConfigMessage {
    AudioConfig(AudioConfigMessage),
    //TODO:
    // VideoConfig,
}

#[derive(Clone)]
pub struct AudioConfigMessage {
    pub config: AudioDecoderConfig,
    pub work_queue: Arc<WorkQueue>,
    pub error_callback: Arc<dyn Fn(Exception) + Send + Sync>,
    pub codec_impl: Arc<Mutex<Option<ffmpeg_next::decoder::Audio>>>,
}

#[derive(Clone)]
pub struct AudioDecoderConfig {
    pub codec: String,
    pub sample_rate: u32,
    pub number_of_channels: u32,
    //TODO:
    // pub description: ?
}

impl AudioDecoderConfig {
    pub fn is_valid(&self) -> bool {
        !self.codec.is_empty() && self.sample_rate > 0 && self.number_of_channels > 0
    }
}
