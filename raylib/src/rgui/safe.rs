use crate::core::{drawing::RaylibDraw, text::Font, RaylibHandle};
use crate::ffi::{self, Color, Rectangle, Vector2};

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr;

/// Global gui modification functions
impl RaylibHandle<'_> {
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
    pub fn gui_fade(&mut self, alpha: f32) {
        unsafe { ffi::GuiFade(alpha) }
    }
    /// Set gui state (global state)
    #[inline]
    pub fn gui_set_state(&mut self, state: ffi::GuiState) {
        unsafe { ffi::GuiSetState(state as i32) }
    }
    /// Get gui state (global state)
    #[inline]
    pub fn gui_get_state(&mut self) -> ffi::GuiState {
        unsafe { std::mem::transmute(ffi::GuiGetState()) }
    }
    /// Set gui custom font (global state)
    #[inline]
    pub fn gui_set_font(&mut self, font: impl AsRef<ffi::Font>) {
        unsafe { ffi::GuiSetFont(*font.as_ref()) }
    }
    /// Get gui custom font (global state)
    #[inline]
    pub fn gui_get_font<'bind>(&'bind mut self) -> ManuallyDrop<Font<'bind, '_>> {
        unsafe { ManuallyDrop::new(Font(ffi::GuiGetFont(), PhantomData, PhantomData)) }
    }
    /// Set one style property
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    pub fn gui_set_style(&mut self, control: ffi::GuiControl, property: i32, value: i32) {
        unsafe { ffi::GuiSetStyle(control as i32, property as i32, value) }
    }

    /// Get one style property
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    pub fn gui_get_style(&mut self, control: ffi::GuiControl, property: i32) -> i32 {
        unsafe { ffi::GuiGetStyle(control as i32, property as i32) }
    }
    /// Load style file (.rgs)
    #[inline]
    pub fn gui_load_style(&mut self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        unsafe { ffi::GuiLoadStyle(filename.as_ptr()) }
    }
    /// Load style default over global style
    #[inline]
    pub fn gui_load_style_default(&mut self) {
        unsafe { ffi::GuiLoadStyleDefault() }
    }
}

impl<'a, D: RaylibDraw> RaylibDrawGui<'a> for D {}

