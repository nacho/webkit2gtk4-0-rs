// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMEventTarget, DOMNode, DOMObject};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitDOMCharacterData")]
    pub struct DOMCharacterData(Object<ffi::WebKitDOMCharacterData, ffi::WebKitDOMCharacterDataClass>) @extends DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_character_data_get_type(),
    }
}

impl DOMCharacterData {
    pub const NONE: Option<&'static DOMCharacterData> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMCharacterData>> Sealed for T {}
}

pub trait DOMCharacterDataExt: IsA<DOMCharacterData> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_character_data_append_data")]
    fn append_data(&self, data: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_append_data(
                self.as_ref().to_glib_none().0,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_character_data_delete_data")]
    fn delete_data(&self, offset: libc::c_ulong, length: libc::c_ulong) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_delete_data(
                self.as_ref().to_glib_none().0,
                offset,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_character_data_get_data")]
    #[doc(alias = "get_data")]
    fn data(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_character_data_get_data(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_character_data_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_character_data_get_length(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_character_data_insert_data")]
    fn insert_data(&self, offset: libc::c_ulong, data: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_insert_data(
                self.as_ref().to_glib_none().0,
                offset,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_character_data_replace_data")]
    fn replace_data(
        &self,
        offset: libc::c_ulong,
        length: libc::c_ulong,
        data: &str,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_replace_data(
                self.as_ref().to_glib_none().0,
                offset,
                length,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_character_data_set_data")]
    fn set_data(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_character_data_set_data(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_character_data_substring_data")]
    fn substring_data(
        &self,
        offset: libc::c_ulong,
        length: libc::c_ulong,
    ) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_character_data_substring_data(
                self.as_ref().to_glib_none().0,
                offset,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "data")]
    fn connect_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_data_trampoline<
            P: IsA<DOMCharacterData>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMCharacterData,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMCharacterData::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_data_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<DOMCharacterData>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMCharacterData,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMCharacterData::from_glib_borrow(this).unsafe_cast_ref())
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

impl<O: IsA<DOMCharacterData>> DOMCharacterDataExt for O {}

impl fmt::Display for DOMCharacterData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMCharacterData")
    }
}
