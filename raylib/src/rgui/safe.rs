use crate::core::color::Color;
use crate::core::drawing::RaylibDraw;
use crate::core::math::{Rectangle, Vector2};
use crate::core::text::WeakFont;
use crate::core::RaylibHandle;
use crate::ffi;

use std::ffi::CString;

/// Global gui modification functions
impl RaylibHandle {
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
    fn gui_set_state(&mut self, state: crate::consts::GuiControlState) {
        unsafe { ffi::GuiSetState(state as i32) }
    }
    /// Get gui state (global state)
    #[inline]
    fn gui_get_state(&mut self) -> crate::consts::GuiControlState {
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
        property: crate::consts::GuiControlProperty,
        value: i32,
    ) {
        unsafe { ffi::GuiSetStyle(control as i32, property as i32, value) }
    }

    /// Get one style property
    #[inline]
    fn gui_get_style(
        &mut self,
        control: crate::consts::GuiControl,
        property: crate::consts::GuiControlProperty,
    ) -> i32 {
        unsafe { ffi::GuiGetStyle(control as i32, property as i32) }
    }
    /// Load style file (.rgs)
    #[inline]
    fn gui_load_style(&mut self, filename: &str) {
        let c_text = CString::new(filename).unwrap();
        unsafe { ffi::GuiLoadStyle(c_text.as_ptr()) }
    }
    /// Load style properties from array
    #[inline]
    fn gui_load_style_props(&mut self, props: &[crate::consts::GuiControlProperty]) {
        unsafe { ffi::GuiLoadStyleProps(props.as_ptr() as *const _, props.len() as i32) }
    }
    /// Load style default over global style
    #[inline]
    fn gui_load_style_default(&mut self) {
        unsafe { ffi::GuiLoadStyleDefault() }
    }
    /// Updates full style properties set with default values
    #[inline]
    fn gui_update_style_complete(&mut self) {
        unsafe { ffi::GuiUpdateStyleComplete() }
    }
}

impl<D: RaylibDraw> RaylibDrawGui for D {}

