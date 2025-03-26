use std::{thread::sleep, time::Duration};

use wcodecs::{
    codec::{AudioDecoder, AudioDecoderConfig},
    data::audio_data::EncodedAudioChunk,
};

fn main() {
    // ffmpeg_next::init() is not needed because it is already done in wcodecs internally (if not already initialised).

    let mut decoder = AudioDecoder::new(
        |audio_data| {
            println!("Decoded AudioData: {:?}", audio_data);
        },
        |err| {
            eprintln!("Decoder error: {:?}", err);
        },
    );

    let config = AudioDecoderConfig {
        codec: "mp3".to_string(),
        sample_rate: 44100,
        number_of_channels: 2,
    };

    if let Err(e) = decoder.configure(config.clone()) {
        eprintln!("Failed to configure decoder: {:?}", e);
        return;
    }

    let input_path = "./examples/samples/beep.mp3";
    let mut ictx = match ffmpeg_next::format::input(input_path) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Failed to open {}: {:?}", input_path, e);
            return;
        }
    };

    let audio_stream_index = ictx
        .streams()
        .best(ffmpeg_next::media::Type::Audio)
        .map(|s| s.index())
        .unwrap_or_else(|| {
            eprintln!("No audio stream found.");
            std::process::exit(1);
        });

    for (stream, packet) in ictx.packets() {
        if stream.index() != audio_stream_index {
            continue;
        }

        let data = match packet.data() {
            Some(d) => d.to_vec(),
            None => {
                eprintln!("No data found in packet.");
                continue;
            }
        };
        let timestamp = packet.pts().unwrap_or(0);

        let encoded_chunk = EncodedAudioChunk {
            data,
            timestamp,
            is_key: true,
        };

        if let Err(e) = decoder.decode(encoded_chunk) {
            eprintln!(
                "Failed to decode packet at timestamp {}: {:?}",
                timestamp, e
            );
        } else {
            println!("Sent packet at timestamp {}", timestamp);
        }

        sleep(Duration::from_millis(50));
    }

    if let Err(e) = decoder.flush() {
        eprintln!("Failed to flush decoder: {:?}", e);
    }

    decoder.reset();
    decoder.close();

    println!("AudioDecoder closed.");
}
