// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::{DOMEventTarget, DOMNode, DOMObject};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitDOMEntityReference")]
    pub struct DOMEntityReference(Object<ffi::WebKitDOMEntityReference, ffi::WebKitDOMEntityReferenceClass>) @extends DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_entity_reference_get_type(),
    }
}

impl DOMEntityReference {
    pub const NONE: Option<&'static DOMEntityReference> = None;
}

impl fmt::Display for DOMEntityReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMEntityReference")
    }
}