pub trait RaylibDrawGui {
    /// Window Box control, shows a window that can be closed
    #[inline]
    fn gui_window_box(&mut self, bounds: impl Into<ffi::Rectangle>, title: &str) -> bool {
        let c_text = CString::new(title).unwrap();
        unsafe { ffi::GuiWindowBox(bounds.into(), c_text.as_ptr()) }
    }
    /// Group Box control with text name
    #[inline]
    fn gui_group_box(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiGroupBox(bounds.into(), c_text.as_ptr()) }
    }
    /// Line separator control, could contain text
    #[inline]
    fn gui_line(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiLine(bounds.into(), c_text.as_ptr()) }
    }
    /// Panel control, useful to group controls
    #[inline]
    fn gui_panel(&mut self, bounds: impl Into<ffi::Rectangle>) {
        unsafe { ffi::GuiPanel(bounds.into()) }
    }
    /// Scroll Panel control
    #[inline]
    fn gui_scroll_panel(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        content: impl Into<ffi::Rectangle>,
        scroll: impl Into<ffi::Vector2>,
    ) -> (Rectangle, Vector2) {
        let mut scroll = scroll.into();
        let bounds: ffi::Rectangle =
            unsafe { ffi::GuiScrollPanel(bounds.into(), content.into(), &mut scroll) };
        return (bounds.into(), scroll.into());
    }
    /// Label control, shows text
    #[inline]
    fn gui_label(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiLabel(bounds.into(), c_text.as_ptr()) }
    }
    /// Button control, returns true when clicked
    #[inline]
    fn gui_button(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiButton(bounds.into(), c_text.as_ptr()) }
    }
    /// Label button control, show true when clicked
    #[inline]
    fn gui_label_button(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiLabelButton(bounds.into(), c_text.as_ptr()) }
    }
    /// Image button control, returns true when clicked
    #[inline]
    fn gui_image_button(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        texture: impl AsRef<ffi::Texture>,
    ) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiImageButton(bounds.into(), c_text.as_ptr(), *texture.as_ref()) }
    }
    /// Image button extended control, returns true when clicked
    #[inline]
    fn gui_image_button_ex(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        texture: impl AsRef<ffi::Texture>,
        tex_source: impl Into<ffi::Rectangle>,
    ) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::GuiImageButtonEx(
                bounds.into(),
                c_text.as_ptr(),
                *texture.as_ref(),
                tex_source.into(),
            )
        }
    }
    /// Toggle Button control, returns true when active
    #[inline]
    fn gui_toggle(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str, active: bool) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiToggle(bounds.into(), c_text.as_ptr(), active) }
    }
    /// Toggle Group control, returns active toggle index
    #[inline]
    fn gui_toggle_group(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        active: i32,
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
        checked: bool,
    ) -> bool {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiCheckBox(bounds.into(), c_text.as_ptr(), checked) }
    }
    /// Combo Box control, returns selected item index
    #[inline]
    fn gui_combo_box(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str, active: i32) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiComboBox(bounds.into(), c_text.as_ptr(), active) }
    }
    /// Dropdown Box control, returns selected item
    #[inline]
    fn gui_dropdown_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        active: i32,
        edit_mode: bool,
    ) -> (i32, bool) {
        let mut active = active;
        let c_text = CString::new(text).unwrap();
        let clicked =
            unsafe { ffi::GuiDropdownBox(bounds.into(), c_text.as_ptr(), &mut active, edit_mode) };
        return (active, clicked);
    }
    /// Spinner control, returns selected value
    #[inline]
    fn gui_spinner_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        value: i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> (i32, bool) {
        debug_assert!(
            min_value >= value && value <= max_value,
            "value out of bounds"
        );
        let mut value = value;
        let c_text = CString::new(text).unwrap();
        let clicked = unsafe {
            ffi::GuiSpinner(
                bounds.into(),
                c_text.as_ptr(),
                &mut value,
                min_value,
                max_value,
                edit_mode,
            )
        };
        return (value, clicked);
    }
    /// Value Box control, updates input text with numbers
    #[inline]
    fn gui_value_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        value: i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> (i32, bool) {
        debug_assert!(
            min_value >= value && value <= max_value,
            "value out of bounds"
        );
        let mut value = value;
        let c_text = CString::new(text).unwrap();
        let clicked = unsafe {
            ffi::GuiValueBox(
                bounds.into(),
                c_text.as_ptr(),
                &mut value,
                min_value,
                max_value,
                edit_mode,
            )
        };
        return (value, clicked);
    }
    /// Text Box control, updates input text
    /// Use at your own risk!!! The allocated vector MUST have enough space for edits.
    #[inline]
    fn gui_text_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        buffer: Vec<u8>,
        edit_mode: bool,
    ) -> (Vec<u8>, bool) {
        let len = buffer.len();
        let c_text = unsafe { CString::from_vec_unchecked(buffer) };
        let clicked = unsafe {
            ffi::GuiTextBox(
                bounds.into(),
                c_text.as_ptr() as *mut _,
                len as i32,
                edit_mode,
            )
        };
        return (c_text.into_bytes(), clicked);
    }
    /// Text Box control with multiple lines
    /// Use at your own risk!!! The allocated vector MUST have a nul terminator.
    #[inline]
    fn gui_text_box_multi(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        buffer: Vec<u8>,
        edit_mode: bool,
    ) -> (Vec<u8>, bool) {
        let len = buffer.len();
        let c_text = unsafe { CString::from_vec_unchecked(buffer) };
        let clicked = unsafe {
            ffi::GuiTextBoxMulti(
                bounds.into(),
                c_text.as_ptr() as *mut _,
                len as i32,
                edit_mode,
            )
        };
        return (c_text.into_bytes(), clicked);
    }
    /// Slider control, returns selected value
    #[inline]
    fn gui_slider(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text_left: &str,
        text_right: &str,
        value: f32,
        min_value: f32,
        max_value: f32,
    ) -> f32 {
        debug_assert!(
            min_value >= value && value <= max_value,
            "value out of bounds"
        );
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
            )
        }
    }
    /// Slider Bar control, returns selected value
    #[inline]
    fn gui_slider_bar(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text_left: &str,
        text_right: &str,
        value: f32,
        min_value: f32,
        max_value: f32,
    ) -> f32 {
        debug_assert!(
            min_value >= value && value <= max_value,
            "value out of bounds"
        );
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
            )
        }
    }
    /// Progress Bar control, shows current progress value
    #[inline]
    fn gui_progress_bar(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text_left: &str,
        text_right: &str,
        value: f32,
        min_value: f32,
        max_value: f32,
    ) -> f32 {
        debug_assert!(
            min_value >= value && value <= max_value,
            "value out of bounds"
        );
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
            )
        }
    }
    /// Status Bar control, shows info text
    #[inline]
    fn gui_status_bar(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiStatusBar(bounds.into(), c_text.as_ptr()) }
    }
    /// Dummy control for placeholders
    #[inline]
    fn gui_dummy_rec(&mut self, bounds: impl Into<ffi::Rectangle>, text: &str) {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::GuiStatusBar(bounds.into(), c_text.as_ptr()) }
    }
    /// Scroll Bar control
    #[inline]
    fn gui_scroll_bar(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        value: i32,
        min_value: i32,
        max_value: i32,
    ) -> i32 {
        debug_assert!(
            min_value >= value && value <= max_value,
            "value out of bounds"
        );
        unsafe { ffi::GuiScrollBar(bounds.into(), value, min_value, max_value) }
    }
    /// Grid control
    #[inline]
    fn gui_grid(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        spacing: f32,
        subdivs: i32,
    ) -> Vector2 {
        unsafe { ffi::GuiGrid(bounds.into(), spacing, subdivs).into() }
    }
    /// List View control, returns selected list item index
    #[inline]
    fn gui_list_view(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &str,
        scroll_index: i32,
        active: i32,
    ) -> (i32, i32) {
        let mut scroll_index = scroll_index;
        let c_text = CString::new(text).unwrap();
        let scroll =
            unsafe { ffi::GuiListView(bounds.into(), c_text.as_ptr(), &mut scroll_index, active) };
        return (scroll, scroll_index);
    }
    /// List View with extended parameters
    #[inline]
    fn gui_list_view_ex(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &[&str],
        count: i32,
        focus: i32,
        scroll_index: i32,
        active: i32,
    ) -> (i32, i32, i32) {
        let mut scroll_index = scroll_index;
        let mut focus = focus;
        let mut buffer = Vec::with_capacity(text.len());
        let mut str_buffer = Vec::with_capacity(text.len());
        for t in text {
            let c_text = CString::new(*t).unwrap();
            buffer.push(c_text.as_ptr());
            str_buffer.push(c_text);
        }
        let scroll = unsafe {
            ffi::GuiListViewEx(
                bounds.into(),
                buffer.as_mut_ptr(),
                count,
                &mut focus,
                &mut scroll_index,
                active,
            )
        };
        return (scroll, focus, scroll_index);
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
        let c_title = CString::new(text).unwrap();
        let c_message = CString::new(message).unwrap();
        let c_buttons = CString::new(buttons).unwrap();

        unsafe {
            ffi::GuiMessageBox(
                bounds.into(),
                c_title.as_ptr(),
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
        mut text: Vec<u8>,
    ) -> (i32, Vec<u8>) {
        let c_title = CString::new(title).unwrap();
        let c_message = CString::new(message).unwrap();
        let c_buttons = CString::new(buttons).unwrap();
        // rgui.h: line 3699 MAX_FILENAME_LEN
        text.reserve((256 - text.len()).max(0) as usize);
        let c_text = unsafe { CString::from_vec_unchecked(text) };
        let out = unsafe {
            ffi::GuiTextInputBox(
                bounds.into(),
                c_title.as_ptr(),
                c_message.as_ptr(),
                c_buttons.as_ptr(),
                c_text.as_ptr() as *mut _,
            )
        };
        return (out, c_text.into_bytes());
    }
    /// Color Picker control
    #[inline]
    fn gui_color_picker(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        color: impl Into<ffi::Color>,
    ) -> Color {
        unsafe { ffi::GuiColorPicker(bounds.into(), color.into()).into() }
    }
}
