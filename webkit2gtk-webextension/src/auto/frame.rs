// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT
#![allow(deprecated)]

#[cfg(feature = "v2_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_22")))]
use crate::{DOMObject, ScriptWorld};
use glib::prelude::*;
#[cfg(feature = "v2_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitFrame")]
    pub struct Frame(Object<ffi::WebKitFrame, ffi::WebKitFrameClass>);

    match fn {
        type_ => || ffi::webkit_frame_get_type(),
    }
}

impl Frame {
    pub const NONE: Option<&'static Frame> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Frame>> Sealed for T {}
}

pub trait FrameExt: IsA<Frame> + sealed::Sealed + 'static {
    #[cfg(feature = "v2_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_26")))]
    #[doc(alias = "webkit_frame_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> u64 {
        unsafe { ffi::webkit_frame_get_id(self.as_ref().to_glib_none().0) }
    }

    //#[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    //#[cfg(feature = "v2_2")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
    //#[allow(deprecated)]
    //#[doc(alias = "webkit_frame_get_javascript_context_for_script_world")]
    //#[doc(alias = "get_javascript_context_for_script_world")]
    //fn javascript_context_for_script_world(&self, world: &impl IsA<ScriptWorld>) -> /*Ignored*/Option<javascriptcore::GlobalContextRef> {
    //    unsafe { TODO: call ffi:webkit_frame_get_javascript_context_for_script_world() }
    //}

    //#[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    //#[cfg(feature = "v2_2")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
    //#[allow(deprecated)]
    //#[doc(alias = "webkit_frame_get_javascript_global_context")]
    //#[doc(alias = "get_javascript_global_context")]
    //fn javascript_global_context(&self) -> /*Ignored*/Option<javascriptcore::GlobalContextRef> {
    //    unsafe { TODO: call ffi:webkit_frame_get_javascript_global_context() }
    //}

    #[cfg(feature = "v2_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_22")))]
    #[doc(alias = "webkit_frame_get_js_context")]
    #[doc(alias = "get_js_context")]
    fn js_context(&self) -> Option<javascriptcore::Context> {
        unsafe {
            from_glib_full(ffi::webkit_frame_get_js_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_22")))]
    #[doc(alias = "webkit_frame_get_js_context_for_script_world")]
    #[doc(alias = "get_js_context_for_script_world")]
    fn js_context_for_script_world(
        &self,
        world: &impl IsA<ScriptWorld>,
    ) -> Option<javascriptcore::Context> {
        unsafe {
            from_glib_full(ffi::webkit_frame_get_js_context_for_script_world(
                self.as_ref().to_glib_none().0,
                world.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_22")))]
    #[doc(alias = "webkit_frame_get_js_value_for_dom_object")]
    #[doc(alias = "get_js_value_for_dom_object")]
    fn js_value_for_dom_object(
        &self,
        dom_object: &impl IsA<DOMObject>,
    ) -> Option<javascriptcore::Value> {
        unsafe {
            from_glib_full(ffi::webkit_frame_get_js_value_for_dom_object(
                self.as_ref().to_glib_none().0,
                dom_object.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_22")))]
    #[doc(alias = "webkit_frame_get_js_value_for_dom_object_in_script_world")]
    #[doc(alias = "get_js_value_for_dom_object_in_script_world")]
    fn js_value_for_dom_object_in_script_world(
        &self,
        dom_object: &impl IsA<DOMObject>,
        world: &impl IsA<ScriptWorld>,
    ) -> Option<javascriptcore::Value> {
        unsafe {
            from_glib_full(
                ffi::webkit_frame_get_js_value_for_dom_object_in_script_world(
                    self.as_ref().to_glib_none().0,
                    dom_object.as_ref().to_glib_none().0,
                    world.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(feature = "v2_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_frame_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_frame_get_uri(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(feature = "v2_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_frame_is_main_frame")]
    fn is_main_frame(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_frame_is_main_frame(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<Frame>> FrameExt for O {}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Frame")
    }
}
