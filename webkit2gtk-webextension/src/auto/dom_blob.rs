// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::DOMObject;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitDOMBlob")]
    pub struct DOMBlob(Object<ffi::WebKitDOMBlob, ffi::WebKitDOMBlobClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_blob_get_type(),
    }
}

impl DOMBlob {
    pub const NONE: Option<&'static DOMBlob> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMBlob>> Sealed for T {}
}

pub trait DOMBlobExt: IsA<DOMBlob> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_blob_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> u64 {
        unsafe { ffi::webkit_dom_blob_get_size(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "size")]
    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P: IsA<DOMBlob>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMBlob,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMBlob::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DOMBlob>> DOMBlobExt for O {}

impl fmt::Display for DOMBlob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMBlob")
    }
}
