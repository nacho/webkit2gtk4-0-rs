// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMHTMLCollection, DOMNode, DOMObject};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

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

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMHTMLOptionsCollection>> Sealed for T {}
}

pub trait DOMHTMLOptionsCollectionExt:
    IsA<DOMHTMLOptionsCollection> + sealed::Sealed + 'static
{
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_options_collection_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_html_options_collection_get_length(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_options_collection_get_selected_index")]
    #[doc(alias = "get_selected_index")]
    fn selected_index(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_options_collection_get_selected_index(
                self.as_ref().to_glib_none().0,
            )
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_options_collection_named_item")]
    fn named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_options_collection_named_item(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_html_options_collection_set_selected_index")]
    fn set_selected_index(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_options_collection_set_selected_index(
                self.as_ref().to_glib_none().0,
                value,
            );
        }
    }

    #[doc(alias = "selected-index")]
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

impl<O: IsA<DOMHTMLOptionsCollection>> DOMHTMLOptionsCollectionExt for O {}

impl fmt::Display for DOMHTMLOptionsCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLOptionsCollection")
    }
}
