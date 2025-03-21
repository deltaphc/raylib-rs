use std::{
    ffi::CString,
    path::{Path, PathBuf},
    ptr::null,
};

use crate::{ffi, RaylibHandle};

make_thin_wrapper!(
    AutomationEventList,
    ffi::AutomationEventList,
    ffi::UnloadAutomationEventList,
    false
);

impl AutomationEventList {
    /// Length of the automation event list
    pub const fn count(&self) -> u32 {
        self.0.count
    }
    /// The amount of automation events that can be held in this list.
    pub const fn capacity(&self) -> u32 {
        self.0.capacity
    }
    /// The events held in this list.
    /// NOTE: This will copy the values into a vector.
    pub fn events(&self) -> Vec<AutomationEvent> {
        unsafe { std::slice::from_raw_parts(self.0.events, self.count() as usize) }
            .iter()
            .map(|f| AutomationEvent(*f))
            .collect()
    }

    /// Export automation events list as text file
    pub fn export(&self, file_name: impl AsRef<Path>) -> bool {
        let c_str = CString::new(file_name.as_ref().to_string_lossy().as_bytes()).unwrap();
        unsafe { ffi::ExportAutomationEventList(self.0, c_str.as_ptr()) }
    }
}

make_thin_wrapper!(
    AutomationEvent,
    ffi::AutomationEvent,
    unload_automation_event,
    false
);

impl AutomationEvent {
    pub const fn frame(&self) -> u32 {
        self.0.frame
    }
    pub const fn get_type(&self) -> u32 {
        self.0.type_
    }
    pub const fn params(&self) -> [i32; 4] {
        self.0.params
    }
}

impl AutomationEvent {
    pub fn play(&self) {
        unsafe { ffi::PlayAutomationEvent(self.0) }
    }
}

fn unload_automation_event(_s: ffi::AutomationEvent) {
    // As far as I can tell, this is actually unloaded when UnloadAnimationEventList is called.
}

impl RaylibHandle {
    pub fn load_automation_event_list(&self, file_name: Option<PathBuf>) -> AutomationEventList {
        match file_name {
            Some(a) => {
                let c_str = CString::new(a.to_string_lossy().as_bytes()).unwrap();
                AutomationEventList(unsafe { ffi::LoadAutomationEventList(c_str.as_ptr()) })
            }
            None => AutomationEventList(unsafe { ffi::LoadAutomationEventList(null()) }),
        }
    }
    pub fn set_automation_event_list(&self, l: &mut AutomationEventList) {
        unsafe {
            ffi::SetAutomationEventList(&mut l.0 as *mut ffi::AutomationEventList);
        }
    }
    pub fn set_automation_event_base_frame(&self, b: i32) {
        unsafe { ffi::SetAutomationEventBaseFrame(b) };
    }
    pub fn start_automation_event_recording(&self) {
        unsafe { ffi::StartAutomationEventRecording() };
    }
    pub fn stop_automation_event_recording(&self) {
        unsafe { ffi::StopAutomationEventRecording() };
    }
}
