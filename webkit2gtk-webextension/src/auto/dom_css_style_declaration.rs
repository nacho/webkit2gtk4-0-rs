// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMCSSRule, DOMObject};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitDOMCSSStyleDeclaration")]
    pub struct DOMCSSStyleDeclaration(Object<ffi::WebKitDOMCSSStyleDeclaration, ffi::WebKitDOMCSSStyleDeclarationClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_css_style_declaration_get_type(),
    }
}

impl DOMCSSStyleDeclaration {
    pub const NONE: Option<&'static DOMCSSStyleDeclaration> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMCSSStyleDeclaration>> Sealed for T {}
}

pub trait DOMCSSStyleDeclarationExt:
    IsA<DOMCSSStyleDeclaration> + sealed::Sealed + 'static
{
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_get_css_text")]
    #[doc(alias = "get_css_text")]
    fn css_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_css_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_css_style_declaration_get_length(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_get_parent_rule")]
    #[doc(alias = "get_parent_rule")]
    fn parent_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_parent_rule(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_get_property_priority")]
    #[doc(alias = "get_property_priority")]
    fn property_priority(&self, propertyName: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_property_priority(
                self.as_ref().to_glib_none().0,
                propertyName.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_get_property_shorthand")]
    #[doc(alias = "get_property_shorthand")]
    fn property_shorthand(&self, propertyName: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(
                ffi::webkit_dom_css_style_declaration_get_property_shorthand(
                    self.as_ref().to_glib_none().0,
                    propertyName.to_glib_none().0,
                ),
            )
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_get_property_value")]
    #[doc(alias = "get_property_value")]
    fn property_value(&self, propertyName: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_get_property_value(
                self.as_ref().to_glib_none().0,
                propertyName.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_is_property_implicit")]
    fn is_property_implicit(&self, propertyName: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_css_style_declaration_is_property_implicit(
                self.as_ref().to_glib_none().0,
                propertyName.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_item")]
    fn item(&self, index: libc::c_ulong) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_declaration_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_remove_property")]
    fn remove_property(&self, propertyName: &str) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_declaration_remove_property(
                self.as_ref().to_glib_none().0,
                propertyName.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_declaration_set_css_text")]
    fn set_css_text(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_declaration_set_css_text(
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
    #[doc(alias = "webkit_dom_css_style_declaration_set_property")]
    fn set_property(
        &self,
        propertyName: &str,
        value: &str,
        priority: &str,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_declaration_set_property(
                self.as_ref().to_glib_none().0,
                propertyName.to_glib_none().0,
                value.to_glib_none().0,
                priority.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "css-text")]
    fn connect_css_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_css_text_trampoline<
            P: IsA<DOMCSSStyleDeclaration>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMCSSStyleDeclaration,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMCSSStyleDeclaration::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::css-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_css_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<DOMCSSStyleDeclaration>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMCSSStyleDeclaration,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMCSSStyleDeclaration::from_glib_borrow(this).unsafe_cast_ref())
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

    #[doc(alias = "parent-rule")]
    fn connect_parent_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_rule_trampoline<
            P: IsA<DOMCSSStyleDeclaration>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMCSSStyleDeclaration,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMCSSStyleDeclaration::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent-rule\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_rule_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DOMCSSStyleDeclaration>> DOMCSSStyleDeclarationExt for O {}

impl fmt::Display for DOMCSSStyleDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMCSSStyleDeclaration")
    }
}
