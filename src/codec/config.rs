#[derive(Clone, Debug)]
pub struct AudioDecoderConfig {
    pub codec: String,
    pub sample_rate: u32,
    pub number_of_channels: u32,
    pub description: Option<Vec<u8>>,
}

impl AudioDecoderConfig {
    pub fn is_valid(&self) -> bool {
        if self.codec.trim().is_empty() {
            return false;
        }

        true
    }
}
