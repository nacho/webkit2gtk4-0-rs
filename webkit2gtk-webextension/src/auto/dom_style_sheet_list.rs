// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMObject, DOMStyleSheet};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitDOMStyleSheetList")]
    pub struct DOMStyleSheetList(Object<ffi::WebKitDOMStyleSheetList, ffi::WebKitDOMStyleSheetListClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_style_sheet_list_get_type(),
    }
}

impl DOMStyleSheetList {
    pub const NONE: Option<&'static DOMStyleSheetList> = None;
}

pub trait DOMStyleSheetListExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_style_sheet_list_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_style_sheet_list_item")]
    fn item(&self, index: libc::c_ulong) -> Option<DOMStyleSheet>;

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMStyleSheetList>> DOMStyleSheetListExt for O {
    #[allow(deprecated)]
    fn length(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_style_sheet_list_get_length(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn item(&self, index: libc::c_ulong) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_list_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<DOMStyleSheetList>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMStyleSheetList,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMStyleSheetList::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMStyleSheetList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMStyleSheetList")
    }
}
