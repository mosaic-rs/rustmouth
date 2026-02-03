#ifndef WRAPPER_HPP
#define WRAPPER_HPP

// Platform defines for bindgen
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

// Explicitly tell bindgen these exist if they are being tricky
extern "C" {
    void structThing_staticInit();
    void structData_staticInit();
    void structVector_staticInit();
    void structSampled_staticInit();
    void structSound_staticInit();
}

#endif