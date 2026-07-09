use std::ffi::{c_char, c_void};

use objc2_core_foundation::CFType;

type Boolean = u8;
type CFIndex = isize;
type OSStatus = i32;
type CFStringEncoding = u32;
type TISInputSourceRef = *const c_void;

const K_CFSTRING_ENCODING_UTF8: CFStringEncoding = 0x0800_0100;

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    static kTISPropertyInputSourceID: *const c_void;

    fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
    fn TISCreateInputSourceList(
        properties: *const c_void,
        includeAllInstalled: Boolean,
    ) -> *const c_void;
    fn TISGetInputSourceProperty(
        inputSource: TISInputSourceRef,
        propertyKey: *const c_void,
    ) -> *const c_void;
    fn TISSelectInputSource(inputSource: TISInputSourceRef) -> OSStatus;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFArrayGetCount(theArray: *const c_void) -> CFIndex;
    fn CFArrayGetValueAtIndex(theArray: *const c_void, idx: CFIndex) -> *const c_void;
    fn CFRelease(cf: *mut CFType);
    fn CFStringGetCString(
        theString: *const c_void,
        buffer: *mut c_char,
        bufferSize: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
}

pub fn current_input_source_id() -> Option<String> {
    let source = unsafe { TISCopyCurrentKeyboardInputSource() };
    if source.is_null() {
        return None;
    }

    let id = input_source_id(source);
    unsafe { CFRelease(source.cast_mut().cast()) };
    id
}

pub fn select_input_source(id: &str) -> Result<(), String> {
    let sources = unsafe { TISCreateInputSourceList(std::ptr::null(), false as Boolean) };
    if sources.is_null() {
        return Err("TISCreateInputSourceList returned null".to_string());
    }

    let result = select_input_source_from_list(sources, id);
    unsafe { CFRelease(sources.cast_mut().cast()) };
    result
}

fn select_input_source_from_list(sources: *const c_void, id: &str) -> Result<(), String> {
    let count = unsafe { CFArrayGetCount(sources) };

    for idx in 0..count {
        let source = unsafe { CFArrayGetValueAtIndex(sources, idx) };
        if source.is_null() {
            continue;
        }

        if input_source_id(source).as_deref() == Some(id) {
            let status = unsafe { TISSelectInputSource(source) };
            return if status == 0 {
                Ok(())
            } else {
                Err(format!("TISSelectInputSource failed with status {status}"))
            };
        }
    }

    Err(format!("input source not found: {id}"))
}

fn input_source_id(source: TISInputSourceRef) -> Option<String> {
    let id_ref = unsafe { TISGetInputSourceProperty(source, kTISPropertyInputSourceID) };
    if id_ref.is_null() {
        return None;
    }

    cf_string_to_string(id_ref)
}

fn cf_string_to_string(value: *const c_void) -> Option<String> {
    let mut buffer = vec![0u8; 1024];
    let ok = unsafe {
        CFStringGetCString(
            value,
            buffer.as_mut_ptr().cast(),
            buffer.len() as CFIndex,
            K_CFSTRING_ENCODING_UTF8,
        )
    };

    if ok == 0 {
        return None;
    }

    let nul = buffer.iter().position(|&byte| byte == 0)?;
    Some(String::from_utf8_lossy(&buffer[..nul]).into_owned())
}
