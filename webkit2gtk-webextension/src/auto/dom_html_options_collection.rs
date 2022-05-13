// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMHTMLCollection;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLOptionsCollection")]
    pub struct DOMHTMLOptionsCollection(Object<ffi::WebKitDOMHTMLOptionsCollection, ffi::WebKitDOMHTMLOptionsCollectionClass>) @extends DOMHTMLCollection, DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_html_options_collection_get_type(),
    }
}

impl DOMHTMLOptionsCollection {
    pub const NONE: Option<&'static DOMHTMLOptionsCollection> = None;
}

pub trait DOMHTMLOptionsCollectionExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_options_collection_get_selected_index")]
    #[doc(alias = "get_selected_index")]
    fn selected_index(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_options_collection_set_selected_index")]
    fn set_selected_index(&self, value: libc::c_long);

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "selected-index")]
    fn connect_selected_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLOptionsCollection>> DOMHTMLOptionsCollectionExt for O {
    fn selected_index(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_options_collection_get_selected_index(
                self.as_ref().to_glib_none().0,
            )
        }
    }

    fn set_selected_index(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_options_collection_set_selected_index(
                self.as_ref().to_glib_none().0,
                value,
            );
        }
    }

    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<DOMHTMLOptionsCollection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLOptionsCollection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLOptionsCollection::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_selected_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_index_trampoline<
            P: IsA<DOMHTMLOptionsCollection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLOptionsCollection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLOptionsCollection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected-index\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_index_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLOptionsCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLOptionsCollection")
    }
}
