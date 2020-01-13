// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::GString;
use std::fmt;
use webkit2_sys;
use PermissionRequest;

glib_wrapper! {
    pub struct InstallMissingMediaPluginsPermissionRequest(Object<webkit2_sys::WebKitInstallMissingMediaPluginsPermissionRequest, webkit2_sys::WebKitInstallMissingMediaPluginsPermissionRequestClass, InstallMissingMediaPluginsPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        get_type => || webkit2_sys::webkit_install_missing_media_plugins_permission_request_get_type(),
    }
}

pub const NONE_INSTALL_MISSING_MEDIA_PLUGINS_PERMISSION_REQUEST: Option<
    &InstallMissingMediaPluginsPermissionRequest,
> = None;

pub trait InstallMissingMediaPluginsPermissionRequestExt: 'static {
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_description(&self) -> Option<GString>;
}

impl<O: IsA<InstallMissingMediaPluginsPermissionRequest>>
    InstallMissingMediaPluginsPermissionRequestExt for O
{
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_install_missing_media_plugins_permission_request_get_description(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for InstallMissingMediaPluginsPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InstallMissingMediaPluginsPermissionRequest")
    }
}
