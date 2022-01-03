// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMDocument;
use crate::DOMElement;
use crate::DOMEventTarget;
use crate::DOMNodeList;
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
    #[doc(alias = "WebKitDOMNode")]
    pub struct DOMNode(Object<ffi::WebKitDOMNode, ffi::WebKitDOMNodeClass>) @extends DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_node_get_type(),
    }
}

impl DOMNode {
    //#[cfg(any(feature = "v2_22", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    //#[doc(alias = "webkit_dom_node_for_js_value")]
    //pub fn for_js_value(value: /*Ignored*/&java_script_core::Value) -> Option<DOMNode> {
    //    unsafe { TODO: call ffi:webkit_dom_node_for_js_value() }
    //}
}

pub const NONE_DOM_NODE: Option<&DOMNode> = None;

pub trait DOMNodeExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_append_child")]
    fn append_child(&self, newChild: &impl IsA<DOMNode>) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[doc(alias = "webkit_dom_node_clone_node")]
    fn clone_node(&self, deep: bool) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[cfg(any(feature = "v2_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_14")))]
    #[doc(alias = "webkit_dom_node_clone_node_with_error")]
    fn clone_node_with_error(&self, deep: bool) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_compare_document_position")]
    fn compare_document_position(&self, other: &impl IsA<DOMNode>) -> libc::c_ushort;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_contains")]
    fn contains(&self, other: &impl IsA<DOMNode>) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_base_uri")]
    #[doc(alias = "get_base_uri")]
    fn base_uri(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_child_nodes")]
    #[doc(alias = "get_child_nodes")]
    fn child_nodes(&self) -> Option<DOMNodeList>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_first_child")]
    #[doc(alias = "get_first_child")]
    fn first_child(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_last_child")]
    #[doc(alias = "get_last_child")]
    fn last_child(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[doc(alias = "webkit_dom_node_get_local_name")]
    #[doc(alias = "get_local_name")]
    fn local_name(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[doc(alias = "webkit_dom_node_get_namespace_uri")]
    #[doc(alias = "get_namespace_uri")]
    fn namespace_uri(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_next_sibling")]
    #[doc(alias = "get_next_sibling")]
    fn next_sibling(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_node_name")]
    #[doc(alias = "get_node_name")]
    fn node_name(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_node_type")]
    #[doc(alias = "get_node_type")]
    fn node_type(&self) -> libc::c_ushort;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_node_value")]
    #[doc(alias = "get_node_value")]
    fn node_value(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_owner_document")]
    #[doc(alias = "get_owner_document")]
    fn owner_document(&self) -> Option<DOMDocument>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_parent_element")]
    #[doc(alias = "get_parent_element")]
    fn parent_element(&self) -> Option<DOMElement>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_parent_node")]
    #[doc(alias = "get_parent_node")]
    fn parent_node(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[doc(alias = "webkit_dom_node_get_prefix")]
    #[doc(alias = "get_prefix")]
    fn prefix(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_previous_sibling")]
    #[doc(alias = "get_previous_sibling")]
    fn previous_sibling(&self) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_get_text_content")]
    #[doc(alias = "get_text_content")]
    fn text_content(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_has_child_nodes")]
    fn has_child_nodes(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_insert_before")]
    fn insert_before(
        &self,
        newChild: &impl IsA<DOMNode>,
        refChild: Option<&impl IsA<DOMNode>>,
    ) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_is_default_namespace")]
    fn is_default_namespace(&self, namespaceURI: &str) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_is_equal_node")]
    fn is_equal_node(&self, other: &impl IsA<DOMNode>) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_is_same_node")]
    fn is_same_node(&self, other: &impl IsA<DOMNode>) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_is_supported")]
    fn is_supported(&self, feature: &str, version: &str) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_lookup_namespace_uri")]
    fn lookup_namespace_uri(&self, prefix: &str) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_lookup_prefix")]
    fn lookup_prefix(&self, namespaceURI: &str) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_normalize")]
    fn normalize(&self);

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_remove_child")]
    fn remove_child(&self, oldChild: &impl IsA<DOMNode>) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_replace_child")]
    fn replace_child(
        &self,
        newChild: &impl IsA<DOMNode>,
        oldChild: &impl IsA<DOMNode>,
    ) -> Result<DOMNode, glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_set_node_value")]
    fn set_node_value(&self, value: &str) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_14", deprecated = "Since 2.14")]
    #[doc(alias = "webkit_dom_node_set_prefix")]
    fn set_prefix(&self, value: &str) -> Result<(), glib::Error>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_node_set_text_content")]
    fn set_text_content(&self, value: &str) -> Result<(), glib::Error>;

    #[doc(alias = "base-uri")]
    fn connect_base_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "child-nodes")]
    fn connect_child_nodes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "first-child")]
    fn connect_first_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "last-child")]
    fn connect_last_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "next-sibling")]
    fn connect_next_sibling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "node-name")]
    fn connect_node_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "node-type")]
    fn connect_node_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "node-value")]
    fn connect_node_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "owner-document")]
    fn connect_owner_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "parent-element")]
    fn connect_parent_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "parent-node")]
    fn connect_parent_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "previous-sibling")]
    fn connect_previous_sibling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "text-content")]
    fn connect_text_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMNode>> DOMNodeExt for O {
    fn append_child(&self, newChild: &impl IsA<DOMNode>) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_append_child(
                self.as_ref().to_glib_none().0,
                newChild.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn clone_node(&self, deep: bool) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_clone_node(
                self.as_ref().to_glib_none().0,
                deep.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_14")))]
    fn clone_node_with_error(&self, deep: bool) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_clone_node_with_error(
                self.as_ref().to_glib_none().0,
                deep.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn compare_document_position(&self, other: &impl IsA<DOMNode>) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_node_compare_document_position(
                self.as_ref().to_glib_none().0,
                other.as_ref().to_glib_none().0,
            )
        }
    }

    fn contains(&self, other: &impl IsA<DOMNode>) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_contains(
                self.as_ref().to_glib_none().0,
                other.as_ref().to_glib_none().0,
            ))
        }
    }

    fn base_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_base_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn child_nodes(&self) -> Option<DOMNodeList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_child_nodes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn first_child(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_first_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn last_child(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_last_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn local_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_local_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn namespace_uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_namespace_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn next_sibling(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_next_sibling(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn node_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_node_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn node_type(&self) -> libc::c_ushort {
        unsafe { ffi::webkit_dom_node_get_node_type(self.as_ref().to_glib_none().0) }
    }

    fn node_value(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_node_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn owner_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_owner_document(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn parent_element(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_parent_element(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn parent_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_parent_node(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn prefix(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_prefix(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn previous_sibling(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_node_get_previous_sibling(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn text_content(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_get_text_content(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_child_nodes(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_has_child_nodes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert_before(
        &self,
        newChild: &impl IsA<DOMNode>,
        refChild: Option<&impl IsA<DOMNode>>,
    ) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_insert_before(
                self.as_ref().to_glib_none().0,
                newChild.as_ref().to_glib_none().0,
                refChild.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn is_default_namespace(&self, namespaceURI: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_default_namespace(
                self.as_ref().to_glib_none().0,
                namespaceURI.to_glib_none().0,
            ))
        }
    }

    fn is_equal_node(&self, other: &impl IsA<DOMNode>) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_equal_node(
                self.as_ref().to_glib_none().0,
                other.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_same_node(&self, other: &impl IsA<DOMNode>) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_same_node(
                self.as_ref().to_glib_none().0,
                other.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_supported(&self, feature: &str, version: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_node_is_supported(
                self.as_ref().to_glib_none().0,
                feature.to_glib_none().0,
                version.to_glib_none().0,
            ))
        }
    }

    fn lookup_namespace_uri(&self, prefix: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_lookup_namespace_uri(
                self.as_ref().to_glib_none().0,
                prefix.to_glib_none().0,
            ))
        }
    }

    fn lookup_prefix(&self, namespaceURI: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_node_lookup_prefix(
                self.as_ref().to_glib_none().0,
                namespaceURI.to_glib_none().0,
            ))
        }
    }

    fn normalize(&self) {
        unsafe {
            ffi::webkit_dom_node_normalize(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_child(&self, oldChild: &impl IsA<DOMNode>) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_remove_child(
                self.as_ref().to_glib_none().0,
                oldChild.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn replace_child(
        &self,
        newChild: &impl IsA<DOMNode>,
        oldChild: &impl IsA<DOMNode>,
    ) -> Result<DOMNode, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_node_replace_child(
                self.as_ref().to_glib_none().0,
                newChild.as_ref().to_glib_none().0,
                oldChild.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_node_value(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_node_set_node_value(
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

    fn set_prefix(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_node_set_prefix(
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

    fn set_text_content(&self, value: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_node_set_text_content(
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

    fn connect_base_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_base_uri_trampoline<P: IsA<DOMNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::base-uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_base_uri_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_child_nodes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_nodes_trampoline<P: IsA<DOMNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child-nodes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_nodes_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_first_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_first_child_trampoline<P: IsA<DOMNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::first-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_first_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_last_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_last_child_trampoline<P: IsA<DOMNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::last-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_last_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_next_sibling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_next_sibling_trampoline<
            P: IsA<DOMNode>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::next-sibling\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_next_sibling_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_node_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_node_name_trampoline<P: IsA<DOMNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::node-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_node_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_node_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_node_type_trampoline<P: IsA<DOMNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::node-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_node_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_node_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_node_value_trampoline<P: IsA<DOMNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::node-value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_node_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_owner_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_document_trampoline<
            P: IsA<DOMNode>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::owner-document\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_owner_document_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_parent_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_element_trampoline<
            P: IsA<DOMNode>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent-element\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_element_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_parent_node_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_node_trampoline<P: IsA<DOMNode>, F: Fn(&P) + 'static>(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent-node\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_node_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_previous_sibling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_previous_sibling_trampoline<
            P: IsA<DOMNode>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::previous-sibling\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_previous_sibling_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_text_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_content_trampoline<
            P: IsA<DOMNode>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMNode::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text-content\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_content_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMNode")
    }
}