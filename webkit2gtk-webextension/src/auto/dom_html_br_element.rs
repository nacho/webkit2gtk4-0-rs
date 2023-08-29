// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMElement, DOMEventTarget, DOMHTMLElement, DOMNode, DOMObject};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLBRElement")]
    pub struct DOMHTMLBRElement(Object<ffi::WebKitDOMHTMLBRElement, ffi::WebKitDOMHTMLBRElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_br_element_get_type(),
    }
}

impl DOMHTMLBRElement {
    pub const NONE: Option<&'static DOMHTMLBRElement> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMHTMLBRElement>> Sealed for T {}
}

pub trait DOMHTMLBRElementExt: IsA<DOMHTMLBRElement> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_br_element_get_clear")]
    #[doc(alias = "get_clear")]
    fn clear(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_br_element_get_clear(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_br_element_set_clear")]
    fn set_clear(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_br_element_set_clear(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "clear")]
    fn connect_clear_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_clear_trampoline<
            P: IsA<DOMHTMLBRElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLBRElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLBRElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::clear\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_clear_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DOMHTMLBRElement>> DOMHTMLBRElementExt for O {}

impl fmt::Display for DOMHTMLBRElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLBRElement")
    }
}
