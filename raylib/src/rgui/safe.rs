use crate::core::color::Color;
use crate::core::drawing::RaylibDraw;
use crate::core::math::{Rectangle, Vector2};
use crate::core::text::WeakFont;
use crate::core::RaylibHandle;
use crate::ffi;

use std::ffi::CStr;

pub trait IntoCStr {
    fn as_cstr_ptr(&self) -> *const std::os::raw::c_char;
}

impl IntoCStr for dyn AsRef<str> {
    fn as_cstr_ptr(&self) -> *const std::os::raw::c_char {
        std::ffi::CString::new(self.as_ref())
            .unwrap()
            .as_c_str()
            .as_ptr()
    }
}

impl IntoCStr for dyn AsRef<CStr> {
    fn as_cstr_ptr(&self) -> *const std::os::raw::c_char {
        self.as_ref().as_ptr()
    }
}

impl IntoCStr for Option<&CStr> {
    fn as_cstr_ptr(&self) -> *const std::os::raw::c_char {
        self.map(CStr::as_ptr).unwrap_or(std::ptr::null())
    }
}

/// Global gui modification functions
impl RaylibHandle {
    /// Enable gui controls (global state)
    #[inline]
    pub fn gui_enable(&mut self) {
        unsafe { ffi::GuiEnable() }
    }
    /// Disable gui controls (global state)
    #[inline]
    pub fn gui_disable(&mut self) {
        unsafe { ffi::GuiDisable() }
    }
    /// Lock gui controls (global state)
    #[inline]
    pub fn gui_lock(&mut self) {
        unsafe { ffi::GuiLock() }
    }
    /// Unlock gui controls (global state)
    #[inline]
    pub fn gui_unlock(&mut self) {
        unsafe { ffi::GuiUnlock() }
    }
    // Set gui controls alpha (global state), alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn gui_fade(&mut self, color: Color, alpha: f32) -> Color {
        unsafe { ffi::Fade(color.into(), alpha).into() }
    }
    /// Set gui state (global state)
    #[inline]
    pub fn gui_set_state(&mut self, state: crate::consts::GuiState) {
        unsafe { ffi::GuiSetState(state as i32) }
    }
    /// Get gui state (global state)
    #[inline]
    pub fn gui_get_state(&mut self) -> crate::consts::GuiState {
        unsafe { std::mem::transmute(ffi::GuiGetState()) }
    }
    /// Set gui custom font (global state)
    #[inline]
    pub fn gui_set_font(&mut self, font: impl AsRef<ffi::Font>) {
        unsafe { ffi::GuiSetFont(*font.as_ref()) }
    }
    /// Get gui custom font (global state)
    #[inline]
    pub fn gui_get_font(&mut self) -> WeakFont {
        unsafe { WeakFont(ffi::GuiGetFont()) }
    }
    /// Set one style property
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    pub fn gui_set_style(&mut self, control: crate::consts::GuiControl, property: i32, value: i32) {
        unsafe { ffi::GuiSetStyle(control as i32, property as i32, value) }
    }

    /// Get one style property
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    pub fn gui_get_style(&mut self, control: crate::consts::GuiControl, property: i32) -> i32 {
        unsafe { ffi::GuiGetStyle(control as i32, property as i32) }
    }
    /// Load style file (.rgs)
    #[inline]
    pub fn gui_load_style(&mut self, filename: impl IntoCStr) {
        unsafe { ffi::GuiLoadStyle(filename.as_cstr_ptr()) }
    }
    /// Load style default over global style
    #[inline]
    pub fn gui_load_style_default(&mut self) {
        unsafe { ffi::GuiLoadStyleDefault() }
    }
}

impl<D: RaylibDraw> RaylibDrawGui for D {}

