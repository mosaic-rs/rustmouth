/*
This file is part of RUSTMOUTH.

RUSTMOUTH is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free 
Software Foundation, either version 3 of the License, or any later version.

RUSTMOUTH is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; 
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR 
PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
RUSTMOUTH. If not, see <https://www.gnu.org/licenses/>.
*/

extern "C" {}

use praat_sys;

pub fn get_praat_version() -> f64 {
    unsafe {
        praat_sys::Melder_appVersion() as f64
    }
}


pub struct Sound {
    ptr: *mut praat_sys::structSound,
}

impl Sound {
    pub fn new(channels: i64, start: f64, end: f64, samples: i64, dx: f64, x1: f64) -> Self {
        unsafe {
            let ptr = praat_sys::rustmouth_Sound_create(channels, start, end, samples, dx, x1);
            if ptr.is_null() { panic!("Failed to create Sound"); }
            Sound { ptr }
        }
    }

    pub fn duration(&self) -> f64 {
        unsafe {
            let sampled = self.ptr as *mut praat_sys::structSampled;
            (*sampled)._base.xmax - (*sampled)._base.xmin
        }
    }

    pub fn get_sample(&self, channel: i64, index: i64) -> f64 {
        unsafe {
            praat_sys::rustmouth_Sound_getSample(self.ptr, channel, index)
        }
    }

    pub fn set_sample(&mut self, channel: i64, index: i64, value: f64) {
        unsafe {
            praat_sys::rustmouth_Sound_setSample(self.ptr, channel, index, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_call() {
        let version = get_praat_version();
        println!("Connected to Praat Engine. Version: {}", version);
        assert!(version >= 0.0);
    }

    #[test]
    fn test_sound_creation() {
        unsafe {
            praat_sys::Melder_init(); 
            
            // praat_sys::structThing_staticInit();
            // praat_sys::structData_staticInit();
            // praat_sys::structVector_staticInit();
            // praat_sys::structSampled_staticInit();
            // praat_sys::structSound_staticInit();
        }
    
        let sound = Sound::new(1, 0.0, 1.0, 44100, 1.0/44100.0, 0.5/44100.0);
        
        let dur = sound.duration();
        println!("Created sound with duration: {} seconds", dur);
        
        assert!((dur - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_get_samples() {
        unsafe { praat_sys::Melder_init(); }
        let sound = Sound::new(1, 0.0, 1.0, 44100, 1.0/44100.0, 0.5/44100.0);

        let s1 = sound.get_sample(1, 1);
        let s100 = sound.get_sample(1, 100);

        println!("Sample 1: {}, Sample 100: {}", s1, s100);

        assert_eq!(s1, 0.0);
        assert_eq!(s100, 0.0);
    }

    #[test]
    fn test_generate_tone() {
        unsafe { praat_sys::Melder_init(); }
        let mut sound = Sound::new(1, 0.0, 1.0, 44100, 1.0/44100.0, 0.5/44100.0);
        
        let freq = 440.0;
        let sample_rate = 44100.0;
    
        for i in 1..=100 {
            let t = (i as f64 - 1.0) / sample_rate;
            let val = (2.0 * std::f64::consts::PI * freq * t).sin();
            
            sound.set_sample(1, i, val);
        }
    
        let s10 = sound.get_sample(1, 10);
        println!("Sample 10 amplitude: {}", s10);
        assert!(s10 != 0.0);
    }
}

#[no_mangle]
pub extern "C" fn gsl_set_error_handler_off() {
}

#[no_mangle]
pub extern "C" fn dlamch_(cmach: *const std::os::raw::c_char) -> f64 {
    0.0
}