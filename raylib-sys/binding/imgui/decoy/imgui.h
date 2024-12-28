/* Decoy imgui.h because bindgen can't actually (and shouldn't actually) bind a
 * C++ file*/

#ifndef IMGUI_DECOY
#define IMGUI_DECOY

#define IMGUI_IMPL_API
typedef struct ImVec2 ImVec2;
typedef struct ImGuiContext ImGuiContext;

#endif