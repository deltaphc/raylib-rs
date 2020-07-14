// Container
// WindowBox
// GroupBox
// Line
// Panel

// Basic
// + Label
// + Button
// + LabelButton
// + ImageButton
// Toggle
// ToggleGroup
// CheckBox
// ComboBox
// DropdownBox
// TextBox
// TextBoxMulti
// ValueBox
// Spinner
// Slider
// SliderBar
// ProgressBar
// StatusBar
// ScrollPanel
// DummyRec

// Advanced
// ListView
// ColorPicker
// GuiMessageBox
// Grid

use std::ffi::CString;

use crate::core::math::Rectangle;
use crate::ffi::*;

pub enum DrawResult {
    Empty,
    Bool(bool),
    Rectangle(Rectangle),
    Scroll(Vector2, Rectangle),
    Selected(i32),
    Dropdown(bool, bool),
    Value(i32, bool),
    Text(CString, bool),
    Percentage(f32),
    ScrollBar(i32),
    // scrollindex, active, bool
    ListView(i32, bool, bool),
    // text, enabled, active, focus, scroll_index
    ListViewEx(CString, bool, bool, bool, i32),
    Color(Color),
    Grid(Vector2),
}

fn u322bool(u: u32) -> bool {
    return match u {
        0 => false,
        1 => true,
        _ => panic!("none zero or one boolean result"),
    };
}

impl From<u32> for DrawResult {
    fn from(b: u32) -> DrawResult {
        DrawResult::Bool(u322bool(b))
    }
}

impl From<i32> for DrawResult {
    fn from(selected: i32) -> DrawResult {
        DrawResult::Selected(selected)
    }
}

impl From<f32> for DrawResult {
    fn from(percentage: f32) -> DrawResult {
        DrawResult::Percentage(percentage)
    }
}

impl From<bool> for DrawResult {
    fn from(b: bool) -> DrawResult {
        DrawResult::Bool(b)
    }
}

impl From<()> for DrawResult {
    fn from(_empty: ()) -> DrawResult {
        DrawResult::Empty
    }
}

impl From<Rectangle> for DrawResult {
    fn from(rect: Rectangle) -> DrawResult {
        DrawResult::Rectangle(rect)
    }
}

pub trait GuiDraw {
    fn draw(&self) -> DrawResult;
}

macro_rules! gui_draw {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }, $draw:expr) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            $(pub $field_name: $field_type,)*
        }

        impl GuiDraw for $name{
            fn draw(&self) -> crate::rgui::DrawResult {
                $(let $field_name = &self.$field_name;)*
                unsafe {
                    $draw
                }
            }
        }

        impl GuiDraw for &$name{
            fn draw(&self) -> crate::rgui::DrawResult {
                $(let $field_name = &self.$field_name;)*
                unsafe {
                    $draw
                }
            }
        }
    };
}

gui_draw! {
    struct WindowBox {
        bounds: Rectangle,
        text: CString,
    },
    GuiWindowBox(bounds.into(), text.as_ptr()).into()
}

gui_draw! {
    struct GroupBox {
        bounds: Rectangle,
        text: CString,
    },
    GuiGroupBox(bounds.into(), text.as_ptr()).into()
}

gui_draw! {
    struct Line {
        bounds: Rectangle,
        text: CString,
    },
    GuiLine(bounds.into(), text.as_ptr()).into()
}

gui_draw! {
    struct Panel {
        bounds: Rectangle,
    },
    GuiPanel(bounds.into()).into()
}

gui_draw! {
    struct ScrollPanel {
        bounds: Rectangle,
        content: Rectangle,
        scroll: Vector2,
    },
    {
        let mut scroll = scroll.clone();
        let rect = GuiScrollPanel(bounds.into(), content.into(), &mut scroll);
        DrawResult::Scroll(scroll, rect.into())
    }
}

gui_draw! {
    struct Label {
        bounds: Rectangle,
        text: CString,
    },
    GuiLabel(bounds.into(), text.as_ptr()).into()
}