pub trait RaylibDrawGui<'a> {
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
    fn gui_fade(&mut self, alpha: f32) {
        unsafe { ffi::GuiFade(alpha) }
    }
    /// Set gui state (global state)
    #[inline]
    fn gui_set_state(&mut self, state: ffi::GuiState) {
        unsafe { ffi::GuiSetState(state as i32) }
    }
    /// Get gui state (global state)
    #[inline]
    fn gui_get_state(&mut self) -> ffi::GuiState {
        unsafe { std::mem::transmute(ffi::GuiGetState()) }
    }
    /// Set gui custom font (global state)
    #[inline]
    fn gui_set_font(&mut self, font: impl AsRef<ffi::Font>) {
        unsafe { ffi::GuiSetFont(*font.as_ref()) }
    }
    /// Get gui custom font (global state)
    #[inline]
    fn gui_get_font<'bind>(&'bind mut self) -> ManuallyDrop<Font<'bind, '_>> {
        unsafe { ManuallyDrop::new(Font(ffi::GuiGetFont(), PhantomData, PhantomData)) }
    }
    /// Set one style property
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    fn gui_set_style(&mut self, control: ffi::GuiControl, property: i32, value: i32) {
        unsafe { ffi::GuiSetStyle(control as i32, property as i32, value) }
    }

    /// Get one style property
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    fn gui_get_style(&self, control: ffi::GuiControl, property: i32) -> i32 {
        unsafe { ffi::GuiGetStyle(control as i32, property as i32) }
    }
    /// Load style file (.rgs)
    #[inline]
    fn gui_load_style(&mut self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        unsafe { ffi::GuiLoadStyle(filename.as_ptr()) }
    }
    /// Load style default over global style
    #[inline]
    fn gui_load_style_default(&mut self) {
        unsafe { ffi::GuiLoadStyleDefault() }
    }
    /// Window Box control, shows a window that can be closed
    #[inline]
    fn gui_window_box(&mut self, bounds: Rectangle, title: &str) -> bool {
        let title = CString::new(title).unwrap();
        unsafe { ffi::GuiWindowBox(bounds, title.as_ptr()) }
    }
    /// Group Box control with text name
    #[inline]
    fn gui_group_box(&mut self, bounds: Rectangle, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiGroupBox(bounds, text.as_ptr()) }
    }
    /// Line separator control, could contain text
    #[inline]
    fn gui_line(&mut self, bounds: Rectangle, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiLine(bounds, text.as_ptr()) }
    }
    /// Panel control, useful to group controls
    #[inline]
    fn gui_panel(&mut self, bounds: Rectangle, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiPanel(bounds, text.as_ptr()) }
    }
    /// Scroll Panel control
    #[inline]
    fn gui_scroll_panel(
        &mut self,
        bounds: Rectangle,
        text: &str,
        content: Rectangle,
        scroll: impl Into<Vector2>,
    ) -> (Rectangle, Vector2) {
        let text = CString::new(text).unwrap();
        let mut scroll = scroll.into();
        let bounds: ffi::Rectangle =
            unsafe { ffi::GuiScrollPanel(bounds, text.as_ptr(), content.into(), &mut scroll) };
        return (bounds, scroll.into());
    }
    /// Label control, shows text
    #[inline]
    fn gui_label(&mut self, bounds: Rectangle, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiLabel(bounds, text.as_ptr()) }
    }
    /// Button control, returns true when clicked
    #[inline]
    fn gui_button(&mut self, bounds: Rectangle, text: &str) -> bool {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiButton(bounds, text.as_ptr()) }
    }
    /// Label button control, show true when clicked
    #[inline]
    fn gui_label_button(&mut self, bounds: Rectangle, text: &str) -> bool {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiLabelButton(bounds, text.as_ptr()) }
    }
    /// Toggle Button control, returns true when active
    #[inline]
    fn gui_toggle(&mut self, bounds: Rectangle, text: &str, active: bool) -> bool {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiToggle(bounds, text.as_ptr(), active) }
    }
    /// Toggle Group control, returns active toggle index
    #[inline]
    fn gui_toggle_group(&mut self, bounds: Rectangle, text: &str, active: i32) -> i32 {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiToggleGroup(bounds, text.as_ptr(), active) }
    }
    /// Check Box control, returns true when active
    #[inline]
    fn gui_check_box(&mut self, bounds: Rectangle, text: &str, checked: bool) -> bool {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiCheckBox(bounds, text.as_ptr(), checked) }
    }
    /// Combo Box control, returns selected item index
    #[inline]
    fn gui_combo_box(&mut self, bounds: Rectangle, text: &str, active: i32) -> i32 {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiComboBox(bounds, text.as_ptr(), active) }
    }
    /// Dropdown Box control, returns selected item
    #[inline]
    fn gui_dropdown_box(
        &mut self,
        bounds: Rectangle,
        text: &str,
        active: &mut i32,
        edit_mode: bool,
    ) -> bool {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiDropdownBox(bounds, text.as_ptr(), active, edit_mode) }
    }
    /// Spinner control, returns selected value
    #[inline]
    fn gui_spinner(
        &mut self,
        bounds: Rectangle,
        text: &str,
        value: &mut i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> bool {
        let text = CString::new(text).unwrap();
        unsafe {
            ffi::GuiSpinner(
                bounds,
                // text.map(CStr::as_ptr).unwrap_or(crate::rstr!("").as_ptr()),
                text.as_ptr(),
                value,
                min_value,
                max_value,
                edit_mode,
            )
        }
    }
    /// Value Box control, updates input text with numbers
    #[inline]
    fn gui_value_box(
        &mut self,
        bounds: Rectangle,
        text: &str,
        value: &mut i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> bool {
        let text = CString::new(text).unwrap();
        unsafe {
            ffi::GuiValueBox(
                bounds,
                text.as_ptr(),
                value,
                min_value,
                max_value,
                edit_mode,
            )
        }
    }
    /// Text Box control, updates input text
    /// Use at your own risk!!! The allocated vector MUST have enough space for edits.
    #[inline]
    fn gui_text_box(&mut self, bounds: Rectangle, buffer: &mut [u8], edit_mode: bool) -> bool {
        let len = buffer.len();
        let c_text = unsafe { CStr::from_bytes_with_nul_unchecked(buffer) };
        unsafe { ffi::GuiTextBox(bounds, c_text.as_ptr() as *mut _, len as i32, edit_mode) }
    }
    /// Text Box control with multiple lines
    /// Use at your own risk!!! The allocated vector MUST have a nul terminator.
    #[inline]
    fn gui_text_box_multi(
        &mut self,
        bounds: Rectangle,
        buffer: &mut [u8],
        edit_mode: bool,
    ) -> bool {
        let len = buffer.len();
        let c_text = unsafe { CStr::from_bytes_with_nul_unchecked(buffer) };
        unsafe { ffi::GuiTextBoxMulti(bounds, c_text.as_ptr() as *mut _, len as i32, edit_mode) }
    }
    /// Slider control, returns selected value
    #[inline]
    fn gui_slider(
        &mut self,
        bounds: Rectangle,
        text_left: &str,
        text_right: &str,
        value: f32,
        min_value: f32,
        max_value: f32,
    ) -> f32 {
        let text_left = CString::new(text_left).unwrap();
        let text_right = CString::new(text_right).unwrap();

        unsafe {
            ffi::GuiSlider(
                bounds,
                text_left.as_ptr(),
                text_right.as_ptr(),
                value,
                min_value,
                max_value,
            )
        }
    }
    /// Slider Bar control, returns selected value
    #[inline]
    fn gui_slider_bar(
        &mut self,
        bounds: Rectangle,
        text_left: &str,
        text_right: &str,
        value: f32,
        min_value: f32,
        max_value: f32,
    ) -> f32 {
        let text_left = CString::new(text_left).unwrap();
        let text_right = CString::new(text_right).unwrap();

        unsafe {
            ffi::GuiSliderBar(
                bounds,
                text_left.as_ptr(),
                text_right.as_ptr(),
                value,
                min_value,
                max_value,
            )
        }
    }
    /// Progress Bar control, shows current progress value
    #[inline]
    fn gui_progress_bar(
        &mut self,
        bounds: Rectangle,
        text_left: &str,
        text_right: &str,
        value: f32,
        min_value: f32,
        max_value: f32,
    ) -> f32 {
        let text_left = CString::new(text_left).unwrap();
        let text_right = CString::new(text_right).unwrap();

        unsafe {
            ffi::GuiProgressBar(
                bounds,
                text_left.as_ptr(),
                text_right.as_ptr(),
                value,
                min_value,
                max_value,
            )
        }
    }
    /// Status Bar control, shows info text
    #[inline]
    fn gui_status_bar(&mut self, bounds: Rectangle, text: &str) {
        let text = CString::new(text).unwrap();

        unsafe { ffi::GuiStatusBar(bounds, text.as_ptr()) }
    }
    /// Dummy control for placeholders
    #[inline]
    fn gui_dummy_rec(&mut self, bounds: Rectangle, text: &str) {
        let text = CString::new(text).unwrap();

        unsafe { ffi::GuiStatusBar(bounds, text.as_ptr()) }
    }
    /// Grid control
    #[inline]
    fn gui_grid(&mut self, bounds: Rectangle, text: &str, spacing: f32, subdivs: i32) -> Vector2 {
        let text = CString::new(text).unwrap();

        unsafe { ffi::GuiGrid(bounds, text.as_ptr(), spacing, subdivs).into() }
    }
    /// List View control, returns selected list item index
    #[inline]
    fn gui_list_view(
        &mut self,
        bounds: Rectangle,
        text: &str,
        scroll_index: &mut i32,
        active: i32,
    ) -> i32 {
        let text = CString::new(text).unwrap();
        unsafe { ffi::GuiListView(bounds, text.as_ptr(), scroll_index, active) }
    }
    /// List View with extended parameters
    #[inline]
    fn gui_list_view_ex(
        &mut self,
        bounds: Rectangle,
        text: &[&str],
        focus: &mut i32,
        scroll_index: &mut i32,
        active: i32,
    ) -> i32 {
        let text_buffer: Box<[CString]> =
            text.iter().map(|str| CString::new(*str).unwrap()).collect();
        let mut buffer: Box<[*mut u8]> = text_buffer.iter().map(|b| b.as_ptr() as _).collect();

        unsafe {
            ffi::GuiListViewEx(
                bounds,
                buffer.as_mut_ptr() as _,
                buffer.len() as i32,
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
        bounds: Rectangle,
        text: &str,
        message: &str,
        buttons: &str,
    ) -> i32 {
        let text = CString::new(text).unwrap();
        let message = CString::new(message).unwrap();
        let buttons = CString::new(buttons).unwrap();

        unsafe { ffi::GuiMessageBox(bounds, text.as_ptr(), message.as_ptr(), buttons.as_ptr()) }
    }
    /// Text Input Box control, ask for text
    #[inline]
    fn gui_text_input_box(
        &mut self,
        bounds: Rectangle,
        title: &str,
        message: &str,
        buttons: &str,
        text: &mut Vec<u8>,
        text_max_size: i32,
        secret_view_active: Option<bool>,
    ) -> (i32, Option<bool>) {
        let title = CString::new(title).unwrap();
        let message = CString::new(message).unwrap();
        let buttons = CString::new(buttons).unwrap();

        let mut secret_view_active_int: Option<i32> =
            secret_view_active.map(|s| if s { 1 } else { 0 });

        // rgui.h: line 3699 MAX_FILENAME_LEN
        text.reserve((256 - text.len()).max(0) as usize);
        let btn_index = unsafe {
            ffi::GuiTextInputBox(
                bounds,
                title.as_ptr(),
                message.as_ptr(),
                buttons.as_ptr(),
                text.as_mut_ptr() as *mut _,
                text_max_size,
                secret_view_active_int
                    .as_mut()
                    .map(|ptr| ptr as *mut i32)
                    .unwrap_or(ptr::null_mut()),
            )
        };

        (btn_index, secret_view_active_int.map(|i| i != 0))
    }

    /// Color Picker control
    #[inline]
    fn gui_color_picker(&mut self, bounds: Rectangle, text: &str, color: Color) -> Color {
        let text = CString::new(text).unwrap();

        unsafe { ffi::GuiColorPicker(bounds, text.as_ptr(), color).into() }
    }
    // Get text with icon id prepended
    // NOTE: Useful to add icons by name id (enum) instead of
    // a number that can change between ricon versions
    #[inline]
    fn gui_icon_text(&mut self, icon_id: ffi::GuiIconName, text: &str) -> String {
        let text = CString::new(text).unwrap();

        let buffer = unsafe { ffi::GuiIconText(icon_id as i32, text.as_ptr()) };
        if buffer.is_null() {
            let ptr = text.as_ptr();
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
    fn gui_color_bar_alpha(&mut self, bounds: Rectangle, text: &str, alpha: f32) -> f32 {
        let text = CString::new(text).unwrap();

        unsafe { ffi::GuiColorBarAlpha(bounds, text.as_ptr(), alpha) }
    }
}
