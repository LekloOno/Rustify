use std::sync::mpsc::{self, Receiver, Sender};
use std::fs::File;
use std::thread;
use cpal::Host;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::{FormatOptions, Packet};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::sample;
use std::sync::mpsc::sync_channel;

pub enum AudioCommand {
    Play(String),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}

pub struct AudioEngine { 
    command_tx: Sender<AudioCommand>,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel();
        
        thread::spawn(move || {
            Self::audio_thread_loop(command_rx);
        });

        AudioEngine { command_tx }
    }

    pub fn send_command(&self, cmd: AudioCommand) {
        if let Err(e) = self.command_tx.send(cmd) {
            eprintln!("erreur {}", e);
        }
    }

    fn audio_thread_loop(rx: Receiver<AudioCommand>) {

        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");
        println!("Using output device: {}", device.name().unwrap_or("Unknown".to_string()));

        let config = device.default_output_config().expect("Failed to get default output config");
        println!("Default output config: {:?}", config);

        for cmd in rx {
            match cmd {
                AudioCommand::Play(file) => {
                    Self::play_file(&file);
                }
                AudioCommand::Pause => {
                    println!("Pausing audio");
                }
                AudioCommand::Resume => {
                    println!("Resuming audio");
                }
                AudioCommand::Stop => {
                    println!("Stopping audio");
                } 
                AudioCommand::SetVolume(vol) => {
                    println!("Setting volume to: {}", (vol * 100.0) as i32);
                }
            }
        }
    }

    fn play_file(path: &str) {
        let file= match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open file {}: {}", path, e);
                return;
            }
        };

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = std::path::Path::new(path).extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = Default::default();

        let probed = match symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts) {
            Ok(probed) => probed,
            Err(e) => {
                eprintln!("Failed to probe media source: {}", e);
                return;
            }
        };

        let mut format = probed.format;

        let track = format.default_track().expect("No default track found");

        let track_id = track.id;

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decoder_opts)
            .unwrap();

        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");
        let config = device.default_output_config().expect("Failed to get default output config");

        let (sample_tx, sample_rx) = sync_channel::<f32>(44100 * 2);

        let stream = device.build_output_stream(
            &config.into(), 
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = sample_rx.recv().unwrap_or(0.0);
                }
            }, 
            |err| eprintln!("Error occurred: {}", err),
            None,
            ).unwrap();

        stream.play().unwrap();

        let mut sample_buf: Option<SampleBuffer<f32>> = None;

        loop {
            let mut packet = format.next_packet().unwrap();

            if packet.track_id() != track_id {
                continue;
            }

            match decoder.decode(&packet) {
                Ok(audio_buf) => {
                    if sample_buf.is_none() {
                        let spec = *audio_buf.spec();
                        let duration = audio_buf.capacity() as u64;
                        sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                    }

                    if let Some(buf) = &mut sample_buf {
                        buf.copy_interleaved_ref(audio_buf);
                        for sample in buf.samples() {
                            sample_tx.send(*sample).unwrap();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error decoding packet: {}", e);
                }
            }
        }

    }
}