gui_draw! {
    struct Button {
        bounds: Rectangle,
        text: CString,
    },
    GuiButton(bounds.into(), text.as_ptr()).into()
}

gui_draw! {
    struct LabelButton {
        bounds: Rectangle,
        text: CString,
    },
    GuiLabelButton(bounds.into(), text.as_ptr()).into()
}

gui_draw! {
    struct ImageButton {
        bounds: Rectangle,
        texture: Texture2D,
    },
    GuiImageButton(bounds.into(), *texture).into()
}

gui_draw! {
    struct ImageButtonEx {
        bounds: Rectangle,
        texture: Texture2D,
        tex_source: Rectangle,
        text: CString,
    },
GuiImageButtonEx(
                    bounds.into(),
                    *texture,
                    tex_source.into(),
                    text.as_ptr(),
                )
                .into()
}

gui_draw! {
    struct Toggle {
        bounds: Rectangle,
        text: CString,
        active: bool,
    },
GuiToggle(
                    bounds.into(),
                    text.as_ptr(),
                    if *active { true } else { false },
                )
                .into()
}

gui_draw! {
    struct ToggleGroup {
        bounds: Rectangle,
        text: CString,
        active: bool,
    },
    GuiToggleGroup(
                    bounds.into(),
                    text.as_ptr(),
                    if *active { 1 } else { 0 },
                )
                .into()
}

gui_draw! {
    struct CheckBox {
        bounds: Rectangle,
        text: CString,
        checked: bool,
    },
    GuiCheckBox(bounds.into(), text.as_ptr(), if *checked {true} else {false}).into()
}

gui_draw! {
    struct DropDownBox {
        bounds: Rectangle,
        text: CString,
        active: bool,
        edit_mode: bool,
    },
{
                    let mut active = if *active { 1 } else { 0 };
                    let b = GuiDropdownBox(
                        bounds.into(),
                        text.as_ptr(),
                        &mut active,
                        if *edit_mode { true } else { false },
                    );
                    DrawResult::Dropdown(u322bool(active as u32), b)
                }
}

gui_draw! {
    struct Spinner {
        bounds: Rectangle,
        value: i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    },
{
                    let mut value = *value;
                    let b = GuiSpinner(
                        bounds.into(),
                        &mut value,
                        *min_value,
                        *max_value,
                        if *edit_mode { true } else { false },
                    );
                    DrawResult::Value(value, b)
                }
}

gui_draw! {
    struct TextBox {
        bounds: Rectangle,
        text: CString,
        text_size: i32,
        edit_mode: bool,
    },
{
                    let update = text.clone();
                    let b = GuiTextBox(
                        bounds.into(),
                        update.as_ptr() as *mut i8,
                        *text_size,
                        if *edit_mode { true } else { false },
                    );
                    DrawResult::Text(update, b)
                }
}

gui_draw! {
    struct TextBoxMulti {
        bounds: Rectangle,
        text: CString,
        text_size: i32,
        edit_mode: bool,
    },
{
                    let update = text.clone();
                    let b = GuiTextBoxMulti(
                        bounds.into(),
                        update.as_ptr() as *mut i8,
                        *text_size,
                        if *edit_mode { true } else { false },
                    );
                    DrawResult::Text(update, b)
                }
}

gui_draw! {
    struct ValueBox {
        bounds: Rectangle,
        value: i32,
        min_value: i32,
        max_value: i32,
        edit_mode: bool,
    },
    {
        let mut value = *value;
        let b = GuiValueBox(bounds.into(), &mut value, *min_value, *max_value, if *edit_mode {true} else {false});
        DrawResult::Value(value, b)
    }
}

gui_draw! {
    struct Slider {
        bounds: Rectangle,
        text: CString,
        valuef: f32,
        min_valuef: f32,
        max_valuef: f32,
        show_value: bool,
    },
GuiSlider(
                    bounds.into(),
                    text.as_ptr(),
                    *valuef,
                    *min_valuef,
                    *max_valuef,
                    if *show_value { true } else { false },
                )
                .into()
}

