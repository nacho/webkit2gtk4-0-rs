// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
use crate::DOMRange;
use crate::{DOMNode, DOMObject};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(feature = "v2_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
use std::ptr;
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "WebKitDOMDOMSelection")]
    pub struct DOMDOMSelection(Object<ffi::WebKitDOMDOMSelection, ffi::WebKitDOMDOMSelectionClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_dom_selection_get_type(),
    }
}

impl DOMDOMSelection {
    pub const NONE: Option<&'static DOMDOMSelection> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DOMDOMSelection>> Sealed for T {}
}

pub trait DOMDOMSelectionExt: IsA<DOMDOMSelection> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_add_range")]
    fn add_range(&self, range: &impl IsA<DOMRange>) {
        unsafe {
            ffi::webkit_dom_dom_selection_add_range(
                self.as_ref().to_glib_none().0,
                range.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_collapse")]
    fn collapse(&self, node: &impl IsA<DOMNode>, offset: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_dom_selection_collapse(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                offset,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_collapse_to_end")]
    fn collapse_to_end(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_selection_collapse_to_end(
                self.as_ref().to_glib_none().0,
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
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_collapse_to_start")]
    fn collapse_to_start(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_selection_collapse_to_start(
                self.as_ref().to_glib_none().0,
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
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_contains_node")]
    fn contains_node(&self, node: &impl IsA<DOMNode>, allowPartial: bool) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_selection_contains_node(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                allowPartial.into_glib(),
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_delete_from_document")]
    fn delete_from_document(&self) {
        unsafe {
            ffi::webkit_dom_dom_selection_delete_from_document(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_empty")]
    fn empty(&self) {
        unsafe {
            ffi::webkit_dom_dom_selection_empty(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_extend")]
    fn extend(&self, node: &impl IsA<DOMNode>, offset: libc::c_ulong) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_dom_selection_extend(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                offset,
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
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_anchor_node")]
    #[doc(alias = "get_anchor_node")]
    fn anchor_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_anchor_node(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_anchor_offset")]
    #[doc(alias = "get_anchor_offset")]
    fn anchor_offset(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_dom_selection_get_anchor_offset(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_base_node")]
    #[doc(alias = "get_base_node")]
    fn base_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_base_node(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_base_offset")]
    #[doc(alias = "get_base_offset")]
    fn base_offset(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_dom_selection_get_base_offset(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_extent_node")]
    #[doc(alias = "get_extent_node")]
    fn extent_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_extent_node(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_extent_offset")]
    #[doc(alias = "get_extent_offset")]
    fn extent_offset(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_dom_selection_get_extent_offset(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_focus_node")]
    #[doc(alias = "get_focus_node")]
    fn focus_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_selection_get_focus_node(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_focus_offset")]
    #[doc(alias = "get_focus_offset")]
    fn focus_offset(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_dom_selection_get_focus_offset(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_is_collapsed")]
    #[doc(alias = "get_is_collapsed")]
    fn is_collapsed(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_selection_get_is_collapsed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_range_at")]
    #[doc(alias = "get_range_at")]
    fn range_at(&self, index: libc::c_ulong) -> Result<DOMRange, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_selection_get_range_at(
                self.as_ref().to_glib_none().0,
                index,
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
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_range_count")]
    #[doc(alias = "get_range_count")]
    fn range_count(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_dom_selection_get_range_count(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_get_selection_type")]
    #[doc(alias = "get_selection_type")]
    fn selection_type(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_dom_selection_get_selection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_modify")]
    fn modify(&self, alter: &str, direction: &str, granularity: &str) {
        unsafe {
            ffi::webkit_dom_dom_selection_modify(
                self.as_ref().to_glib_none().0,
                alter.to_glib_none().0,
                direction.to_glib_none().0,
                granularity.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_remove_all_ranges")]
    fn remove_all_ranges(&self) {
        unsafe {
            ffi::webkit_dom_dom_selection_remove_all_ranges(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_select_all_children")]
    fn select_all_children(&self, node: &impl IsA<DOMNode>) {
        unsafe {
            ffi::webkit_dom_dom_selection_select_all_children(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_set_base_and_extent")]
    fn set_base_and_extent(
        &self,
        baseNode: &impl IsA<DOMNode>,
        baseOffset: libc::c_ulong,
        extentNode: &impl IsA<DOMNode>,
        extentOffset: libc::c_ulong,
    ) {
        unsafe {
            ffi::webkit_dom_dom_selection_set_base_and_extent(
                self.as_ref().to_glib_none().0,
                baseNode.as_ref().to_glib_none().0,
                baseOffset,
                extentNode.as_ref().to_glib_none().0,
                extentOffset,
            );
        }
    }

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(feature = "v2_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_16")))]
    #[allow(deprecated)]
    #[doc(alias = "webkit_dom_dom_selection_set_position")]
    fn set_position(&self, node: &impl IsA<DOMNode>, offset: libc::c_ulong) {
        unsafe {
            ffi::webkit_dom_dom_selection_set_position(
                self.as_ref().to_glib_none().0,
                node.as_ref().to_glib_none().0,
                offset,
            );
        }
    }

    #[doc(alias = "anchor-node")]
    fn get_property_anchor_node(&self) -> Option<DOMNode> {
        ObjectExt::property(self.as_ref(), "anchor-node")
    }

    #[doc(alias = "anchor-offset")]
    fn get_property_anchor_offset(&self) -> libc::c_ulong {
        ObjectExt::property(self.as_ref(), "anchor-offset")
    }

    #[doc(alias = "base-node")]
    fn get_property_base_node(&self) -> Option<DOMNode> {
        ObjectExt::property(self.as_ref(), "base-node")
    }

    #[doc(alias = "base-offset")]
    fn get_property_base_offset(&self) -> libc::c_ulong {
        ObjectExt::property(self.as_ref(), "base-offset")
    }

    #[doc(alias = "extent-node")]
    fn get_property_extent_node(&self) -> Option<DOMNode> {
        ObjectExt::property(self.as_ref(), "extent-node")
    }

    #[doc(alias = "extent-offset")]
    fn get_property_extent_offset(&self) -> libc::c_ulong {
        ObjectExt::property(self.as_ref(), "extent-offset")
    }

    #[doc(alias = "focus-node")]
    fn get_property_focus_node(&self) -> Option<DOMNode> {
        ObjectExt::property(self.as_ref(), "focus-node")
    }

    #[doc(alias = "focus-offset")]
    fn get_property_focus_offset(&self) -> libc::c_ulong {
        ObjectExt::property(self.as_ref(), "focus-offset")
    }

    #[doc(alias = "is-collapsed")]
    fn get_property_is_collapsed(&self) -> bool {
        ObjectExt::property(self.as_ref(), "is-collapsed")
    }

    #[doc(alias = "range-count")]
    fn get_property_range_count(&self) -> libc::c_ulong {
        ObjectExt::property(self.as_ref(), "range-count")
    }

    #[doc(alias = "type")]
    fn type_(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "type")
    }

    #[doc(alias = "anchor-node")]
    fn connect_anchor_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anchor_node_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::anchor-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_anchor_node_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "anchor-offset")]
    fn connect_anchor_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_anchor_offset_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::anchor-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_anchor_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "base-node")]
    fn connect_base_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_base_node_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::base-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_base_node_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "base-offset")]
    fn connect_base_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_base_offset_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::base-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_base_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "extent-node")]
    fn connect_extent_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_extent_node_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::extent-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_extent_node_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "extent-offset")]
    fn connect_extent_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_extent_offset_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::extent-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_extent_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "focus-node")]
    fn connect_focus_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_node_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::focus-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_focus_node_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "focus-offset")]
    fn connect_focus_offset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_focus_offset_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::focus-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_focus_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-collapsed")]
    fn connect_is_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_collapsed_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-collapsed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_collapsed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "range-count")]
    fn connect_range_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_range_count_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::range-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_range_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "type")]
    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<
            P: IsA<DOMDOMSelection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDOMSelection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDOMSelection::from_glib_borrow(this).unsafe_cast_ref())
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

impl<O: IsA<DOMDOMSelection>> DOMDOMSelectionExt for O {}

impl fmt::Display for DOMDOMSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMDOMSelection")
    }
}
