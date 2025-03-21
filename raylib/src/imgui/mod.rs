use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    ptr::{null, null_mut},
    sync::OnceLock,
};

use imgui::{Context, Ui, UiBuffer};
use imgui_sys::{
    igGetCurrentContext, igGetIO, igStyleColorsDark, igStyleColorsLight,
    ImFontAtlas_AddFontDefault, ImGuiStyle,
};

use super::drawing::RaylibDrawHandle;

static mut CONTEXT: OnceLock<Context> = OnceLock::new();

fn context() -> &'static Context {
    unsafe { CONTEXT.get_or_init(|| imgui::Context::create()) }
}

/**
   Recreation of rlImGuiSetup because imgui-rs **really** wants you to use it's own function for the imgui context.

   We currently use a version of rlImGui where this is actually possible, and hopefully you aren't reading this in a point of time where it's not.
*/
pub(crate) unsafe fn init_imgui_context(dark: bool) {
    context();
    if dark {
        igStyleColorsDark(null_mut() as *mut ImGuiStyle);
    } else {
        igStyleColorsLight(null_mut() as *mut ImGuiStyle);
    }

    let io = igGetIO().as_ref().unwrap();
    ImFontAtlas_AddFontDefault(io.Fonts, null());

    raylib_sys::rlImGuiEndInitImGui();
}

/// The interface for rlImGui.
///
/// This can be constructed via [RaylibDrawHandle::start_imgui] or [RaylibDrawHandle::begin_imgui]
pub struct RayImGUIHandle(Ui);

impl RayImGUIHandle {
    fn new() -> Option<Self> {
        unsafe {
            // Correct an assertion error that sometimes happens with DeltaTime.
            // We have to step into unsafe code to set the actual values that ImGui looks at
            // and not imgui-rs's custom stuff.
            if let Some(_ctx) = igGetCurrentContext().as_mut() {
                if let Some(io) = igGetIO().as_mut() {
                    if io.DeltaTime <= 0.0 {
                        io.DeltaTime = 0.01;
                        // Yes we have to return None at this point, setting DeltaTime did not actually
                        // work. Yes this sucks.
                        return None;
                    }
                }
            }
            raylib_sys::rlImGuiBegin();
        };

        // We don't actually have a ui buffer from imgui-rs to store here, but what we do have
        // is the fact that we can just make our own ui buffer with unsafe code, and it works
        // because the struct only has one value and is most likely going to work unless they add
        // something else.
        Some(Self(unsafe {
            std::mem::transmute(UnsafeCell::new(UiBuffer::new(1024)))
        }))
    }
}

impl Drop for RayImGUIHandle {
    fn drop(&mut self) {
        unsafe {
            raylib_sys::rlImGuiEnd();
        }
    }
}

impl Deref for RayImGUIHandle {
    type Target = Ui;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RayImGUIHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait RayImGUITrait {
    /// Setup ImGUI to start drawing. Prefer using the closure version, [RaylibHandle::start_imgui]. This version returns a handle that calls [raylib_sys::rlImGuiEnd] at the end of the scope and is provided as a fallback incase you run into issues with closures(such as lifetime or performance reasons)
    ///
    /// Returns None in the specific but also common case that the delta time is negative on any frame other then 0.
    fn begin_imgui(&self) -> Option<RayImGUIHandle> {
        return RayImGUIHandle::new();
    }

    /// Setup ImGUI then call the closure with the appropriate handle.
    ///
    /// Fails silently if the delta time is negative on any frame other then 0.
    fn draw_imgui(&self, f: impl Fn(&mut Ui)) {
        if let Some(mut new_frame) = RayImGUIHandle::new() {
            f(&mut new_frame);
        }
    }
}

impl RayImGUITrait for RaylibDrawHandle<'_> {}

/// The theme chosen for imgui integeration.
#[cfg(feature = "imgui")]
#[derive(Debug, PartialEq)]
pub enum ImGuiTheme {
    Light,
    Dark,
}

#[cfg(feature = "imgui")]
impl Default for ImGuiTheme {
    fn default() -> Self {
        Self::Dark
    }
}
