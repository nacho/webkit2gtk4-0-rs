// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitSecurityManager")]
    pub struct SecurityManager(Object<ffi::WebKitSecurityManager, ffi::WebKitSecurityManagerClass>);

    match fn {
        type_ => || ffi::webkit_security_manager_get_type(),
    }
}

impl SecurityManager {
    pub const NONE: Option<&'static SecurityManager> = None;
}

pub trait SecurityManagerExt: 'static {
    #[doc(alias = "webkit_security_manager_register_uri_scheme_as_cors_enabled")]
    fn register_uri_scheme_as_cors_enabled(&self, scheme: &str);

    #[doc(alias = "webkit_security_manager_register_uri_scheme_as_display_isolated")]
    fn register_uri_scheme_as_display_isolated(&self, scheme: &str);

    #[doc(alias = "webkit_security_manager_register_uri_scheme_as_empty_document")]
    fn register_uri_scheme_as_empty_document(&self, scheme: &str);

    #[doc(alias = "webkit_security_manager_register_uri_scheme_as_local")]
    fn register_uri_scheme_as_local(&self, scheme: &str);

    #[doc(alias = "webkit_security_manager_register_uri_scheme_as_no_access")]
    fn register_uri_scheme_as_no_access(&self, scheme: &str);

    #[doc(alias = "webkit_security_manager_register_uri_scheme_as_secure")]
    fn register_uri_scheme_as_secure(&self, scheme: &str);

    #[doc(alias = "webkit_security_manager_uri_scheme_is_cors_enabled")]
    fn uri_scheme_is_cors_enabled(&self, scheme: &str) -> bool;

    #[doc(alias = "webkit_security_manager_uri_scheme_is_display_isolated")]
    fn uri_scheme_is_display_isolated(&self, scheme: &str) -> bool;

    #[doc(alias = "webkit_security_manager_uri_scheme_is_empty_document")]
    fn uri_scheme_is_empty_document(&self, scheme: &str) -> bool;

    #[doc(alias = "webkit_security_manager_uri_scheme_is_local")]
    fn uri_scheme_is_local(&self, scheme: &str) -> bool;

    #[doc(alias = "webkit_security_manager_uri_scheme_is_no_access")]
    fn uri_scheme_is_no_access(&self, scheme: &str) -> bool;

    #[doc(alias = "webkit_security_manager_uri_scheme_is_secure")]
    fn uri_scheme_is_secure(&self, scheme: &str) -> bool;
}

impl<O: IsA<SecurityManager>> SecurityManagerExt for O {
    fn register_uri_scheme_as_cors_enabled(&self, scheme: &str) {
        unsafe {
            ffi::webkit_security_manager_register_uri_scheme_as_cors_enabled(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            );
        }
    }

    fn register_uri_scheme_as_display_isolated(&self, scheme: &str) {
        unsafe {
            ffi::webkit_security_manager_register_uri_scheme_as_display_isolated(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            );
        }
    }

    fn register_uri_scheme_as_empty_document(&self, scheme: &str) {
        unsafe {
            ffi::webkit_security_manager_register_uri_scheme_as_empty_document(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            );
        }
    }

    fn register_uri_scheme_as_local(&self, scheme: &str) {
        unsafe {
            ffi::webkit_security_manager_register_uri_scheme_as_local(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            );
        }
    }

    fn register_uri_scheme_as_no_access(&self, scheme: &str) {
        unsafe {
            ffi::webkit_security_manager_register_uri_scheme_as_no_access(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            );
        }
    }

    fn register_uri_scheme_as_secure(&self, scheme: &str) {
        unsafe {
            ffi::webkit_security_manager_register_uri_scheme_as_secure(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            );
        }
    }

    fn uri_scheme_is_cors_enabled(&self, scheme: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_security_manager_uri_scheme_is_cors_enabled(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            ))
        }
    }

    fn uri_scheme_is_display_isolated(&self, scheme: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_security_manager_uri_scheme_is_display_isolated(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            ))
        }
    }

    fn uri_scheme_is_empty_document(&self, scheme: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_security_manager_uri_scheme_is_empty_document(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            ))
        }
    }

    fn uri_scheme_is_local(&self, scheme: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_security_manager_uri_scheme_is_local(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            ))
        }
    }

    fn uri_scheme_is_no_access(&self, scheme: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_security_manager_uri_scheme_is_no_access(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            ))
        }
    }

    fn uri_scheme_is_secure(&self, scheme: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_security_manager_uri_scheme_is_secure(
                self.as_ref().to_glib_none().0,
                scheme.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for SecurityManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SecurityManager")
    }
}
