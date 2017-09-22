use js::jsapi;
use js::jsapi::JSClass;
use js::jsapi::JSClassOps;
use js::jsapi::JSNativeWrapper;
use js::jsapi::JSPropertySpec;
use js::jsapi::Value;

use libc::c_char;
use libc::c_uint;

use linjs::CanAccess;
use linjs::CanAlloc;
use linjs::HasGlobal;
use linjs::InCompartment;
use linjs::JSContext;
use linjs::JSInitializer;
use linjs::null_property;
use linjs::null_wrapper;
use linjs::jsclass_global_flags_with_slots;

use minidom::Console;
use minidom::Document;
use minidom::Window;
use minidom::WindowClass;

use std::ptr;

// In a real example, this code would be produced by a codegen tool
// from webidl. For the moment, we do it by hand.

#[allow(non_snake_case)]
pub trait WindowMethods {
    fn Console<'a, C, S>(cx: &'a mut JSContext<S>, this: Window<'a, C>) -> Console<'a, C> where
        S: 'a + CanAccess + CanAlloc + InCompartment<C>,
        C: 'a + HasGlobal<WindowClass>;

    fn Document<'a, C, S>(cx: &'a mut JSContext<S>, this: Window<'a, C>) -> Document<'a, C> where
        S: 'a + CanAccess + CanAlloc + InCompartment<C>,
        C: 'a + HasGlobal<WindowClass>;
}

static WINDOW_CLASS: JSClass = JSClass {
    name: b"Window\0" as *const u8 as *const c_char,
    flags: jsclass_global_flags_with_slots(1),
    cOps: &JSClassOps {
        addProperty: None,
        call: None,
        construct: None,
        delProperty: None,
        enumerate: None,
        finalize: None,
        getProperty: None,
        hasInstance: None,
        mayResolve: None,
        resolve: None,
        setProperty: None,
        trace: None,
    },
    reserved: [0 as *mut _; 3],
};

const WINDOW_PROPERTIES: &[JSPropertySpec] = &[
    JSPropertySpec {
        name: b"console\0" as *const u8 as *const c_char,
        flags: 0,
        getter: JSNativeWrapper {
            op: Some(window_console_getter),
            info: ptr::null(),
        },
        setter: null_wrapper(),
    },
    JSPropertySpec {
        name: b"document\0" as *const u8 as *const c_char,
        flags: 0,
        getter: JSNativeWrapper {
            op: Some(window_document_getter),
            info: ptr::null(),
        },
        setter: null_wrapper(),
    },
    null_property(),
];

// Just stub methods for now.

#[allow(unsafe_code)]
unsafe extern "C" fn window_console_getter(_cx: *mut jsapi::JSContext, _argc: c_uint, _vp: *mut Value) -> bool {
    // let args = CallArgs::from_vp(vp, argc);
    // let window = args.thisv();
    // let ref mut cx = JSContext::from(cx);
    // let result = WindowClass::Console(cx, window);
    // args.rval().set(result);
    true
}

#[allow(unsafe_code)]
unsafe extern "C" fn window_document_getter(_cx: *mut jsapi::JSContext, _argc: c_uint, _vp: *mut Value) -> bool {
    // let args = CallArgs::from_vp(vp, argc);
    // let window = args.thisv();
    // let ref mut cx = JSContext::from(cx);
    // let result = WindowClass::Document(cx, window);
    // args.rval().set(result);
    true
}

pub struct WindowInitializer;

impl JSInitializer for WindowInitializer {
    #[allow(unsafe_code)]
    unsafe fn global_classp() -> *const JSClass {
        &WINDOW_CLASS
    }

    #[allow(unsafe_code)]
    unsafe fn properties() -> *const JSPropertySpec {
        &WINDOW_PROPERTIES[0]
    }
}
