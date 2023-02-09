// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMCharacterData, DOMEventTarget, DOMNode, DOMObject};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitDOMText")]
    pub struct DOMText(Object<ffi::WebKitDOMText, ffi::WebKitDOMTextClass>) @extends DOMCharacterData, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_text_get_type(),
    }
}

impl DOMText {
    pub const NONE: Option<&'static DOMText> = None;
}

pub trait DOMTextExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_text_get_whole_text")]
    #[doc(alias = "get_whole_text")]
    fn whole_text(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_text_replace_whole_text")]
    fn replace_whole_text(&self, content: &str) -> Result<DOMText, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_text_split_text")]
    fn split_text(&self, offset: libc::c_ulong) -> Result<DOMText, glib::Error>;

    #[doc(alias = "whole-text")]
    fn connect_whole_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMText>> DOMTextExt for O {
    #[allow(deprecated)]
    fn whole_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_text_get_whole_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn replace_whole_text(&self, content: &str) -> Result<DOMText, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_text_replace_whole_text(
                self.as_ref().to_glib_none().0,
                content.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[allow(deprecated)]
    fn split_text(&self, offset: libc::c_ulong) -> Result<DOMText, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::webkit_dom_text_split_text(self.as_ref().to_glib_none().0, offset, &mut error);
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_whole_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_whole_text_trampoline<P: IsA<DOMText>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMText,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMText::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::whole-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_whole_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMText")
    }
}
