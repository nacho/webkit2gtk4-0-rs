// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::Value;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "JSCWeakValue")]
    pub struct WeakValue(Object<ffi::JSCWeakValue, ffi::JSCWeakValueClass>);

    match fn {
        type_ => || ffi::jsc_weak_value_get_type(),
    }
}

impl WeakValue {
    pub const NONE: Option<&'static WeakValue> = None;

    #[doc(alias = "jsc_weak_value_new")]
    pub fn new(value: &impl IsA<Value>) -> WeakValue {
        unsafe { from_glib_full(ffi::jsc_weak_value_new(value.as_ref().to_glib_none().0)) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`WeakValue`] objects.
    ///
    /// This method returns an instance of [`WeakValueBuilder`](crate::builders::WeakValueBuilder) which can be used to create [`WeakValue`] objects.
    pub fn builder() -> WeakValueBuilder {
        WeakValueBuilder::new()
    }
}

impl Default for WeakValue {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`WeakValue`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct WeakValueBuilder {
    builder: glib::object::ObjectBuilder<'static, WeakValue>,
}

impl WeakValueBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn value(self, value: &impl IsA<Value>) -> Self {
        Self {
            builder: self.builder.property("value", value.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`WeakValue`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> WeakValue {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WeakValue>> Sealed for T {}
}

pub trait WeakValueExt: IsA<WeakValue> + sealed::Sealed + 'static {
    #[doc(alias = "jsc_weak_value_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self) -> Option<Value> {
        unsafe {
            from_glib_full(ffi::jsc_weak_value_get_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "cleared")]
    fn connect_cleared<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cleared_trampoline<P: IsA<WeakValue>, F: Fn(&P) + 'static>(
            this: *mut ffi::JSCWeakValue,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(WeakValue::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cleared\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cleared_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<WeakValue>> WeakValueExt for O {}

impl fmt::Display for WeakValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WeakValue")
    }
}
