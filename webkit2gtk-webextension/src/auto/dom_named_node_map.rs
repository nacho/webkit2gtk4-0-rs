// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMNode, DOMObject};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitDOMNamedNodeMap")]
    pub struct DOMNamedNodeMap(Object<ffi::WebKitDOMNamedNodeMap, ffi::WebKitDOMNamedNodeMapClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_named_node_map_get_type(),
    }
}

impl DOMNamedNodeMap {
    pub const NONE: Option<&'static DOMNamedNodeMap> = None;
}

pub trait DOMNamedNodeMapExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_named_node_map_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_named_node_map_get_named_item")]
    #[doc(alias = "get_named_item")]
    fn named_item(&self, name: &str) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_named_node_map_get_named_item_ns")]
    #[doc(alias = "get_named_item_ns")]
    fn named_item_ns(&self, namespaceURI: &str, localName: &str) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_named_node_map_item")]
    fn item(&self, index: libc::c_ulong) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_named_node_map_remove_named_item")]
    fn remove_named_item(&self, name: &str) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_named_node_map_remove_named_item_ns")]
    fn remove_named_item_ns(
        &self,
        namespaceURI: &str,
        localName: &str,
    ) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_named_node_map_set_named_item")]
    fn set_named_item(&self, node: &impl IsA<DOMNode>) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_named_node_map_set_named_item_ns")]
    fn set_named_item_ns(&self, node: &impl IsA<DOMNode>) -> Result<DOMNode, glib::Error>;

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMNamedNodeMap>> DOMNamedNodeMapExt for O {
    #[allow(deprecated)]
    fn length(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_named_node_map_get_length(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_named_node_map_get_named_item(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn named_item_ns(&self, namespaceURI: &str, localName: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_named_node_map_get_named_item_ns(
                self.as_ref().to_glib_none().0,
                namespaceURI.to_glib_none().0,
                localName.to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn item(&self, index: libc::c_ulong) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_named_node_map_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    #[allow(deprecated)]
    fn remove_named_item(&self, name: &str) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_remove_named_item(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
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
    fn remove_named_item_ns(
        &self,
        namespaceURI: &str,
        localName: &str,
    ) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_remove_named_item_ns(
                self.as_ref().to_glib_none().0,
                namespaceURI.to_glib_none().0,
                localName.to_glib_none().0,
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
    fn set_named_item(&self, node: &impl IsA<DOMNode>) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_set_named_item(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
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
    fn set_named_item_ns(&self, node: &impl IsA<DOMNode>) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_named_node_map_set_named_item_ns(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<DOMNamedNodeMap>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNamedNodeMap,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNamedNodeMap::from_glib_borrow(this).unsafe_cast_ref())
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

impl fmt::Display for DOMNamedNodeMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMNamedNodeMap")
    }
}
