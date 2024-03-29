// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::WebPage;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitWebEditor")]
    pub struct WebEditor(Object<ffi::WebKitWebEditor, ffi::WebKitWebEditorClass>);

    match fn {
        type_ => || ffi::webkit_web_editor_get_type(),
    }
}

impl WebEditor {
    pub const NONE: Option<&'static WebEditor> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WebEditor>> Sealed for T {}
}

pub trait WebEditorExt: IsA<WebEditor> + sealed::Sealed + 'static {
    #[doc(alias = "webkit_web_editor_get_page")]
    #[doc(alias = "get_page")]
    fn page(&self) -> Option<WebPage> {
        unsafe {
            from_glib_none(ffi::webkit_web_editor_get_page(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_10")))]
    #[doc(alias = "selection-changed")]
    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn selection_changed_trampoline<
            P: IsA<WebEditor>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitWebEditor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(WebEditor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"selection-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    selection_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<WebEditor>> WebEditorExt for O {}

impl fmt::Display for WebEditor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebEditor")
    }
}
