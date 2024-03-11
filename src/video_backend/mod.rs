use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
pub enum VideoBackendType {
    FFMPEG(FFMPEGBackend),
    BgraRAW(BgraRAWBackend),
    Gstreamer,
}

pub struct VideoBackend {
    pub backend_type: VideoBackendType,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorOrder {
    Bgra,
    Rgba,
}

impl Display for ColorOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorOrder::Bgra => {
                write!(f, "bgra")
            }
            ColorOrder::Rgba => {
                write!(f, "rgba")
            }
        }
    }
}

pub struct VideoConfig {
    pub filename: String,
    pub framerate: u32,
    pub output_width: u32,
    pub output_height: u32,
    pub color_order: ColorOrder,
}

pub struct FFMPEGBackend {
    child: std::process::Child,
    stdin: std::process::ChildStdin,
}

#[allow(non_camel_case_types)]
pub enum FFMPEGEncoder {
    libx264,
    libx265,
    hevc_nvenc,
}

impl FFMPEGEncoder {
    fn get_encoder_name(&self) -> &'static str {
        match self {
            Self::libx264 => "libx264",
            Self::libx265 => "libx265",
            Self::hevc_nvenc => "hevc_nvenc",
        }
    }
    fn get_highest_preset_name(&self) -> &'static str {
        match self {
            Self::libx264 => "veryslow",
            Self::libx265 => "veryslow",
            Self::hevc_nvenc => "p7",
        }
    }
    fn get_fastest_preset_name(&self) -> &'static str {
        match self {
            Self::libx264 => "ultrafast",
            Self::libx265 => "ultrafast",
            Self::hevc_nvenc => "p1",
        }
    }
    fn get_highest_pixel_format(&self) -> &'static str {
        match self {
            Self::libx264 => "yuv444p",
            Self::libx265 => "yuv444p",
            Self::hevc_nvenc => "yuv444p",
        }
    }
    fn get_fastest_pixel_format(&self) -> &'static str {
        match self {
            Self::libx264 => "yuv420p",
            Self::libx265 => "yuv420p",
            Self::hevc_nvenc => "yuv420p",
        }
    }
}
pub struct FFMPEGConfig {
    pub ffmpeg_encoder: FFMPEGEncoder,
}

pub struct BgraRAWBackend {
    file: std::fs::File,
}

pub enum FrameMessage {
    Frame,
    End,
}

pub enum FrameDoneMessage {
    Ok,
    Err,
}

#[derive(Clone, Copy)]
pub enum VideoBackendState {
    Running,
    Sleeping,
}

impl VideoBackend {
    pub fn write_frame(&mut self, frame_data: &[u8]) {
        match &mut self.backend_type {
            VideoBackendType::FFMPEG(f) => {
                use std::io::Write;
                f.stdin.write_all(frame_data);
            }
            VideoBackendType::BgraRAW(f) => {
                use std::io::Write;
                f.file.write_all(frame_data);
            }
            _ => {}
        }
    }
    pub fn write_frame_background(
        &mut self,
        rx: Receiver<FrameMessage>,
        state: Arc<Mutex<VideoBackendState>>,
        queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    ) {
        loop {
            let now = std::time::Instant::now();
            let data;
            {
                let mut queue_guard = queue.lock().unwrap();
                data = queue_guard.pop_front();
            }
            if data.is_none() {
                {
                    let mut state_guard = state.lock().unwrap();
                    *state_guard = VideoBackendState::Sleeping;
                }
                println!("sleeping!");
                match rx.recv() {
                    Ok(f) => match f {
                        FrameMessage::Frame => {}
                        FrameMessage::End => {
                            break;
                        }
                    },
                    Err(e) => {
                        //no more frame
                        break;
                    }
                }
            } else {
                self.write_frame(&data.unwrap());
            }
            println!("write takes: {:?}", now.elapsed());
        }
    }
}

impl FFMPEGBackend {
    pub fn new(
        video_config: &VideoConfig,
        encoder_config: FFMPEGEncoder,
        high_profile: bool,
    ) -> Self {
        let encoder_name = encoder_config.get_encoder_name();
        let preset = if high_profile {
            encoder_config.get_highest_preset_name()
        } else {
            encoder_config.get_fastest_preset_name()
        };
        let out_pixel_format = if high_profile {
            encoder_config.get_highest_pixel_format()
        } else {
            encoder_config.get_fastest_pixel_format()
        };
        let mut c = std::process::Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "rawvideo",
                "-pix_fmt",
                &format!("{}", video_config.color_order),
                "-s",
                &format!(
                    "{}x{}",
                    video_config.output_width, video_config.output_height
                ),
                "-r",
                &format!("{}", video_config.framerate),
                "-i",
                "-",
                "-an",
                "-vcodec",
                &format!("{}", encoder_name),
                "-preset",
                &format!("{}", preset),
                "-pix_fmt",
                &format!("{}", out_pixel_format),
                &format!("{}", video_config.filename),
            ])
            .stdin(std::process::Stdio::piped())
            // .stdout(std::process::Stdio::null())
            // .stderr(std::process::Stdio::null())
            .spawn()
            .expect("failed to spawn child process");
        let mut stdin = c.stdin.take().expect("failed to open stdin");
        Self {
            child: c,
            stdin: stdin,
        }
    }
}

impl BgraRAWBackend {
    pub fn new(video_config: &VideoConfig) -> Self {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&format!("{}", video_config.filename))
            .unwrap();
        Self { file }
    }
}
