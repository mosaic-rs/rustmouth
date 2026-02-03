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

#ifndef WRAPPER_HPP
#define WRAPPER_HPP

#ifndef macintosh
#define macintosh 1
#endif

#include "melder/Melder.h"
#include "sys/Thing.h"
#include "fon/Sound.h"
#include "LPC/LPC.h"

#include "Melder.h"
#include "Thing.h"
#include "Data.h"
#include "Sampled.h"
#include "Sound.h"
#include "LPC.h"

extern "C" {
    void structThing_staticInit();
    void structData_staticInit();
    void structVector_staticInit();
    void structSampled_staticInit();
    void structSound_staticInit();
}

#endif