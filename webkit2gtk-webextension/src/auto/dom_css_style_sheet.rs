// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMCSSRule, DOMCSSRuleList, DOMObject, DOMStyleSheet};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "WebKitDOMCSSStyleSheet")]
    pub struct DOMCSSStyleSheet(Object<ffi::WebKitDOMCSSStyleSheet, ffi::WebKitDOMCSSStyleSheetClass>) @extends DOMStyleSheet, DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_css_style_sheet_get_type(),
    }
}

impl DOMCSSStyleSheet {
    pub const NONE: Option<&'static DOMCSSStyleSheet> = None;
}

pub trait DOMCSSStyleSheetExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_add_rule")]
    fn add_rule(
        &self,
        selector: &str,
        style: &str,
        index: libc::c_ulong,
    ) -> Result<libc::c_long, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_delete_rule")]
    fn delete_rule(&self, index: libc::c_ulong) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_get_css_rules")]
    #[doc(alias = "get_css_rules")]
    fn css_rules(&self) -> Option<DOMCSSRuleList>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_get_owner_rule")]
    #[doc(alias = "get_owner_rule")]
    fn owner_rule(&self) -> Option<DOMCSSRule>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_get_rules")]
    #[doc(alias = "get_rules")]
    fn rules(&self) -> Option<DOMCSSRuleList>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_insert_rule")]
    fn insert_rule(&self, rule: &str, index: libc::c_ulong) -> Result<libc::c_ulong, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_css_style_sheet_remove_rule")]
    fn remove_rule(&self, index: libc::c_ulong) -> Result<(), glib::Error>;

    #[doc(alias = "css-rules")]
    fn connect_css_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "owner-rule")]
    fn connect_owner_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "rules")]
    fn connect_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMCSSStyleSheet>> DOMCSSStyleSheetExt for O {
    #[allow(deprecated)]
    fn add_rule(
        &self,
        selector: &str,
        style: &str,
        index: libc::c_ulong,
    ) -> Result<libc::c_long, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_sheet_add_rule(
                self.as_ref().to_glib_none().0,
                selector.to_glib_none().0,
                style.to_glib_none().0,
                index,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[allow(deprecated)]
    fn delete_rule(&self, index: libc::c_ulong) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_sheet_delete_rule(
                self.as_ref().to_glib_none().0,
                index,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[allow(deprecated)]
    fn css_rules(&self) -> Option<DOMCSSRuleList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_css_rules(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn owner_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_owner_rule(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn rules(&self) -> Option<DOMCSSRuleList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_rules(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn insert_rule(&self, rule: &str, index: libc::c_ulong) -> Result<libc::c_ulong, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_sheet_insert_rule(
                self.as_ref().to_glib_none().0,
                rule.to_glib_none().0,
                index,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[allow(deprecated)]
    fn remove_rule(&self, index: libc::c_ulong) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_sheet_remove_rule(
                self.as_ref().to_glib_none().0,
                index,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_css_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_css_rules_trampoline<
            P: IsA<DOMCSSStyleSheet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMCSSStyleSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMCSSStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::css-rules\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_css_rules_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_owner_rule_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_rule_trampoline<
            P: IsA<DOMCSSStyleSheet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMCSSStyleSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMCSSStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::owner-rule\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_owner_rule_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rules_trampoline<
            P: IsA<DOMCSSStyleSheet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMCSSStyleSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMCSSStyleSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rules\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rules_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMCSSStyleSheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMCSSStyleSheet")
    }
}
