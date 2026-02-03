#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_praat_identity() {
        unsafe {
            let version = Melder_appVersion();
            println!("Praat Melder_appVersion returned: {}", version);
            assert!(version >= 0);
        }
    }

    #[test]
    fn test_melder_play_exists() {
        unsafe {
            let func_ptr = Melder_play as *const ();
            assert!(!func_ptr.is_null());
            println!("Linker successfully resolved Melder_play at {:?}", func_ptr);
        }
    }
}