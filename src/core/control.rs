use crate::{
    codec::{AudioConfigMessage, ConfigMessage, Exception},
    core::work_queue::WorkQueue,
    data::audio_data::{AudioData, EncodedAudioChunk},
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum Outcome {
    Processed,
    NotProcessed,
}

pub trait ControlMessageTrait {
    fn process(&mut self) -> Outcome;
}

pub enum ControlMessage {
    Config(ConfigMessage),
    Decode(DecodeMessage),
    Encode(EncodeMessage),
    Flush(FlushMessage),
}

pub enum DecodeMessage {
    AudioDecode(AudioDecodeMessage),
    //TODO:
    // VideoDecode(VideoDecodeMessage)
}

pub enum EncodeMessage {
    AudioEncode(AudioEncodeMessage),
}

pub enum FlushMessage {
    AudioFlush(AudioFlushMessage),
    //TODO:
    // VideoDecode(VideoDecodeMessage)
}

pub struct AudioDecodeMessage {
    pub chunk: EncodedAudioChunk,
    pub work_queue: Arc<WorkQueue>,
    pub output_callback: Arc<dyn Fn(AudioData) + Send + Sync>,
    pub error_callback: Arc<dyn Fn(Exception) + Send + Sync>,
    pub codec_impl: Arc<Mutex<Option<ffmpeg_next::decoder::Audio>>>,
}

pub struct AudioEncodeMessage {
    pub data: String,
    pub work_queue: Arc<WorkQueue>,
}

pub struct AudioFlushMessage {
    pub work_queue: Arc<WorkQueue>,
    pub output_callback: Arc<dyn Fn(AudioData) + Send + Sync>,
    pub error_callback: Arc<dyn Fn(Exception) + Send + Sync>,
    pub codec_impl: Arc<Mutex<Option<ffmpeg_next::decoder::Audio>>>,
}

impl ControlMessageTrait for AudioConfigMessage {
    fn process(&mut self) -> Outcome {
        let config = self.config.clone();
        let work_queue = self.work_queue.clone();
        let error_callback = self.error_callback.clone();
        let codec_impl = self.codec_impl.clone();

        work_queue.enqueue(Box::new(move || {
            if ffmpeg_next::init().is_err() {
                error_callback(Exception::InternalError);
                return;
            }

            let codec = match ffmpeg_next::codec::decoder::find_by_name(&config.codec) {
                Some(codec) => codec,
                None => {
                    error_callback(Exception::NotSupportedError);
                    return;
                }
            };

            let context = ffmpeg_next::codec::Context::new_with_codec(codec);
            let decoder = match context.decoder().audio() {
                Ok(decoder) => decoder,
                Err(e) => {
                    eprintln!("{e}");
                    error_callback(Exception::NotSupportedError);
                    return;
                }
            };

            {
                let mut dec_lock = match codec_impl.lock() {
                    Ok(lock) => lock,
                    Err(_) => {
                        error_callback(Exception::InternalError);
                        return;
                    }
                };
                *dec_lock = Some(decoder);
            }
        }));
        Outcome::Processed
    }
}

impl ControlMessageTrait for AudioDecodeMessage {
    fn process(&mut self) -> Outcome {
        let chunk = self.chunk.clone();
        let work_queue = self.work_queue.clone();
        let output_callback = self.output_callback.clone();
        let error_callback = self.error_callback.clone();
        let codec_impl = self.codec_impl.clone();

        work_queue.enqueue(Box::new(move || {
            let mut decoder_lock = codec_impl.lock().unwrap();
            if let Some(decoder) = decoder_lock.as_mut() {
                let mut packet = ffmpeg_next::Packet::new(chunk.data.len());
                let chunk = chunk.clone();
                if let Some(data) = packet.data_mut() {
                    data.copy_from_slice(&chunk.data);
                } else {
                    eprintln!("Warn: packet.data_mut() is None ");
                }

                if let Err(e) = decoder.send_packet(&packet) {
                    eprintln!("Error sending packet: {:?}", e);
                    error_callback(Exception::DecodeError);
                    return;
                }
                decode_audio_frames(decoder, chunk.timestamp as f64, output_callback);
            } else {
                error_callback(Exception::InvalidStateError);
            }
        }));
        Outcome::Processed
    }
}

impl ControlMessageTrait for AudioEncodeMessage {
    fn process(&mut self) -> Outcome {
        let data = self.data.clone();
        let work_queue = self.work_queue.clone();
        work_queue.enqueue(Box::new(move || todo!()));
        Outcome::Processed
    }
}

impl ControlMessageTrait for AudioFlushMessage {
    fn process(&mut self) -> Outcome {
        let mut dec_lock = self.codec_impl.lock().unwrap();
        if let Some(decoder) = dec_lock.as_mut() {
            decoder
                .send_eof()
                .map_err(|_| Exception::DecodeError)
                .unwrap();
        }
        // TODO: Draining not implemented yet. Need to figure out how to get timestamps for
        // these remaining frames
        let mut decoder_lock = self.codec_impl.lock().unwrap();
        // if let Some(decoder) = decoder_lock.as_mut() {
        // decode_audio_frames(decoder, timestamp, self.output_callback);
        // }
        Outcome::Processed
    }
}

fn decode_audio_frames(
    decoder: &mut ffmpeg_next::decoder::Audio,
    timestamp: f64,
    output_callback: Arc<dyn Fn(AudioData) + Send + Sync>,
) {
    let mut frame = ffmpeg_next::frame::Audio::empty();
    while decoder.receive_frame(&mut frame).is_ok() {
        let sample_format = frame.format();
        let target_format =
            ffmpeg_next::format::Sample::F32(ffmpeg_next::format::sample::Type::Planar);
        let converted_frame = if sample_format != target_format {
            let mut resampler = ffmpeg_next::software::resampling::Context::get(
                sample_format,
                frame.channel_layout(),
                frame.rate(),
                target_format,
                frame.channel_layout(),
                frame.rate(),
            )
            .unwrap();
            let mut cf = ffmpeg_next::frame::Audio::empty();
            resampler.run(&frame, &mut cf).unwrap();
            cf
        } else {
            frame.clone()
        };

        let num_frames = converted_frame.samples() as u16;
        let sample_rate = converted_frame.rate();
        let channels = converted_frame.channels();
        let bytes_per_sample = std::mem::size_of::<f32>() as u16;
        let mut audio_buffer =
            Vec::with_capacity((num_frames * channels * bytes_per_sample).into());

        for ch in 0..channels {
            let plane = converted_frame.data(ch.into());
            audio_buffer.extend_from_slice(plane);
        }
        let audio_data = AudioData::new(
            "f32-planar".to_string(),
            sample_rate as f64,
            channels as u32,
            num_frames as u32,
            timestamp,
            audio_buffer,
        );
        output_callback(audio_data);
    }
}