gui_draw! {
    struct SliderBar {
        bounds: Rectangle,
        text: CString,
        valuef: f32,
        min_valuef: f32,
        max_valuef: f32,
        show_value: bool,
    },
GuiSliderBar(
                    bounds.into(),
                    text.as_ptr(),
                    *valuef,
                    *min_valuef,
                    *max_valuef,
                    if *show_value { true } else { false },
                )
                .into()
}

gui_draw! {
    struct ProgressBar {
        bounds: Rectangle,
        text: CString,
        valuef: f32,
        min_valuef: f32,
        max_valuef: f32,
        show_value: bool,
    },
GuiProgressBar(
                    bounds.into(),
                    text.as_ptr(),
                    *valuef,
                    *min_valuef,
                    *max_valuef,
                    if *show_value { true } else { false },
                )
                .into()
}

gui_draw! {
    struct StatusBar {
        bounds: Rectangle,
        text: CString,
    },
    GuiStatusBar(bounds.into(), text.as_ptr()).into()
}

gui_draw! {
    struct DummyRec {
        bounds: Rectangle,
        text: CString,
    },
    GuiDummyRec(bounds.into(), text.as_ptr()).into()
}

gui_draw! {
    struct ScrollBar {
        bounds: Rectangle,
        value: i32,
        min_value: i32,
        max_value: i32,
    },
DrawResult::ScrollBar(GuiScrollBar(
                    bounds.into(),
                    *value,
                    *min_value,
                    *max_value,
                ))
}

gui_draw! {
    struct ListView {
        bounds: Rectangle,
        text: CString,
        active: bool,
        scroll_index: i32,
        edit_mode: bool,
    },
{
                    let mut scroll_index = *scroll_index;
                    let mut active = if *active { 1 } else { 0 };
                    let b = GuiListView(
                        bounds.into(),
                        text.as_ptr(),
                        &mut active,
                        &mut scroll_index,
                        if *edit_mode { true } else { false },
                    );
                    DrawResult::ListView(scroll_index, u322bool(active as u32), b)
                }
}

gui_draw! {
    struct ListViewEx {
        bounds: Rectangle,
        text: CString,
        count: i32,
        enabled: bool,
        active: bool,
        focus: bool,
        scroll_index: i32,
        edit_mode: bool,
    },
{
                    let mut current_text = text.clone();
                    let current_text_ptr = current_text.as_ptr() as *mut i8;
                    let next_text = current_text_ptr;
                    let mut enabled = if *enabled { 1 } else { 0 };
                    let mut active = if *active { 1 } else { 0 };
                    let mut focus = if *focus { 1 } else { 0 };
                    let mut scroll_index = *scroll_index;
                    let _b = GuiListViewEx(
                        bounds.into(),
                        &mut (next_text as *const i8),
                        *count,
                        &mut enabled,
                        &mut active,
                        &mut focus,
                        &mut scroll_index,
                        if *edit_mode { true } else { false },
                    );
                    current_text = CString::from_raw(next_text);
                    DrawResult::ListViewEx(
                        current_text,
                        u322bool(enabled as u32),
                        u322bool(active as u32),
                        u322bool(focus as u32),
                        scroll_index,
                    )
                }
}

gui_draw! {
    struct MessageBox {
        bounds: Rectangle,
        window_title: CString,
        message: CString,
        buttons: CString,
    },
DrawResult::Selected(GuiMessageBox(
                    bounds.into(),
                    window_title.as_ptr(),
                    message.as_ptr(),
                    buttons.as_ptr(),
                ))
}

gui_draw! {
    struct ColorPicker {
        bounds: Rectangle,
        color: Color,
    },
    DrawResult::Color(GuiColorPicker(bounds.into(), *color))
}

gui_draw! {
    struct Grid {
        bounds: Rectangle,
        spacing: f32,
        subdivs: i32,
    },
    DrawResult::Grid(GuiGrid(bounds.into(), *spacing, *subdivs))
}
