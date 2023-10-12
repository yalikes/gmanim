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

pub struct FFMPEGBackend {}


