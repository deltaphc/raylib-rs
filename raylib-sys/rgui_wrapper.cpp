#include "raylib.h"
#define RLIGHTS_IMPLEMENTATION
#define RICONS_IMPLEMENTATION
#define RAYGUI_IMPLEMENTATION
#define RAYGUI_SUPPORT_ICONS
#define RLGL_IMPLEMENTATION
#define RLGL_SUPPORT_TRACELOG
// #include "rlgl.h" // Don't include rlgl since it's in raylib
#include "raygui.h"
#undef RAYGUI_IMPLEMENTATION
#include "rlights.h"