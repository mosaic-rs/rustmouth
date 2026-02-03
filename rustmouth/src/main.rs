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

pub use praat_sys;

fn main() {
    println!("Initializing Rustmouth...");

    unsafe {
        // Most Praat-sys builds need the Melder library started 
        // to handle strings and memory properly.
        // Replace 'Melder_init' with the specific init function in your headers.
        // praat_sys::Melder_init(); 
        
        println!("Praat environment linked successfully");
    }
    
    println!("FFI didn't segfault, yippeee");
}