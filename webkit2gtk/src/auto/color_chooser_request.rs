// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_8", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitColorChooserRequest")]
    pub struct ColorChooserRequest(Object<ffi::WebKitColorChooserRequest, ffi::WebKitColorChooserRequestClass>);

    match fn {
        type_ => || ffi::webkit_color_chooser_request_get_type(),
    }
}

impl ColorChooserRequest {
    pub const NONE: Option<&'static ColorChooserRequest> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ColorChooserRequest`] objects.
    ///
    /// This method returns an instance of [`ColorChooserRequestBuilder`](crate::builders::ColorChooserRequestBuilder) which can be used to create [`ColorChooserRequest`] objects.
    pub fn builder() -> ColorChooserRequestBuilder {
        ColorChooserRequestBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ColorChooserRequest`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ColorChooserRequestBuilder {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    rgba: Option<gdk::RGBA>,
}

impl ColorChooserRequestBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ColorChooserRequestBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ColorChooserRequest`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ColorChooserRequest {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v2_8", feature = "dox"))]
        if let Some(ref rgba) = self.rgba {
            properties.push(("rgba", rgba));
        }
        glib::Object::new::<ColorChooserRequest>(&properties)
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    pub fn rgba(mut self, rgba: &gdk::RGBA) -> Self {
        self.rgba = Some(rgba.clone());
        self
    }
}

pub trait ColorChooserRequestExt: 'static {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_color_chooser_request_cancel")]
    fn cancel(&self);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_color_chooser_request_finish")]
    fn finish(&self);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_color_chooser_request_get_element_rectangle")]
    #[doc(alias = "get_element_rectangle")]
    fn element_rectangle(&self) -> gdk::Rectangle;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_color_chooser_request_get_rgba")]
    #[doc(alias = "get_rgba")]
    fn rgba(&self) -> gdk::RGBA;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_color_chooser_request_set_rgba")]
    fn set_rgba(&self, rgba: &gdk::RGBA);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "finished")]
    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "rgba")]
    fn connect_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ColorChooserRequest>> ColorChooserRequestExt for O {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn cancel(&self) {
        unsafe {
            ffi::webkit_color_chooser_request_cancel(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn finish(&self) {
        unsafe {
            ffi::webkit_color_chooser_request_finish(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn element_rectangle(&self) -> gdk::Rectangle {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            ffi::webkit_color_chooser_request_get_element_rectangle(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none_mut().0,
            );
            rect
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn rgba(&self) -> gdk::RGBA {
        unsafe {
            let mut rgba = gdk::RGBA::uninitialized();
            ffi::webkit_color_chooser_request_get_rgba(
                self.as_ref().to_glib_none().0,
                rgba.to_glib_none_mut().0,
            );
            rgba
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn set_rgba(&self, rgba: &gdk::RGBA) {
        unsafe {
            ffi::webkit_color_chooser_request_set_rgba(
                self.as_ref().to_glib_none().0,
                rgba.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn finished_trampoline<
            P: IsA<ColorChooserRequest>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitColorChooserRequest,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ColorChooserRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"finished\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    finished_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn connect_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rgba_trampoline<
            P: IsA<ColorChooserRequest>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitColorChooserRequest,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ColorChooserRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rgba\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rgba_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ColorChooserRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ColorChooserRequest")
    }
}
