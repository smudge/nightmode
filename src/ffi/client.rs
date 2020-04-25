use crate::ffi::BlueLightStatus;
use objc::rc::StrongPtr;
use objc::runtime::{Object, BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};

pub struct CBBlueLightClient {
    inner: StrongPtr,
}

impl CBBlueLightClient {
    pub fn new() -> CBBlueLightClient {
        let client_class = class!(CBBlueLightClient);
        let client = unsafe {
            let obj: *mut Object = msg_send![client_class, alloc];
            let obj: *mut Object = msg_send![obj, init];
            StrongPtr::new(obj)
        };
        CBBlueLightClient { inner: client }
    }

    pub fn set_enabled(&self, enabled: bool) -> Result<(), String> {
        let result: BOOL = unsafe { msg_send![*self.inner, setEnabled: (enabled as BOOL)] };
        if result == YES {
            Ok(())
        } else {
            Err(format!("Failed to turn Night Shift {}", on_or_off(enabled)))
        }
    }

    pub fn set_strength(&self, strength: f32) -> Result<(), String> {
        let result: BOOL = unsafe { msg_send![*self.inner, setStrength:strength commit:YES] };

        if result == YES {
            Ok(())
        } else {
            Err("Failed to set color temperature".to_string())
        }
    }

    pub fn get_strength(&self) -> Result<i32, String> {
        let mut value: f32 = -1.0;
        let result: BOOL = unsafe { msg_send![*self.inner, getStrength: &mut value] };

        if result == YES && value >= 0.0 {
            Ok((value * 100.0) as i32)
        } else {
            Err("Failed to get color temperature".to_string())
        }
    }

    pub fn status(&self) -> Result<BlueLightStatus, String> {
        let mut ptr = BlueLightStatus::c_ptr();
        let result: BOOL = unsafe { msg_send![*self.inner, getBlueLightStatus: &mut ptr] };
        if result == YES {
            Ok(BlueLightStatus::new(ptr))
        } else {
            Err("Failed to get status".to_string())
        }
    }
}

fn on_or_off(value: bool) -> String {
    if value {
        "on".to_string()
    } else {
        "off".to_string()
    }
}