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
    hevc_vaapi,
}

impl FFMPEGEncoder {
    fn get_encoder_name(&self) -> &'static str {
        match self {
            Self::libx264 => "libx264",
            Self::libx265 => "libx265",
            Self::hevc_nvenc => "hevc_nvenc",
            Self::hevc_vaapi => "hevc_vaapi",
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
struct FFMPEGOutputOptionBuilder {
    high_quality: bool,
    encoder: FFMPEGEncoder,
}

impl FFMPEGOutputOptionBuilder {
    fn build_option(&self, args: &mut Vec<String>) {
        args.push("-an".to_string());
        args.extend([
            "-vcodec".to_string(),
            self.encoder.get_encoder_name().to_string(),
        ]);

        self.specify_hwaccel_device_option(args);
        self.specify_quality_option(args);
    }
    fn specify_hwaccel_device_option(&self, args: &mut Vec<String>) {
        match self.encoder {
            FFMPEGEncoder::hevc_vaapi => {
                args.extend([
                    "-vaapi_device".to_string(),
                    "/dev/dri/renderD128".to_string(),
                    "-vf".to_string(),
                    "format=nv12,hwupload".to_string(),
                ]);
            }
            _ => {}
        }
    }

    fn specify_quality_option(&self, args: &mut Vec<String>) {
        let mut quality_options = match self.encoder {
            FFMPEGEncoder::hevc_vaapi => {
                if self.high_quality {
                    vec!["-compression_level", "29", "-qp", "1"]
                } else {
                    vec!["-compression_level", "0", "-qp", "52"]
                }
            }
            FFMPEGEncoder::hevc_nvenc => {
                if self.high_quality {
                    vec!["-preset", "p7"]
                } else {
                    vec!["-preset", "p1"]
                }
            }
            _ => {
                if self.high_quality {
                    vec!["-preset", "veryslow"]
                } else {
                    vec!["-preset", "ultrafast"]
                }
            }
        };
        //vaapi only support "vaapi" pix_fmt
        if !matches!(self.encoder, FFMPEGEncoder::hevc_vaapi) {
            if self.high_quality {
                quality_options.extend(["-pix_fmt", "yuv444p"]);
            } else {
                quality_options.extend(["-pix_fmt", "yuv420p"]);
            }
        }
        args.extend(quality_options.iter().map(|x| x.to_string()))
    }
}

impl FFMPEGBackend {
    pub fn new(
        video_config: &VideoConfig,
        encoder_config: FFMPEGEncoder,
        high_profile: bool,
    ) -> Self {
        let encoder_name = encoder_config.get_encoder_name();

        let mut args = vec![
            "-y".to_string(),
            "-f".to_string(),
            "rawvideo".to_string(),
            "-pix_fmt".to_string(),
            format!("{}", video_config.color_order).to_string(),
            "-s".to_string(),
            format!(
                "{}x{}",
                video_config.output_width, video_config.output_height
            ),
            "-r".to_string(),
            format!("{}", video_config.framerate),
            "-i".to_string(),
            "-".to_string(),
        ];
        let encoder_option_builder = FFMPEGOutputOptionBuilder {
            high_quality: high_profile,
            encoder: encoder_config,
        };
        
        encoder_option_builder.build_option(&mut args);

        args.push(video_config.filename.to_string());

        let mut c = std::process::Command::new("ffmpeg")
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
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
