// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use PermissionRequest;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GeolocationPermissionRequest(Object<ffi::WebKitGeolocationPermissionRequest, ffi::WebKitGeolocationPermissionRequestClass>): PermissionRequest;

    match fn {
        get_type => || ffi::webkit_geolocation_permission_request_get_type(),
    }
}

impl GeolocationPermissionRequest {}
