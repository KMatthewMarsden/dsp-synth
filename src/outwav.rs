use std::{
    f32::consts::PI,
};

pub fn get_oscillator(freq: f32, sampleRate: f32) -> f32 {
    let increment = (2.0 * PI * freq) / sampleRate;
    return increment;
}

