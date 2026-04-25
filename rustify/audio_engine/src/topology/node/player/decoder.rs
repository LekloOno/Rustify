/// Handles audio decoding using the symphonia crate.

use std::fs::File;
use std::sync::mpsc::SyncSender;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Represents the audio decoder.
/// It holds the format reader, decoder, and sample buffer.
pub struct AudioDecoder {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    sample_buf: Option<SampleBuffer<f32>>,
}


impl AudioDecoder {

    /// Open an audio file and initialize the decoder.
    /// 
    /// Panic if the file cannot be opened, probed, or decoded.
    /// 
    /// # Example
    /// ```rust, no_run
    /// use audio_engine::decoder::AudioDecoder;
    /// 
    /// // Initialize the audio decoder with a file path
    /// let decoder = AudioDecoder::new("test.mp3");
    /// ```
    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open audio file");
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = std::path::Path::new(path).extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .expect("Failed to probe media source");

        let format = probed.format;
        let track = format.default_track().expect("No default track found");
        let track_id = track.id;

        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decoder_opts)
            .expect("Failed to create decoder");

        AudioDecoder {
            format,
            decoder,
            track_id,
            sample_buf: None,
        }
    }

    /// Decode the next audio packet and send the samples to the provided channel.
    /// 
    /// Returns Ok(false) if the end of the file is reached, Ok(true) if decoding is successful, or Err with an error message if an error occurs.
    pub fn decode_next(&mut self, sample_tx: &mut SyncSender<f32>, volume: f32) -> Result<bool, String> {
        let packet = match self.format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(_)) => {
                return Ok(false);
            }
            Err(e) => {
                return Err(format!("Error reading packet: {}", e));
            }
        };

        if packet.track_id() != self.track_id {
            return Ok(true);
        }

        match self.decoder.decode(&packet) {
            Ok(audio_buf) => {
                if self.sample_buf.is_none() {
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;
                    self.sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }

                if let Some(buf) = &mut self.sample_buf {
                    buf.copy_interleaved_ref(audio_buf);
                    for sample in buf.samples() {
                        let _ = sample_tx.send(sample * volume);
                    }
                }
                Ok(true)
            }
            Err(e) => {
                eprintln!("Error decoding packet: {}", e);
                Ok(true)
            }
        }
    }
}