pub trait RaylibDrawGui {
    /// Enable gui controls (global state)
    #[inline]
    fn gui_enable(&mut self) {
        unsafe { ffi::GuiEnable() }
    }
    /// Disable gui controls (global state)
    #[inline]
    fn gui_disable(&mut self) {
        unsafe { ffi::GuiDisable() }
    }
    /// Lock gui controls (global state)
    #[inline]
    fn gui_lock(&mut self) {
        unsafe { ffi::GuiLock() }
    }
    /// Unlock gui controls (global state)
    #[inline]
    fn gui_unlock(&mut self) {
        unsafe { ffi::GuiUnlock() }
    }
    // Set gui controls alpha (global state), alpha goes from 0.0f to 1.0f
    #[inline]
    fn gui_fade(&mut self, color: Color, alpha: f32) -> Color {
        unsafe { ffi::Fade(color.into(), alpha).into() }
    }
    /// Set gui state (global state)
    #[inline]
    fn gui_set_state(&mut self, state: crate::consts::GuiState) {
        unsafe { ffi::GuiSetState(state as i32) }
    }
    /// Get gui state (global state)
    #[inline]
    fn gui_get_state(&mut self) -> crate::consts::GuiState {
        unsafe { std::mem::transmute(ffi::GuiGetState()) }
    }
    /// Set gui custom font (global state)
    #[inline]
    fn gui_set_font(&mut self, font: impl AsRef<ffi::Font>) {
        unsafe { ffi::GuiSetFont(*font.as_ref()) }
    }
    /// Get gui custom font (global state)
    #[inline]
    fn gui_get_font(&mut self) -> WeakFont {
        unsafe { WeakFont(ffi::GuiGetFont()) }
    }
    /// Set one style property
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    fn gui_set_style(&mut self, control: crate::consts::GuiControl, property: i32, value: i32) {
        unsafe { ffi::GuiSetStyle(control as i32, property as i32, value) }
    }

