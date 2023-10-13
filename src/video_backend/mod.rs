use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
pub enum VideoBackendType {
    FFMPEG,
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

pub struct FFMPEGBackend<'a> {
    pub frame_receiver: mpsc::Receiver<FrameMessage<'a>>,
}

pub enum FrameMessage<'a> {
    Frame(&'a [u8]),
    End,
}

impl<'a> VideoWriter for FFMPEGBackend<'a> {
    fn write_frame(&mut self, frame_bytes: &[u8]) {}
}

impl<'a> FFMPEGBackend<'a> {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        FFMPEGBackend { frame_receiver: rx }
    }
    pub fn start_video_backend(&self) {
        use std::process::Command;
        let mut c = Command::new("ffmpeg")
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
        use std::io::Write;
        loop {
            match self.frame_receiver.recv() {
                Ok(data) => match data {
                    FrameMessage::Frame(frame_bytes) => {
                        stdin.write_all(frame_bytes);
                    }
                    FrameMessage::End => {
                        break;
                    }
                },
                Err(e) => {
                    break;
                }
            }
        }
    }
}

trait VideoWriter {
    fn write_frame(&mut self, frame_bytes: &[u8]);
}
