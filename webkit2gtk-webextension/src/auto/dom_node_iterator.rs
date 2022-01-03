// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitDOMNodeIterator")]
    pub struct DOMNodeIterator(Object<ffi::WebKitDOMNodeIterator, ffi::WebKitDOMNodeIteratorClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_node_iterator_get_type(),
    }
}

pub const NONE_DOM_NODE_ITERATOR: Option<&DOMNodeIterator> = None;

pub trait DOMNodeIteratorExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_iterator_detach")]
    fn detach(&self);

    #[cfg_attr(feature = "v2_12", deprecated = "Since 2.12")]
    #[doc(alias = "webkit_dom_node_iterator_get_expand_entity_references")]
    #[doc(alias = "get_expand_entity_references")]
    fn expands_entity_references(&self) -> bool;

    //#[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    //#[doc(alias = "webkit_dom_node_iterator_get_filter")]
    //#[doc(alias = "get_filter")]
    //fn filter(&self) -> /*Ignored*/Option<DOMNodeFilter>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_iterator_get_pointer_before_reference_node")]
    #[doc(alias = "get_pointer_before_reference_node")]
    fn is_pointer_before_reference_node(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_iterator_get_reference_node")]
    #[doc(alias = "get_reference_node")]
    fn reference_node(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_iterator_get_root")]
    #[doc(alias = "get_root")]
    fn root(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_iterator_get_what_to_show")]
    #[doc(alias = "get_what_to_show")]
    fn what_to_show(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_iterator_next_node")]
    fn next_node(&self) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_iterator_previous_node")]
    fn previous_node(&self) -> Result<DOMNode, glib::Error>;

    #[doc(alias = "filter")]
    fn connect_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "pointer-before-reference-node")]
    fn connect_pointer_before_reference_node_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "reference-node")]
    fn connect_reference_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "root")]
    fn connect_root_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "what-to-show")]
    fn connect_what_to_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMNodeIterator>> DOMNodeIteratorExt for O {
    fn detach(&self) {
        unsafe {
            ffi::webkit_dom_node_iterator_detach(self.as_ref().to_glib_none().0);
        }
    }

    fn expands_entity_references(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_iterator_get_expand_entity_references(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn filter(&self) -> /*Ignored*/Option<DOMNodeFilter> {
    //    unsafe { TODO: call ffi:webkit_dom_node_iterator_get_filter() }
    //}

    fn is_pointer_before_reference_node(&self) -> bool {
        unsafe {
            from_glib(
                ffi::webkit_dom_node_iterator_get_pointer_before_reference_node(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn reference_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_iterator_get_reference_node(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn root(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_iterator_get_root(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn what_to_show(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_node_iterator_get_what_to_show(self.as_ref().to_glib_none().0) }
    }

    fn next_node(&self) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::webkit_dom_node_iterator_next_node(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn previous_node(&self) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_iterator_previous_node(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<
            P: IsA<DOMNodeIterator>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNodeIterator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNodeIterator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pointer_before_reference_node_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pointer_before_reference_node_trampoline<
            P: IsA<DOMNodeIterator>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNodeIterator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNodeIterator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pointer-before-reference-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pointer_before_reference_node_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reference_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reference_node_trampoline<
            P: IsA<DOMNodeIterator>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNodeIterator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNodeIterator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reference-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reference_node_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_root_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_root_trampoline<
            P: IsA<DOMNodeIterator>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNodeIterator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNodeIterator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::root\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_root_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_what_to_show_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_what_to_show_trampoline<
            P: IsA<DOMNodeIterator>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNodeIterator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNodeIterator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::what-to-show\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_what_to_show_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMNodeIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMNodeIterator")
    }
}