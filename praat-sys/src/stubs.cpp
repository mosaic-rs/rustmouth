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
#include <string.h> 
#include "Sound.h"

extern "C" {
    void Graphics_ps_init() {}
    void Graphics_x11_init() {}
    void Praat_gui_init() {}
    
    int Melder_isGuiAvailable() {
        return 0; 
    }

    // These may move around but they were used to really establish a link 

    structSound* rustmouth_Sound_create(long long channels, double xmin, double xmax, long long nx, double dx, double x1) {
        autoSound sound = Sound_create(channels, xmin, xmax, nx, dx, x1);
        structSound* rawPtr = *(structSound**)&sound;
        memset(&sound, 0, sizeof(sound));
        return rawPtr;
    }

    double rustmouth_Sound_getSample(structSound* sound, long long channel, long long sample_index) {
        if (!sound) return 0.0;
        return sound -> z [channel] [sample_index];
    }

    void rustmouth_Sound_setSample(structSound* sound, long long channel, long long sample_index, double value) {
        if (sound) {
            sound -> z [channel] [sample_index] = value;
        }
    }
    
}