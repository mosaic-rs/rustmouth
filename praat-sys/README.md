# PRAAT-SYS
Praat-sys is a low level linker which compiles a headless Praat. 

## Warning!
Though v0.1.0 is significantly more stable than 0.0.1, compiling a (more) stable headless Praat, it is still experimental. 

## Supported OS:
* Mac OS 11.0+ on x86 architecture (M1/M2/M3/M4/M5)
* (In Progress) Windows - Groundwork is layed out
* (In Progress) Linux - Groundwork is layed out

## Things To Know
This comes with a copy of the praat source code and certain macintosh headers in praat-src/sys/Gui.h

```c
#elif defined (macintosh)
    //#include "macport_on.h"
    //#include <Cocoa/Cocoa.h>
    //#include "macport_off.h"
#elif defined (macintosh)

#ifndef RUSTMOUTH_HEADLESS 
    // TXNObject d_macMlteObject;
    // TXNFrameID d_macMlteFrameId;
#endif
```

## License
praat-sys is part of RustMouth, falling under:
Licensed under the GNU General Public License v3.0 (GPL-3.0)