// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::DOMEvent;
use glib::{prelude::*, translate::*};
use std::{fmt, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitDOMEventTarget")]
    pub struct DOMEventTarget(Interface<ffi::WebKitDOMEventTarget, ffi::WebKitDOMEventTargetIface>);

    match fn {
        type_ => || ffi::webkit_dom_event_target_get_type(),
    }
}

impl DOMEventTarget {
    pub const NONE: Option<&'static DOMEventTarget> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMEventTarget>> Sealed for T {}
}

pub trait DOMEventTargetExt: IsA<DOMEventTarget> + sealed::Sealed + 'static {
    //#[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    //#[allow(deprecated)]
    //#[doc(alias = "webkit_dom_event_target_add_event_listener")]
    //fn add_event_listener<P: FnOnce() + 'static>(&self, event_name: &str, handler: P, use_capture: bool, user_data: /*Unimplemented*/Option<Basic: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:webkit_dom_event_target_add_event_listener() }
    //}

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_event_target_add_event_listener_with_closure")]
    fn add_event_listener_with_closure(
        &self,
        event_name: &str,
        handler: &glib::Closure,
        use_capture: bool,
    ) -> bool {
        unsafe {
            from_glib(
                ffi::webkit_dom_event_target_add_event_listener_with_closure(
                    self.as_ref().to_glib_none().0,
                    event_name.to_glib_none().0,
                    handler.to_glib_none().0,
                    use_capture.into_glib(),
                ),
            )
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_event_target_dispatch_event")]
    fn dispatch_event(&self, event: &impl IsA<DOMEvent>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::webkit_dom_event_target_dispatch_event(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    //#[allow(deprecated)]
    //#[doc(alias = "webkit_dom_event_target_remove_event_listener")]
    //fn remove_event_listener<P: FnMut()>(&self, event_name: &str, handler: P, use_capture: bool) -> bool {
    //    unsafe { TODO: call ffi:webkit_dom_event_target_remove_event_listener() }
    //}

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_event_target_remove_event_listener_with_closure")]
    fn remove_event_listener_with_closure(
        &self,
        event_name: &str,
        handler: &glib::Closure,
        use_capture: bool,
    ) -> bool {
        unsafe {
            from_glib(
                ffi::webkit_dom_event_target_remove_event_listener_with_closure(
                    self.as_ref().to_glib_none().0,
                    event_name.to_glib_none().0,
                    handler.to_glib_none().0,
                    use_capture.into_glib(),
                ),
            )
        }
    }
}

impl<O: IsA<DOMEventTarget>> DOMEventTargetExt for O {}

impl fmt::Display for DOMEventTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMEventTarget")
    }
}
