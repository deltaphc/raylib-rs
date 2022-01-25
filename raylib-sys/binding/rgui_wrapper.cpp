#include "../raylib/src/raylib.h"
#define RAYGUI_IMPLEMENTATION
#define RAYGUI_SUPPORT_ICONS
#define RLGL_IMPLEMENTATION
#define RLGL_SUPPORT_TRACELOG
// // Support TRACELOG macros
// #if !defined(TRACELOG)
//     #define TRACELOG(level, ...) (void)0
//     #define TRACELOGD(...) (void)0
// #endif
// #include "rlgl.h" // Don't include rlgl since it's in raylib
#include "raygui.h"
#undef RAYGUI_IMPLEMENTATION