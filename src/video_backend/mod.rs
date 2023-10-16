use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
pub enum VideoBackendType {
    FFMPEG(FFMPEGBackend),
    BgraRAW(BgraRAWBackend),
    Gstreamer,
}

pub struct VideoBackend {
    pub backend_type: VideoBackendType,
    pub video_config: VideoConfig,
}

pub struct VideoConfig {
    pub filename: String,
    pub framerate: u32,
    pub output_width: u32,
    pub output_height: u32,
}

pub struct FFMPEGBackend {
    child: std::process::Child,
    stdin: std::process::ChildStdin,
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
}

impl FFMPEGBackend {
    pub fn new() -> Self {
        let mut c = std::process::Command::new("ffmpeg")
            .args([
                "-y",
                "-f",
                "rawvideo",
                "-pix_fmt",
                "bgra",
                "-s",
                "1920x1080",
                "-r",
                "60",
                "-i",
                "-",
                "-an",
                "-vcodec",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "output.mp4",
            ])
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
    pub fn new() -> Self {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open("/tmp/output.bgra")
            .unwrap();
        Self { file }
    }
}
