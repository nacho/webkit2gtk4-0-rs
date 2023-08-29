// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::{DOMNode, HitTestResult};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitWebHitTestResult")]
    pub struct WebHitTestResult(Object<ffi::WebKitWebHitTestResult, ffi::WebKitWebHitTestResultClass>) @extends HitTestResult;

    match fn {
        type_ => || ffi::webkit_web_hit_test_result_get_type(),
    }
}

impl WebHitTestResult {
    pub const NONE: Option<&'static WebHitTestResult> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WebHitTestResult>> Sealed for T {}
}

pub trait WebHitTestResultExt: IsA<WebHitTestResult> + sealed::Sealed + 'static {
    #[cfg(feature = "v2_8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_web_hit_test_result_get_node")]
    #[doc(alias = "get_node")]
    fn node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_web_hit_test_result_get_node(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_node(&self) -> Option<DOMNode> {
        ObjectExt::property(self.as_ref(), "node")
    }
}

impl<O: IsA<WebHitTestResult>> WebHitTestResultExt for O {}

impl fmt::Display for WebHitTestResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebHitTestResult")
    }
}
