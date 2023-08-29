// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMDOMWindow, DOMEvent, DOMObject, DOMUIEvent};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitDOMKeyboardEvent")]
    pub struct DOMKeyboardEvent(Object<ffi::WebKitDOMKeyboardEvent, ffi::WebKitDOMKeyboardEventClass>) @extends DOMUIEvent, DOMEvent, DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_keyboard_event_get_type(),
    }
}

impl DOMKeyboardEvent {
    pub const NONE: Option<&'static DOMKeyboardEvent> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMKeyboardEvent>> Sealed for T {}
}

pub trait DOMKeyboardEventExt: IsA<DOMKeyboardEvent> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_keyboard_event_get_alt_graph_key")]
    #[doc(alias = "get_alt_graph_key")]
    fn is_alt_graph_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_alt_graph_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_keyboard_event_get_alt_key")]
    #[doc(alias = "get_alt_key")]
    fn is_alt_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_alt_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_keyboard_event_get_ctrl_key")]
    #[doc(alias = "get_ctrl_key")]
    fn is_ctrl_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_ctrl_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_keyboard_event_get_key_identifier")]
    #[doc(alias = "get_key_identifier")]
    fn key_identifier(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_keyboard_event_get_key_identifier(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_keyboard_event_get_key_location")]
    #[doc(alias = "get_key_location")]
    fn key_location(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_keyboard_event_get_key_location(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_keyboard_event_get_meta_key")]
    #[doc(alias = "get_meta_key")]
    fn is_meta_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_meta_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_keyboard_event_get_modifier_state")]
    #[doc(alias = "get_modifier_state")]
    fn is_modifier_state(&self, keyIdentifierArg: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_modifier_state(
                self.as_ref().to_glib_none().0,
                keyIdentifierArg.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_keyboard_event_get_shift_key")]
    #[doc(alias = "get_shift_key")]
    fn is_shift_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_shift_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_keyboard_event_init_keyboard_event")]
    fn init_keyboard_event(
        &self,
        type_: &str,
        canBubble: bool,
        cancelable: bool,
        view: &impl IsA<DOMDOMWindow>,
        keyIdentifier: &str,
        location: libc::c_ulong,
        ctrlKey: bool,
        altKey: bool,
        shiftKey: bool,
        metaKey: bool,
        altGraphKey: bool,
    ) {
        unsafe {
            ffi::webkit_dom_keyboard_event_init_keyboard_event(
                self.as_ref().to_glib_none().0,
                type_.to_glib_none().0,
                canBubble.into_glib(),
                cancelable.into_glib(),
                view.as_ref().to_glib_none().0,
                keyIdentifier.to_glib_none().0,
                location,
                ctrlKey.into_glib(),
                altKey.into_glib(),
                shiftKey.into_glib(),
                metaKey.into_glib(),
                altGraphKey.into_glib(),
            );
        }
    }

    #[doc(alias = "alt-graph-key")]
    fn connect_alt_graph_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alt_graph_key_trampoline<
            P: IsA<DOMKeyboardEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMKeyboardEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::alt-graph-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alt_graph_key_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "alt-key")]
    fn connect_alt_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alt_key_trampoline<
            P: IsA<DOMKeyboardEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMKeyboardEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::alt-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_alt_key_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ctrl-key")]
    fn connect_ctrl_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ctrl_key_trampoline<
            P: IsA<DOMKeyboardEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMKeyboardEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ctrl-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ctrl_key_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "key-identifier")]
    fn connect_key_identifier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_key_identifier_trampoline<
            P: IsA<DOMKeyboardEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMKeyboardEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::key-identifier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_key_identifier_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "key-location")]
    fn connect_key_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_key_location_trampoline<
            P: IsA<DOMKeyboardEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMKeyboardEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::key-location\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_key_location_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "meta-key")]
    fn connect_meta_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_meta_key_trampoline<
            P: IsA<DOMKeyboardEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMKeyboardEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::meta-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_meta_key_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "shift-key")]
    fn connect_shift_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shift_key_trampoline<
            P: IsA<DOMKeyboardEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMKeyboardEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMKeyboardEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shift-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shift_key_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DOMKeyboardEvent>> DOMKeyboardEventExt for O {}

impl fmt::Display for DOMKeyboardEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMKeyboardEvent")
    }
}
