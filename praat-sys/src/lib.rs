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

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(doc))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(doc)]
pub const DOCS_ONLY_PRAAT_SYS_IS_LOADED: bool = true;

/*#[cfg(test)]
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
}*/