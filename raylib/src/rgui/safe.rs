use crate::core::color::Color;
use crate::core::drawing::RaylibDraw;
use crate::core::math::{Rectangle, Vector2};
use crate::core::text::WeakFont;
use crate::core::RaylibHandle;
use crate::ffi;

use std::ffi::{CStr, CString};

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
    pub fn gui_fade(&mut self, alpha: f32) {
        unsafe { ffi::GuiFade(alpha) }
    }
    /// Set gui state (global state)
    #[inline]
    pub fn gui_set_state(&mut self, state: crate::consts::GuiControlState) {
        unsafe { ffi::GuiSetState(state as i32) }
    }
    /// Get gui state (global state)
    #[inline]
    pub fn gui_get_state(&mut self) -> crate::consts::GuiControlState {
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
    pub fn gui_load_style(&mut self, filename: Option<&CStr>) {
        unsafe { ffi::GuiLoadStyle(filename.map(CStr::as_ptr).unwrap_or(std::ptr::null())) }
    }
    /// Load style properties from array
    #[inline]
    pub fn gui_load_style_props(&mut self, props: &[crate::consts::GuiControlProperty]) {
        unsafe { ffi::GuiLoadStyleProps(props.as_ptr() as *const _, props.len() as i32) }
    }
    /// Load style default over global style
    #[inline]
    pub fn gui_load_style_default(&mut self) {
        unsafe { ffi::GuiLoadStyleDefault() }
    }
    /// Updates full style properties set with default values
    #[inline]
    pub fn gui_update_style_complete(&mut self) {
        unsafe { ffi::GuiUpdateStyleComplete() }
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
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    fn gui_set_style(&mut self, control: crate::consts::GuiControl, property: i32, value: i32) {
        unsafe { ffi::GuiSetStyle(control as i32, property as i32, value) }
    }

    /// Get one style property
    /// SHOULD use one of the Gui*Property enums
    #[inline]
    fn gui_get_style(&mut self, control: crate::consts::GuiControl, property: i32) -> i32 {
        unsafe { ffi::GuiGetStyle(control as i32, property as i32) }
    }
    /// Load style file (.rgs)
    #[inline]
    fn gui_load_style(&mut self, filename: Option<&CStr>) {
        unsafe { ffi::GuiLoadStyle(filename.map(CStr::as_ptr).unwrap_or(std::ptr::null())) }
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
    /// Window Box control, shows a window that can be closed
    #[inline]
    fn gui_window_box(&mut self, bounds: impl Into<ffi::Rectangle>, title: Option<&CStr>) -> bool {
        unsafe {
            ffi::GuiWindowBox(
                bounds.into(),
                title.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        }
    }
    /// Group Box control with text name
    #[inline]
    fn gui_group_box(&mut self, bounds: impl Into<ffi::Rectangle>, text: Option<&CStr>) {
        unsafe {
            ffi::GuiGroupBox(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        }
    }
    /// Line separator control, could contain text
    #[inline]
    fn gui_line(&mut self, bounds: impl Into<ffi::Rectangle>, text: Option<&CStr>) {
        unsafe {
            ffi::GuiLine(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        }
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
    fn gui_label(&mut self, bounds: impl Into<ffi::Rectangle>, text: Option<&CStr>) {
        unsafe {
            ffi::GuiLabel(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        }
    }
    /// Button control, returns true when clicked
    #[inline]
    fn gui_button(&mut self, bounds: impl Into<ffi::Rectangle>, text: Option<&CStr>) -> bool {
        unsafe {
            ffi::GuiButton(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        }
    }
    /// Label button control, show true when clicked
    #[inline]
    fn gui_label_button(&mut self, bounds: impl Into<ffi::Rectangle>, text: Option<&CStr>) -> bool {
        unsafe {
            ffi::GuiLabelButton(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        }
    }
    /// Image button control, returns true when clicked
    #[inline]
    fn gui_image_button(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: Option<&CStr>,
        texture: impl AsRef<ffi::Texture>,
    ) -> bool {
        unsafe {
            ffi::GuiImageButton(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                *texture.as_ref(),
            )
        }
    }
    /// Image button extended control, returns true when clicked
    #[inline]
    fn gui_image_button_ex(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: Option<&CStr>,
        texture: impl AsRef<ffi::Texture>,
        tex_source: impl Into<ffi::Rectangle>,
    ) -> bool {
        unsafe {
            ffi::GuiImageButtonEx(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                *texture.as_ref(),
                tex_source.into(),
            )
        }
    }
    /// Toggle Button control, returns true when active
    #[inline]
    fn gui_toggle(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: Option<&CStr>,
        active: bool,
    ) -> bool {
        unsafe {
            ffi::GuiToggle(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                active,
            )
        }
    }
    /// Toggle Group control, returns active toggle index
    #[inline]
    fn gui_toggle_group(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: Option<&CStr>,
        active: i32,
    ) -> i32 {
        unsafe {
            ffi::GuiToggleGroup(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                active,
            )
        }
    }
    /// Check Box control, returns true when active
    #[inline]
    fn gui_check_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: Option<&CStr>,
        checked: bool,
    ) -> bool {
        unsafe {
            ffi::GuiCheckBox(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                checked,
            )
        }
    }
    /// Combo Box control, returns selected item index
    #[inline]
    fn gui_combo_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: Option<&CStr>,
        active: i32,
    ) -> i32 {
        unsafe {
            ffi::GuiComboBox(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                active,
            )
        }
    }
    /// Dropdown Box control, returns selected item
    #[inline]
    fn gui_dropdown_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: Option<&CStr>,
        active: &mut i32,
        edit_mode: bool,
    ) -> bool {
        unsafe {
            ffi::GuiDropdownBox(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                active,
                edit_mode,
            )
        }
    }
    /// Spinner control, returns selected value
    #[inline]
    fn gui_spinner(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: Option<&CStr>,
        value: &mut i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> bool {
        debug_assert!(
            min_value >= *value && *value <= max_value,
            "value out of bounds"
        );
        let clicked = unsafe {
            ffi::GuiSpinner(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                value,
                min_value,
                max_value,
                edit_mode,
            )
        };
        return clicked;
    }
    /// Value Box control, updates input text with numbers
    #[inline]
    fn gui_value_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: Option<&CStr>,
        value: &mut i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    ) -> bool {
        debug_assert!(
            min_value >= *value && *value <= max_value,
            "value out of bounds"
        );
        unsafe {
            ffi::GuiValueBox(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
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
            )
        }
    }
    /// Text Box control with multiple lines
    /// Use at your own risk!!! The allocated vector MUST have a nul terminator.
    #[inline]
    fn gui_text_box_multi(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        buffer: &mut [u8],
        edit_mode: bool,
    ) -> bool {
        let len = buffer.len();
        let c_text = unsafe { CStr::from_bytes_with_nul_unchecked(buffer) };
        unsafe {
            ffi::GuiTextBoxMulti(
                bounds.into(),
                c_text.as_ptr() as *mut _,
                len as i32,
                edit_mode,
            )
        }
    }
    /// Slider control, returns selected value
    #[inline]
    fn gui_slider(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text_left: Option<&CStr>,
        text_right: Option<&CStr>,
        value: f32,
        min_value: f32,
        max_value: f32,
    ) -> f32 {
        debug_assert!(
            min_value >= value && value <= max_value,
            "value out of bounds"
        );
        unsafe {
            ffi::GuiSlider(
                bounds.into(),
                text_left.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                text_right.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
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
        text_left: Option<&CStr>,
        text_right: Option<&CStr>,
        value: f32,
        min_value: f32,
        max_value: f32,
    ) -> f32 {
        debug_assert!(
            min_value >= value && value <= max_value,
            "value out of bounds"
        );
        unsafe {
            ffi::GuiSliderBar(
                bounds.into(),
                text_left.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                text_right.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
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
        text_left: Option<&CStr>,
        text_right: Option<&CStr>,
        value: f32,
        min_value: f32,
        max_value: f32,
    ) -> f32 {
        debug_assert!(
            min_value >= value && value <= max_value,
            "value out of bounds"
        );
        unsafe {
            ffi::GuiProgressBar(
                bounds.into(),
                text_left.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                text_right.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                value,
                min_value,
                max_value,
            )
        }
    }
    /// Status Bar control, shows info text
    #[inline]
    fn gui_status_bar(&mut self, bounds: impl Into<ffi::Rectangle>, text: Option<&CStr>) {
        unsafe {
            ffi::GuiStatusBar(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        }
    }
    /// Dummy control for placeholders
    #[inline]
    fn gui_dummy_rec(&mut self, bounds: impl Into<ffi::Rectangle>, text: Option<&CStr>) {
        unsafe {
            ffi::GuiStatusBar(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        }
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
        text: Option<&CStr>,
        scroll_index: &mut i32,
        active: i32,
    ) -> i32 {
        unsafe {
            ffi::GuiListView(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                scroll_index,
                active,
            )
        }
    }
    /// List View with extended parameters
    #[inline]
    fn gui_list_view_ex(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        text: &[&CStr],
        count: i32,
        focus: &mut i32,
        scroll_index: &mut i32,
        active: i32,
    ) -> i32 {
        let mut buffer = Vec::with_capacity(text.len());
        for t in text {
            buffer.push(t.as_ptr());
        }
        unsafe {
            ffi::GuiListViewEx(
                bounds.into(),
                buffer.as_mut_ptr(),
                count,
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
        text: Option<&CStr>,
        message: Option<&CStr>,
        buttons: Option<&CStr>,
    ) -> i32 {
        unsafe {
            ffi::GuiMessageBox(
                bounds.into(),
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                message.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                buttons.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        }
    }
    /// Text Input Box control, ask for text
    #[inline]
    fn gui_text_input_box(
        &mut self,
        bounds: impl Into<ffi::Rectangle>,
        title: Option<&CStr>,
        message: Option<&CStr>,
        buttons: Option<&CStr>,
        text: &mut Vec<u8>,
    ) -> i32 {
        // rgui.h: line 3699 MAX_FILENAME_LEN
        text.reserve((256 - text.len()).max(0) as usize);
        unsafe {
            ffi::GuiTextInputBox(
                bounds.into(),
                title.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                message.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                buttons.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
                text.as_mut_ptr() as *mut _,
            )
        }
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
    // Get text with icon id prepended
    // NOTE: Useful to add icons by name id (enum) instead of
    // a number that can change between ricon versions
    #[inline]
    unsafe fn gui_icon_text(
        &mut self,
        icon_id: crate::consts::rIconDescription,
        text: Option<&CStr>,
    ) -> CString {
        let buffer = unsafe {
            ffi::GuiIconText(
                icon_id as i32,
                text.map(CStr::as_ptr).unwrap_or(std::ptr::null()),
            )
        };
        // TODO This is probably 100% unsafe. buffer is 1024 and I'm not sure if freeing this will result in a segfault.
        // I don't want to do the copy allocation, but I'll probably have to.
        CString::from_raw(buffer as *mut i8)
    }

    /// Color Bar Alpha control
    /// NOTE: Returns alpha value normalized [0..1]
    #[inline]
    fn gui_color_bar_alpha(&mut self, bounds: impl Into<ffi::Rectangle>, alpha: f32) -> f32 {
        unsafe { ffi::GuiColorBarAlpha(bounds.into(), alpha).into() }
    }
}
