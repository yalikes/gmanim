cfg_if::cfg_if! {
    if #[cfg(feature = "gmfloat_f16")]{
        pub const PI: f16 = std::f16::consts::PI;
    }else if #[cfg(feature = "gmfloat_f32")]{
        pub const PI: f32 = std::f32::consts::PI;
    }else if #[cfg(feature = "gmfloat_f64")]{
        pub const PI: f64 = std::f64::consts::PI;
    }else{
        pub const PI: f32 = std::f32::consts::PI;
    }
}