    /// Get one style property
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    fn gui_get_style(&self, control: crate::consts::GuiControl, property: i32) -> i32 {
        unsafe { ffi::GuiGetStyle(control as i32, property as i32) }
    }
    /// Load style file (.rgs)
    #[inline]
    fn gui_load_style(&mut self, filename: impl IntoCStr) {
        unsafe { ffi::GuiLoadStyle(filename.as_cstr_ptr()) }
    }
    /// Load style default over global style
    #[inline]
    fn gui_load_style_default(&mut self) {
        unsafe { ffi::GuiLoadStyleDefault() }
    }
    /// Window Box control, shows a window that can be closed
    #[inline]
    fn gui_window_box(&mut self, bounds: impl Into<ffi::Rectangle>, title: impl IntoCStr) -> bool {
        unsafe { ffi::GuiWindowBox(bounds.into(), title.as_cstr_ptr()) > 0 }
    }
    /// Group Box control with text name
    #[inline]
    fn gui_group_box(&mut self, bounds: impl Into<ffi::Rectangle>, text: impl IntoCStr) -> bool {
        unsafe { ffi::GuiGroupBox(bounds.into(), text.as_cstr_ptr()) > 0 }
    }
    /// Line separator control, could contain text
    #[inline]
    fn gui_line(&mut self, bounds: impl Into<ffi::Rectangle>, text: impl IntoCStr) -> bool {
        unsafe { ffi::GuiLine(bounds.into(), text.as_cstr_ptr()) > 0 }
    }
    /// Panel control, useful to group controls
    #[inline]
    fn gui_panel(&mut self, bounds: impl Into<ffi::Rectangle>, text: impl IntoCStr) -> bool {
        unsafe { ffi::GuiPanel(bounds.into(), text.as_cstr_ptr()) > 0 }
    }
    /// Scroll Panel control
    #[inline]
    fn gui_scroll_panel(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        content: impl Into<ffi::Rectangle>,
        scroll: impl Into<ffi::Vector2>,
        view: impl Into<ffi::Rectangle>,
    ) -> (bool, Rectangle, Vector2) {
        let mut scroll = scroll.into();
        let mut view = view.into();
        let result = unsafe {
            ffi::GuiScrollPanel(
                bounds.into(),
                text.as_cstr_ptr(),
                content.into(),
                &mut scroll,
                &mut view,
            )
        };
        (result > 0, view.into(), scroll.into())
    }
    /// Label control, shows text
    #[inline]
    fn gui_label(&mut self, bounds: impl Into<ffi::Rectangle>, text: impl IntoCStr) -> bool {
        unsafe { ffi::GuiLabel(bounds.into(), text.as_cstr_ptr()) > 0 }
    }
    /// Button control, returns true when clicked
    #[inline]
    fn gui_button(&mut self, bounds: impl Into<ffi::Rectangle>, text: impl IntoCStr) -> bool {
        unsafe { ffi::GuiButton(bounds.into(), text.as_cstr_ptr()) > 0 }
    }
    /// Label button control, show true when clicked
    #[inline]
    fn gui_label_button(&mut self, bounds: impl Into<ffi::Rectangle>, text: impl IntoCStr) -> bool {
        unsafe { ffi::GuiLabelButton(bounds.into(), text.as_cstr_ptr()) > 0 }
    }
    /// Toggle Button control, returns true when active
    #[inline]
    fn gui_toggle(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        active: &mut bool,
    ) -> bool {
        unsafe { ffi::GuiToggle(bounds.into(), text.as_cstr_ptr(), active) > 0 }
    }
    /// Toggle Group control, returns active toggle index
    #[inline]
    fn gui_toggle_group(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        active: &mut i32,
    ) -> i32 {
        unsafe { ffi::GuiToggleGroup(bounds.into(), text.as_cstr_ptr(), active) }
    }
    /// Check Box control, returns true when active
    #[inline]
    fn gui_check_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        checked: &mut bool,
    ) -> bool {
        unsafe { ffi::GuiCheckBox(bounds.into(), text.as_cstr_ptr(), checked) > 0 }
    }
    /// Combo Box control, returns selected item index
    #[inline]
    fn gui_combo_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        active: &mut i32,
    ) -> i32 {
        unsafe { ffi::GuiComboBox(bounds.into(), text.as_cstr_ptr(), active) }
    }
    /// Dropdown Box control, returns selected item
    #[inline]
    fn gui_dropdown_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        active: &mut i32,
        edit_mode: bool,
    ) -> bool {
        unsafe { ffi::GuiDropdownBox(bounds.into(), text.as_cstr_ptr(), active, edit_mode) > 0 }
    }
    /// Spinner control, returns selected value
    #[inline]
    fn gui_spinner(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        value: &mut i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> bool {
        unsafe {
            ffi::GuiSpinner(
                bounds.into(),
                // text.map(CStr::as_ptr).unwrap_or(crate::rstr!("").as_ptr()),
                text.as_cstr_ptr(),
                value,
                min_value,
                max_value,
                edit_mode,
            ) > 0
        }
    }
    /// Value Box control, updates input text with numbers
    #[inline]
    fn gui_value_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        value: &mut i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> bool {
        unsafe {
            ffi::GuiValueBox(
                bounds.into(),
                text.as_cstr_ptr(),
                value,
                min_value,
                max_value,
                edit_mode,
            ) > 0
        }
    }
    /// Text Box control, updates input text
    /// Use at your own risk!!! The allocated vector MUST have enough space for edits.
    #[inline]
    fn gui_text_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        buffer: &mut [u8],
        edit_mode: bool,
    ) -> bool {
        let len = buffer.len();
        let c_text = unsafe { CStr::from_bytes_with_nul_unchecked(buffer) };
        unsafe {
            ffi::GuiTextBox(
                bounds.into(),
                c_text.as_ptr() as *mut _,
                len as i32,
                edit_mode,
            ) > 0
        }
    }

    /// Slider control, returns selected value
    #[inline]
    fn gui_slider(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text_left: impl IntoCStr,
        text_right: impl IntoCStr,
        value: &mut f32,
        min_value: f32,
        max_value: f32,
    ) -> bool {
        unsafe {
            ffi::GuiSlider(
                bounds.into(),
                text_left.as_cstr_ptr(),
                text_right.as_cstr_ptr(),
                value,
                min_value,
                max_value,
            ) > 0
        }
    }
    /// Slider Bar control, returns selected value
    #[inline]
    fn gui_slider_bar(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text_left: impl IntoCStr,
        text_right: impl IntoCStr,
        value: &mut f32,
        min_value: f32,
        max_value: f32,
    ) -> bool {
        unsafe {
            ffi::GuiSliderBar(
                bounds.into(),
                text_left.as_cstr_ptr(),
                text_right.as_cstr_ptr(),
                value,
                min_value,
                max_value,
            ) > 0
        }
    }
    /// Progress Bar control, shows current progress value
    #[inline]
    fn gui_progress_bar(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text_left: impl IntoCStr,
        text_right: impl IntoCStr,
        value: &mut f32,
        min_value: f32,
        max_value: f32,
    ) -> bool {
        unsafe {
            ffi::GuiProgressBar(
                bounds.into(),
                text_left.as_cstr_ptr(),
                text_right.as_cstr_ptr(),
                value,
                min_value,
                max_value,
            ) > 0
        }
    }
    /// Status Bar control, shows info text
    #[inline]
    fn gui_status_bar(&mut self, bounds: impl Into<ffi::Rectangle>, text: impl IntoCStr) -> bool {
        unsafe { ffi::GuiStatusBar(bounds.into(), text.as_cstr_ptr()) > 0 }
    }
    /// Dummy control for placeholders
    #[inline]
    fn gui_dummy_rec(&mut self, bounds: impl Into<ffi::Rectangle>, text: impl IntoCStr) -> bool {
        unsafe { ffi::GuiStatusBar(bounds.into(), text.as_cstr_ptr()) > 0 }
    }
    /// Grid control
    #[inline]
    fn gui_grid(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        spacing: f32,
        subdivs: i32,
    ) -> (bool, Vector2) {
        let mut mouseCell = ffi::Vector2 { x: 0.0, y: 0.0 };
        (
            unsafe {
                ffi::GuiGrid(
                    bounds.into(),
                    text.as_cstr_ptr(),
                    spacing,
                    subdivs,
                    &mut mouseCell,
                ) > 0
            },
            mouseCell.into(),
        )
    }
    /// List View control, returns selected list item index
    #[inline]
    fn gui_list_view(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        scroll_index: &mut i32,
        active: &mut i32,
    ) -> i32 {
        unsafe { ffi::GuiListView(bounds.into(), text.as_cstr_ptr(), scroll_index, active) }
    }
    /// List View with extended parameters
    #[inline]
    fn gui_list_view_ex(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &[&CStr],
        focus: &mut i32,
        scroll_index: &mut i32,
        active: &mut i32,
    ) -> i32 {
        let mut buffer = Vec::with_capacity(text.len());
        for t in text {
            buffer.push(t.as_ptr());
        }
        unsafe {
            ffi::GuiListViewEx(
                bounds.into(),
                buffer.as_mut_ptr(),
                text.len() as i32,
                focus,
                scroll_index,
                active,
            )
        }
    }
    /// Message Box control, displays a message
    #[inline]
    fn gui_message_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        message: impl IntoCStr,
        buttons: impl IntoCStr,
    ) -> i32 {
        unsafe {
            ffi::GuiMessageBox(
                bounds.into(),
                text.as_cstr_ptr(),
                message.as_cstr_ptr(),
                buttons.as_cstr_ptr(),
            )
        }
    }
    /// Text Input Box control, ask for text
    #[inline]
    fn gui_text_input_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        title: impl IntoCStr,
        message: impl IntoCStr,
        buttons: impl IntoCStr,
        text: &mut Vec<u8>,
        text_max_size: i32,
        secret_view_active: &mut bool,
    ) -> i32 {
        // rgui.h: line 3699 MAX_FILENAME_LEN
        text.reserve((256 - text.len()).max(0) as usize);
        let btn_index = unsafe {
            ffi::GuiTextInputBox(
                bounds.into(),
                title.as_cstr_ptr(),
                message.as_cstr_ptr(),
                buttons.as_cstr_ptr(),
                text.as_mut_ptr() as *mut _,
                text_max_size,
                secret_view_active,
            )
        };

        btn_index
    }

    /// Color Picker control
    #[inline]
    fn gui_color_picker(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        color: impl Into<ffi::Color>,
    ) -> Color {
        let mut out = color.into();
        let result = unsafe { ffi::GuiColorPicker(bounds.into(), text.as_cstr_ptr(), &mut out) };
        return out.into();
    }
    // Get text with icon id prepended
    // NOTE: Useful to add icons by name id (enum) instead of
    // a number that can change between ricon versions
    #[inline]
    fn gui_icon_text(
        &mut self,
        icon_id: crate::consts::GuiIconName,
        text: impl IntoCStr,
    ) -> String {
        let buffer = unsafe { ffi::GuiIconText(icon_id as i32, text.as_cstr_ptr()) };
        if buffer.is_null() {
            let ptr = text.as_cstr_ptr();
            if ptr.is_null() {
                return String::default();
            }
            return unsafe { CStr::from_ptr(ptr).to_string_lossy().to_string() };
        }
        let c_str = unsafe { CStr::from_ptr(buffer) };
        let str_slice = c_str.to_str().unwrap_or("");
        let str_buf = str_slice.to_owned();
        // THERE IS NO WAY THIS DOESN"T LEEK MEMORY. TODO figure out a way to free this buffer.
        str_buf
    }

    /// Color Bar Alpha control
    /// NOTE: Returns alpha value normalized [0..1]
    #[inline]
    fn gui_color_bar_alpha(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl IntoCStr,
        alpha: &mut f32,
    ) -> bool {
        unsafe { ffi::GuiColorBarAlpha(bounds.into(), text.as_cstr_ptr(), alpha) > 0 }
    }
}
