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

#ifndef RUSTMOUTH_CONFIG_H
#define RUSTMOUTH_CONFIG_H

#ifdef macintosh
    #undef macintosh
    #define macintosh 1
#endif


#ifdef macintosh
    struct OpaqueTXNObject;
    typedef struct OpaqueTXNObject* TXNObject;

    typedef unsigned int TXNFrameID; 
#endif

#define NO_GUI 1
#define NO_GRAPHICS 1
#define NO_AUDIO 1 
#define MAIN_CONSOLE 1
#define RUSTMOUTH_HEADLESS 1

#ifdef macintosh
    #define OBJC_SILENCE_DEPRECATION 1
#endif

#endif