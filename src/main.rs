use std::{thread, time::Duration};

use wcodecs::codec::{AudioDecoder, AudioDecoderConfig, AudioEncoder};

fn main() {
    let mut codec = AudioDecoder::new();

    codec
        .configure(AudioDecoderConfig {
            codec: "".to_string(),
            sample_rate: 200,
            number_of_channels: 2,
            description: None,
        })
        .unwrap();
    codec.decode("Decoding video frame 1".to_string());
    codec.decode("Decoding video frame 2".to_string());
    codec.decode("Decoding video frame 3".to_string());
    codec.decode("Decoding video frame 4".to_string());
    codec.decode("Decoding video frame 5".to_string());

    let mut codec = AudioEncoder::new();

    codec.encode("ecoding video frame 1".to_string());
    codec.encode("ecoding video frame 2".to_string());
    codec.encode("ecoding video frame 3".to_string());
    codec.encode("ecoding video frame 4".to_string());
    codec.encode("ecoding video frame 5".to_string());

    thread::sleep(Duration::from_secs(20));
}
