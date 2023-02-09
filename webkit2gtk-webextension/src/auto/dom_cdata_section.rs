// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::{DOMCharacterData, DOMEventTarget, DOMNode, DOMObject, DOMText};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitDOMCDATASection")]
    pub struct DOMCDATASection(Object<ffi::WebKitDOMCDATASection, ffi::WebKitDOMCDATASectionClass>) @extends DOMText, DOMCharacterData, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_cdata_section_get_type(),
    }
}

impl DOMCDATASection {
    pub const NONE: Option<&'static DOMCDATASection> = None;
}

impl fmt::Display for DOMCDATASection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMCDATASection")
    }
}
