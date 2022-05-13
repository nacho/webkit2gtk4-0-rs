// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMHTMLElement;
use crate::DOMNode;
use crate::DOMObject;
use crate::DOMStyleSheet;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLStyleElement")]
    pub struct DOMHTMLStyleElement(Object<ffi::WebKitDOMHTMLStyleElement, ffi::WebKitDOMHTMLStyleElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_html_style_element_get_type(),
    }
}

impl DOMHTMLStyleElement {
    pub const NONE: Option<&'static DOMHTMLStyleElement> = None;
}

pub trait DOMHTMLStyleElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_style_element_get_disabled")]
    #[doc(alias = "get_disabled")]
    fn is_disabled(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_style_element_get_media")]
    #[doc(alias = "get_media")]
    fn media(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_style_element_get_sheet")]
    #[doc(alias = "get_sheet")]
    fn sheet(&self) -> Option<DOMStyleSheet>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_style_element_get_type_attr")]
    #[doc(alias = "get_type_attr")]
    fn type_attr(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_style_element_set_disabled")]
    fn set_disabled(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_style_element_set_media")]
    fn set_media(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_style_element_set_type_attr")]
    fn set_type_attr(&self, value: &str);

    #[doc(alias = "type")]
    fn type_(&self) -> Option<glib::GString>;

    #[doc(alias = "type")]
    fn set_type(&self, type_: Option<&str>);

    #[doc(alias = "disabled")]
    fn connect_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "media")]
    fn connect_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "sheet")]
    fn connect_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "type")]
    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLStyleElement>> DOMHTMLStyleElementExt for O {
    fn is_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_style_element_get_disabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn media(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_style_element_get_media(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_style_element_get_sheet(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn type_attr(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_style_element_get_type_attr(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_style_element_set_disabled(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    fn set_media(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_style_element_set_media(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_style_element_set_type_attr(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn type_(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "type")
    }

    fn set_type(&self, type_: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "type", &type_)
    }

    fn connect_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_disabled_trampoline<
            P: IsA<DOMHTMLStyleElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLStyleElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLStyleElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::disabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_disabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_media_trampoline<
            P: IsA<DOMHTMLStyleElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLStyleElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLStyleElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::media\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_media_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sheet_trampoline<
            P: IsA<DOMHTMLStyleElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLStyleElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLStyleElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sheet\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sheet_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<
            P: IsA<DOMHTMLStyleElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLStyleElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLStyleElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLStyleElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLStyleElement")
    }
}
