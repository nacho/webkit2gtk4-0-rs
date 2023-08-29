// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitWindowProperties")]
    pub struct WindowProperties(Object<ffi::WebKitWindowProperties, ffi::WebKitWindowPropertiesClass>);

    match fn {
        type_ => || ffi::webkit_window_properties_get_type(),
    }
}

impl WindowProperties {
    pub const NONE: Option<&'static WindowProperties> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`WindowProperties`] objects.
    ///
    /// This method returns an instance of [`WindowPropertiesBuilder`](crate::builders::WindowPropertiesBuilder) which can be used to create [`WindowProperties`] objects.
    pub fn builder() -> WindowPropertiesBuilder {
        WindowPropertiesBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`WindowProperties`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct WindowPropertiesBuilder {
    builder: glib::object::ObjectBuilder<'static, WindowProperties>,
}

impl WindowPropertiesBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn fullscreen(self, fullscreen: bool) -> Self {
        Self {
            builder: self.builder.property("fullscreen", fullscreen),
        }
    }

    pub fn geometry(self, geometry: &gdk::Rectangle) -> Self {
        Self {
            builder: self.builder.property("geometry", geometry),
        }
    }

    pub fn locationbar_visible(self, locationbar_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("locationbar-visible", locationbar_visible),
        }
    }

    pub fn menubar_visible(self, menubar_visible: bool) -> Self {
        Self {
            builder: self.builder.property("menubar-visible", menubar_visible),
        }
    }

    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            builder: self.builder.property("resizable", resizable),
        }
    }

    pub fn scrollbars_visible(self, scrollbars_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("scrollbars-visible", scrollbars_visible),
        }
    }

    pub fn statusbar_visible(self, statusbar_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("statusbar-visible", statusbar_visible),
        }
    }

    pub fn toolbar_visible(self, toolbar_visible: bool) -> Self {
        Self {
            builder: self.builder.property("toolbar-visible", toolbar_visible),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`WindowProperties`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> WindowProperties {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WindowProperties>> Sealed for T {}
}

pub trait WindowPropertiesExt: IsA<WindowProperties> + sealed::Sealed + 'static {
    #[doc(alias = "webkit_window_properties_get_fullscreen")]
    #[doc(alias = "get_fullscreen")]
    fn is_fullscreen(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_fullscreen(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_window_properties_get_geometry")]
    #[doc(alias = "get_geometry")]
    fn geometry(&self) -> gdk::Rectangle {
        unsafe {
            let mut geometry = gdk::Rectangle::uninitialized();
            ffi::webkit_window_properties_get_geometry(
                self.as_ref().to_glib_none().0,
                geometry.to_glib_none_mut().0,
            );
            geometry
        }
    }

    #[doc(alias = "webkit_window_properties_get_locationbar_visible")]
    #[doc(alias = "get_locationbar_visible")]
    fn is_locationbar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_locationbar_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_window_properties_get_menubar_visible")]
    #[doc(alias = "get_menubar_visible")]
    fn is_menubar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_menubar_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_window_properties_get_resizable")]
    #[doc(alias = "get_resizable")]
    fn is_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_resizable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_window_properties_get_scrollbars_visible")]
    #[doc(alias = "get_scrollbars_visible")]
    fn is_scrollbars_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_scrollbars_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_window_properties_get_statusbar_visible")]
    #[doc(alias = "get_statusbar_visible")]
    fn is_statusbar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_statusbar_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_window_properties_get_toolbar_visible")]
    #[doc(alias = "get_toolbar_visible")]
    fn is_toolbar_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_window_properties_get_toolbar_visible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<WindowProperties>> WindowPropertiesExt for O {}

impl fmt::Display for WindowProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WindowProperties")
    }
}
