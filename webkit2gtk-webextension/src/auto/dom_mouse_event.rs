// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

use crate::{DOMDOMWindow, DOMEvent, DOMEventTarget, DOMNode, DOMObject, DOMUIEvent};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitDOMMouseEvent")]
    pub struct DOMMouseEvent(Object<ffi::WebKitDOMMouseEvent, ffi::WebKitDOMMouseEventClass>) @extends DOMUIEvent, DOMEvent, DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_mouse_event_get_type(),
    }
}

impl DOMMouseEvent {
    pub const NONE: Option<&'static DOMMouseEvent> = None;
}

pub trait DOMMouseEventExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_alt_key")]
    #[doc(alias = "get_alt_key")]
    fn is_alt_key(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_button")]
    #[doc(alias = "get_button")]
    fn button(&self) -> libc::c_ushort;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_client_x")]
    #[doc(alias = "get_client_x")]
    fn client_x(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_client_y")]
    #[doc(alias = "get_client_y")]
    fn client_y(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_ctrl_key")]
    #[doc(alias = "get_ctrl_key")]
    fn is_ctrl_key(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_from_element")]
    #[doc(alias = "get_from_element")]
    fn from_element(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_meta_key")]
    #[doc(alias = "get_meta_key")]
    fn is_meta_key(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_offset_x")]
    #[doc(alias = "get_offset_x")]
    fn offset_x(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_offset_y")]
    #[doc(alias = "get_offset_y")]
    fn offset_y(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_related_target")]
    #[doc(alias = "get_related_target")]
    fn related_target(&self) -> Option<DOMEventTarget>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_screen_x")]
    #[doc(alias = "get_screen_x")]
    fn screen_x(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_screen_y")]
    #[doc(alias = "get_screen_y")]
    fn screen_y(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_shift_key")]
    #[doc(alias = "get_shift_key")]
    fn is_shift_key(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_to_element")]
    #[doc(alias = "get_to_element")]
    fn to_element(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_x")]
    #[doc(alias = "get_x")]
    fn x(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_get_y")]
    #[doc(alias = "get_y")]
    fn y(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_mouse_event_init_mouse_event")]
    fn init_mouse_event(
        &self,
        type_: &str,
        canBubble: bool,
        cancelable: bool,
        view: &impl IsA<DOMDOMWindow>,
        detail: libc::c_long,
        screenX: libc::c_long,
        screenY: libc::c_long,
        clientX: libc::c_long,
        clientY: libc::c_long,
        ctrlKey: bool,
        altKey: bool,
        shiftKey: bool,
        metaKey: bool,
        button: libc::c_ushort,
        relatedTarget: &impl IsA<DOMEventTarget>,
    );

    #[doc(alias = "alt-key")]
    fn connect_alt_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "button")]
    fn connect_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "client-x")]
    fn connect_client_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "client-y")]
    fn connect_client_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "ctrl-key")]
    fn connect_ctrl_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "from-element")]
    fn connect_from_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "meta-key")]
    fn connect_meta_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "offset-x")]
    fn connect_offset_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "offset-y")]
    fn connect_offset_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "related-target")]
    fn connect_related_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "screen-x")]
    fn connect_screen_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "screen-y")]
    fn connect_screen_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "shift-key")]
    fn connect_shift_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "to-element")]
    fn connect_to_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "x")]
    fn connect_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "y")]
    fn connect_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMMouseEvent>> DOMMouseEventExt for O {
    #[allow(deprecated)]
    fn is_alt_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_alt_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn button(&self) -> libc::c_ushort {
        unsafe { ffi::webkit_dom_mouse_event_get_button(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn client_x(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_mouse_event_get_client_x(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn client_y(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_mouse_event_get_client_y(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn is_ctrl_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_ctrl_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn from_element(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_mouse_event_get_from_element(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn is_meta_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_meta_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn offset_x(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_mouse_event_get_offset_x(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn offset_y(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_mouse_event_get_offset_y(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn related_target(&self) -> Option<DOMEventTarget> {
        unsafe {
            from_glib_full(ffi::webkit_dom_mouse_event_get_related_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn screen_x(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_mouse_event_get_screen_x(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn screen_y(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_mouse_event_get_screen_y(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn is_shift_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_mouse_event_get_shift_key(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn to_element(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_mouse_event_get_to_element(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[allow(deprecated)]
    fn x(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_mouse_event_get_x(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn y(&self) -> libc::c_long {
        unsafe { ffi::webkit_dom_mouse_event_get_y(self.as_ref().to_glib_none().0) }
    }

    #[allow(deprecated)]
    fn init_mouse_event(
        &self,
        type_: &str,
        canBubble: bool,
        cancelable: bool,
        view: &impl IsA<DOMDOMWindow>,
        detail: libc::c_long,
        screenX: libc::c_long,
        screenY: libc::c_long,
        clientX: libc::c_long,
        clientY: libc::c_long,
        ctrlKey: bool,
        altKey: bool,
        shiftKey: bool,
        metaKey: bool,
        button: libc::c_ushort,
        relatedTarget: &impl IsA<DOMEventTarget>,
    ) {
        unsafe {
            ffi::webkit_dom_mouse_event_init_mouse_event(
                self.as_ref().to_glib_none().0,
                type_.to_glib_none().0,
                canBubble.into_glib(),
                cancelable.into_glib(),
                view.as_ref().to_glib_none().0,
                detail,
                screenX,
                screenY,
                clientX,
                clientY,
                ctrlKey.into_glib(),
                altKey.into_glib(),
                shiftKey.into_glib(),
                metaKey.into_glib(),
                button,
                relatedTarget.as_ref().to_glib_none().0,
            );
        }
    }

    fn connect_alt_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alt_key_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_button_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::button\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_client_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_client_x_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::client-x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_client_x_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_client_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_client_y_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::client-y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_client_y_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ctrl_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ctrl_key_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_from_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_from_element_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::from-element\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_from_element_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_meta_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_meta_key_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_offset_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_x_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::offset-x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_offset_x_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_offset_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_y_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::offset-y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_offset_y_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_related_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_related_target_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::related-target\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_related_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_screen_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_screen_x_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::screen-x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_screen_x_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_screen_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_screen_y_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::screen-y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_screen_y_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_shift_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shift_key_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_to_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_to_element_trampoline<
            P: IsA<DOMMouseEvent>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::to-element\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_to_element_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_x_trampoline<P: IsA<DOMMouseEvent>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_x_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_y_trampoline<P: IsA<DOMMouseEvent>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMMouseEvent,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMMouseEvent::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_y_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMMouseEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMMouseEvent")
    }
}
