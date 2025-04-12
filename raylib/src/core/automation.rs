use std::{
    ffi::CString,
    path::{Path, PathBuf},
    ptr::null,
};

use crate::{ffi, RaylibHandle};

#[derive(Debug, Clone)]
pub struct AutomationEventIter<'a> {
    iter: std::slice::Iter<'a, ffi::AutomationEvent>
}
impl<'a> AutomationEventIter<'a> {
    unsafe fn new(events: *mut ffi::AutomationEvent, count: u32) -> Self {
        // No new items are being created that get dropped here, these are just changes in perspective of how to borrow-check the pointers.
        assert!(!events.is_null(), "automation event array cannot be null");
        assert!(events.is_aligned(), "automation event array must be aligned");
        let iter = unsafe { std::slice::from_raw_parts(events, count as usize) }.iter();
        Self { iter }
    }
    fn func(e: &ffi::AutomationEvent) -> AutomationEvent {
        // This relies on the fact that `ffi::AutomationEvent` is Copy `unload_automation_event` doesn't actually do anything.
        AutomationEvent(*e)
    }
}
impl<'a> Iterator for AutomationEventIter<'a> {
    type Item = AutomationEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Self::func)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    fn last(self) -> Option<Self::Item> {
        self.iter.last().map(Self::func)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(Self::func)
    }
}
impl<'a> DoubleEndedIterator for AutomationEventIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Self::func)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Self::func)
    }
}
impl<'a> ExactSizeIterator for AutomationEventIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

make_thin_wrapper!(
    AutomationEventList,
    ffi::AutomationEventList,
    ffi::UnloadAutomationEventList,
    false
);

impl AutomationEventList {
    /// Length of the automation event list
    #[inline]
    pub const fn count(&self) -> u32 {
        self.0.count
    }
    /// The amount of automation events that can be held in this list.
    #[inline]
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
    /// An iterator over the events held in this list.
    pub fn iter<'a>(&'a self) -> AutomationEventIter<'a> {
        unsafe { AutomationEventIter::new(self.0.events, self.count()) }
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
    /// Event frame
    #[inline]
    pub const fn frame(&self) -> u32 {
        self.0.frame
    }
    /// Event type (AutomationEventType)
    #[inline]
    pub const fn get_type(&self) -> u32 {
        self.0.type_
    }
    /// Event parameters (if required)
    #[inline]
    pub const fn params(&self) -> [i32; 4] {
        self.0.params
    }
}

impl AutomationEvent {
    /// Play a recorded automation event
    #[inline]
    pub fn play(&self) {
        unsafe { ffi::PlayAutomationEvent(self.0) }
    }
}

fn unload_automation_event(_s: ffi::AutomationEvent) {
    // As far as I can tell, this is actually unloaded when UnloadAnimationEventList is called.
}

impl RaylibHandle {
    /// Load automation events list from file, NULL for empty list, capacity = MAX_AUTOMATION_EVENTS
    pub fn load_automation_event_list(&self, file_name: Option<PathBuf>) -> AutomationEventList {
        match file_name {
            Some(a) => {
                let c_str = CString::new(a.to_string_lossy().as_bytes()).unwrap();
                AutomationEventList(unsafe { ffi::LoadAutomationEventList(c_str.as_ptr()) })
            }
            None => AutomationEventList(unsafe { ffi::LoadAutomationEventList(null()) }),
        }
    }
    /// Set automation event list to record to
    #[inline]
    pub fn set_automation_event_list(&self, l: &mut AutomationEventList) {
        unsafe {
            ffi::SetAutomationEventList(&mut l.0 as *mut ffi::AutomationEventList);
        }
    }
    /// Set automation event internal base frame to start recording
    #[inline]
    pub fn set_automation_event_base_frame(&self, b: i32) {
        unsafe { ffi::SetAutomationEventBaseFrame(b) };
    }
    /// Start recording automation events (AutomationEventList must be set)
    #[inline]
    pub fn start_automation_event_recording(&self) {
        unsafe { ffi::StartAutomationEventRecording() };
    }
    /// Stop recording automation events
    #[inline]
    pub fn stop_automation_event_recording(&self) {
        unsafe { ffi::StopAutomationEventRecording() };
    }
}
