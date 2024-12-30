use crate::core::color::Color;
use crate::core::drawing::RaylibDraw;
use crate::core::math::{Rectangle, Vector2};
use crate::core::text::WeakFont;
use crate::core::RaylibHandle;
use crate::ffi;

use std::ffi::{c_char, CStr, CString};

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
    #[inline]
    pub fn gui_set_style(
        &mut self,
        control: crate::consts::GuiControl,
        property: impl GuiProperty,
        value: i32,
    ) {
        unsafe { ffi::GuiSetStyle(control as i32, property.as_i32(), value) }
    }

    /// Get one style property
    #[inline]
    pub fn gui_get_style(
        &mut self,
        control: crate::consts::GuiControl,
        property: impl GuiProperty,
    ) -> i32 {
        unsafe { ffi::GuiGetStyle(control as i32, property.as_i32()) }
    }
    /// Load style file (.rgs)
    #[inline]
    pub fn gui_load_style(&mut self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe { ffi::GuiLoadStyle(c_filename.as_ptr()) }
    }
    /// Load style default over global style
    #[inline]
    pub fn gui_load_style_default(&mut self) {
        unsafe { ffi::GuiLoadStyleDefault() }
    }

    /// Enable gui tooltips (global state)
    #[inline]
    pub fn gui_enable_tooltip(&mut self) {
        unsafe { ffi::GuiEnableTooltip() };
    }

    /// Disable gui tooltips (global state)
    #[inline]
    pub fn gui_disable_tooltip(&mut self) {
        unsafe { ffi::GuiDisableTooltip() };
    }

    /// Set tooltip string
    #[inline]
    pub fn gui_set_tooltip(&mut self, tooltip: &str) {
        let c_text = CString::new(tooltip).unwrap();
        unsafe {
            ffi::GuiSetTooltip(c_text.as_ptr());
        }
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

    /// Check if gui is locked (global state)
    #[inline]
    fn gui_is_locked(&mut self) -> bool {
        unsafe { ffi::GuiIsLocked() }
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
    #[inline]
    fn gui_set_style(
        &mut self,
        control: crate::consts::GuiControl,
        property: impl GuiProperty,
        value: i32,
    ) {
        unsafe { ffi::GuiSetStyle(control as i32, property.as_i32(), value) }
    }

    /// Set gui controls alpha (global state), alpha goes from 0.0f to 1.0f
    fn gui_set_alpha(&mut self, alpha: f32) {
        unsafe {
            ffi::GuiSetAlpha(alpha);
        }
    }

    /// Get one style property
    #[inline]
    fn gui_get_style(&self, control: crate::consts::GuiControl, property: impl GuiProperty) -> i32 {
        unsafe { ffi::GuiGetStyle(control as i32, property.as_i32()) }
    }
    /// Load style file (.rgs)
    #[inline]
    fn gui_load_style(&mut self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe { ffi::GuiLoadStyle(c_filename.as_ptr()) }
    }
    /// Load style default over global style
    #[inline]
    fn gui_load_style_default(&mut self) {
        unsafe { ffi::GuiLoadStyleDefault() }
    }
    /// Window Box control, shows a window that can be closed
    #[inline]
    fn gui_window_box(&mut self, bounds: impl Into<ffi::Rectangle>, title: &str) -> bool {
        let c_filename = CString::new(title).unwrap();
        unsafe { ffi::GuiWindowBox(bounds.into(), c_filename.as_ptr()) > 0 }
    }
    /// Group Box control with text name
    #[inline]
    fn gui_group_box(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_filename = CString::new(text).unwrap();
        unsafe { ffi::GuiGroupBox(bounds.into(), c_filename.as_ptr()) > 0 }
    }
    /// Line separator control, could contain text
    #[inline]
    fn gui_line(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_filename = CString::new(text).unwrap();
        unsafe { ffi::GuiLine(bounds.into(), c_filename.as_ptr()) > 0 }
    }
    /// Panel control, useful to group controls
    #[inline]
    fn gui_panel(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_filename = CString::new(text).unwrap();
        unsafe { ffi::GuiPanel(bounds.into(), c_filename.as_ptr()) > 0 }
    }
    /// Scroll Panel control
    #[inline]
    fn gui_scroll_panel(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        content: impl Into<ffi::Rectangle>,
        scroll: impl Into<ffi::Vector2>,
        view: impl Into<ffi::Rectangle>,
    ) -> (bool, Rectangle, Vector2) {
        let mut scroll = scroll.into();
        let mut view = view.into();
        let c_filename = CString::new(text).unwrap();
        let result = unsafe {
            ffi::GuiScrollPanel(
                bounds.into(),
                c_filename.as_ptr(),
                content.into(),
                &mut scroll,
                &mut view,
            )
        };
        (result > 0, view.into(), scroll.into())
    }
    /// Label control, shows text
    #[inline]
    fn gui_label(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiLabel(bounds.into(), c_text.as_ptr()) > 0 }
    }
    /// Button control, returns true when clicked
    #[inline]
    fn gui_button(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiButton(bounds.into(), c_text.as_ptr()) > 0 }
    }
    /// Label button control, show true when clicked
    #[inline]
    fn gui_label_button(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiLabelButton(bounds.into(), c_text.as_ptr()) > 0 }
    }
    /// Toggle Button control, returns true when active
    #[inline]
    fn gui_toggle(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        active: &mut bool,
    ) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiToggle(bounds.into(), c_text.as_ptr(), active) > 0 }
    }
    /// Toggle Group control, returns active toggle index
    #[inline]
    fn gui_toggle_group(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        active: &mut i32,
    ) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiToggleGroup(bounds.into(), c_text.as_ptr(), active) }
    }
    /// Check Box control, returns true when active
    #[inline]
    fn gui_check_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        checked: &mut bool,
    ) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiCheckBox(bounds.into(), c_text.as_ptr(), checked) > 0 }
    }
    /// Combo Box control, returns selected item index
    #[inline]
    fn gui_combo_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        active: &mut i32,
    ) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiComboBox(bounds.into(), c_text.as_ptr(), active) }
    }
    /// Dropdown Box control, returns selected item
    #[inline]
    fn gui_dropdown_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        active: &mut i32,
        edit_mode: bool,
    ) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiDropdownBox(bounds.into(), c_text.as_ptr(), active, edit_mode) > 0 }
    }
    /// Spinner control, returns selected value
    #[inline]
    fn gui_spinner(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        value: &mut i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::GuiSpinner(
                bounds.into(),
                // text.map(CStr::as_ptr).unwrap_or(crate::rstr!("").as_ptr()),
                c_text.as_ptr(),
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
        text: &str,
        value: &mut i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::GuiValueBox(
                bounds.into(),
                c_text.as_ptr(),
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
        buffer: &mut String,
        edit_mode: bool,
    ) -> bool {
        let len = buffer.len();
        unsafe {
            ffi::GuiTextBox(
                bounds.into(),
                buffer.as_mut_ptr() as *mut _,
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
        text_left: &str,
        text_right: &str,
        value: &mut f32,
        min_value: f32,
        max_value: f32,
    ) -> bool {
        let c_text_left = CString::new(text_left).unwrap();
        let c_text_right = CString::new(text_right).unwrap();
        unsafe {
            ffi::GuiSlider(
                bounds.into(),
                c_text_left.as_ptr(),
                c_text_right.as_ptr(),
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
        text_left: &str,
        text_right: &str,
        value: &mut f32,
        min_value: f32,
        max_value: f32,
    ) -> bool {
        let c_text_left = CString::new(text_left).unwrap();
        let c_text_right = CString::new(text_right).unwrap();
        unsafe {
            ffi::GuiSliderBar(
                bounds.into(),
                c_text_left.as_ptr(),
                c_text_right.as_ptr(),
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
        text_left: &str,
        text_right: &str,
        value: &mut f32,
        min_value: f32,
        max_value: f32,
    ) -> bool {
        let c_text_left = CString::new(text_left).unwrap();
        let c_text_right = CString::new(text_right).unwrap();
        unsafe {
            ffi::GuiProgressBar(
                bounds.into(),
                c_text_left.as_ptr(),
                c_text_right.as_ptr(),
                value,
                min_value,
                max_value,
            ) > 0
        }
    }
    /// Status Bar control, shows info text
    #[inline]
    fn gui_status_bar(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiStatusBar(bounds.into(), c_text.as_ptr()) > 0 }
    }

    /// Grid control
    #[inline]
    fn gui_grid(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        spacing: f32,
        subdivs: i32,
    ) -> (bool, Vector2) {
        let c_text = CString::new(text).unwrap();
        let mut mouseCell = ffi::Vector2 { x: 0.0, y: 0.0 };
        (
            unsafe {
                ffi::GuiGrid(
                    bounds.into(),
                    c_text.as_ptr(),
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
        text: &str,
        scroll_index: &mut i32,
        active: &mut i32,
    ) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiListView(bounds.into(), c_text.as_ptr(), scroll_index, active) }
    }
    /// List View with extended parameters
    #[inline]
    fn gui_list_view_ex(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: impl Iterator<Item = impl AsRef<str>>,
        focus: &mut i32,
        scroll_index: &mut i32,
        active: &mut i32,
    ) -> i32 {
        // We need to keep track of all CStr buffers.
        let buffer: Box<[Box<CStr>]> = text
            .map(|s| CString::new(s.as_ref()).unwrap().into_boxed_c_str())
            .collect();

        let mut text_params: Box<[*const c_char]> =
            buffer.iter().map(|cstr| cstr.as_ptr()).collect();

        unsafe {
            ffi::GuiListViewEx(
                bounds.into(),
                text_params.as_mut_ptr(),
                text_params.len() as i32,
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
        text: &str,
        message: &str,
        buttons: &str,
    ) -> i32 {
        let c_text = CString::new(text).unwrap();
        let c_message = CString::new(message).unwrap();
        let c_buttons = CString::new(buttons).unwrap();
        unsafe {
            ffi::GuiMessageBox(
                bounds.into(),
                c_text.as_ptr(),
                c_message.as_ptr(),
                c_buttons.as_ptr(),
            )
        }
    }
    /// Text Input Box control, ask for text
    #[inline]
    fn gui_text_input_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        title: &str,
        message: &str,
        buttons: &str,
        text: &mut String,
        text_max_size: i32,
        secret_view_active: &mut bool,
    ) -> i32 {
        // rgui.h: line 3699 MAX_FILENAME_LEN
        text.reserve((256 - text.len()).max(0) as usize);
        let c_title = CString::new(title).unwrap();
        let c_message = CString::new(message).unwrap();
        let c_buttons = CString::new(buttons).unwrap();
        let btn_index = unsafe {
            ffi::GuiTextInputBox(
                bounds.into(),
                c_title.as_ptr(),
                c_message.as_ptr(),
                c_buttons.as_ptr(),
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
        text: &str,
        color: impl Into<ffi::Color>,
    ) -> Color {
        let mut out = color.into();
        let c_text = CString::new(text).unwrap();

        let result = unsafe { ffi::GuiColorPicker(bounds.into(), c_text.as_ptr(), &mut out) };
        return out.into();
    }
    // Get text with icon id prepended
    // NOTE: Useful to add icons by name id (enum) instead of
    // a number that can change between ricon versions
    #[inline]
    fn gui_icon_text(&mut self, icon_id: crate::consts::GuiIconName, text: &str) -> String {
        let c_text = CString::new(text).unwrap();
        let buffer = unsafe { ffi::GuiIconText(icon_id as i32, c_text.as_ptr()) };
        if buffer.is_null() {
            let ptr = c_text.as_ptr();
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
        text: &str,
        alpha: &mut f32,
    ) -> bool {
        let c_text = CString::new(text).unwrap();

        unsafe { ffi::GuiColorBarAlpha(bounds.into(), c_text.as_ptr(), alpha) > 0 }
    }

    /// Toggle Slider control
    #[inline]
    fn gui_toggle_slider(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        active: &mut i32,
    ) -> bool {
        let c_text = CString::new(text).unwrap();

        unsafe { ffi::GuiToggleSlider(bounds.into(), c_text.as_ptr(), active) > 0 }
    }

    /// Dummy control for placeholders
    #[inline]
    fn gui_dummy_rec(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_text = CString::new(text).unwrap();

        unsafe { ffi::GuiDummyRec(bounds.into(), c_text.as_ptr()) > 0 }
    }

    /// Color Bar Hue control
    #[inline]
    fn gui_color_bar_hue(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        value: &mut f32,
    ) -> bool {
        let c_text = CString::new(text).unwrap();

        unsafe { ffi::GuiColorBarHue(bounds.into(), c_text.as_ptr(), value) > 0 }
    }
}

#[diagnostic::on_unimplemented(
    message = "{Self} is not a gui property, or does not implement the GuiProperty trait.",
    note = "As of Raylib 5.5, raygui functions that once took \"property enum as i32\" now just take the enum."
)]
pub trait GuiProperty {
    fn as_i32(self) -> i32;
}

impl GuiProperty for crate::consts::GuiControlProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiDefaultProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiCheckBoxProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiColorPickerProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiComboBoxProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiDropdownBoxProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiListViewProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiProgressBarProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiScrollBarProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiSliderProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiSpinnerProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
impl GuiProperty for crate::consts::GuiToggleProperty {
    fn as_i32(self) -> i32 {
        self as i32
    }
}
