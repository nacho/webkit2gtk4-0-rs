// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitDOMObject")]
    pub struct DOMObject(Object<ffi::WebKitDOMObject, ffi::WebKitDOMObjectClass>);

    match fn {
        type_ => || ffi::webkit_dom_object_get_type(),
    }
}

impl DOMObject {}

pub const NONE_DOM_OBJECT: Option<&DOMObject> = None;

impl fmt::Display for DOMObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMObject")
    }
}