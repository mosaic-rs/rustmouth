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

#include <stdio.h>

// this hopefully cancels the praat GUI and means I don't have to manually comment out
// headers in the praat source code

extern "C" {
    void Graphics_ps_init() {}
    void Graphics_x11_init() {}
    void Praat_gui_init() {}
    
    int Melder_isGuiAvailable() {
        return 0; 
    }
}