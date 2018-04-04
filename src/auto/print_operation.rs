// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use Error;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use PrintCustomWidget;
use PrintOperationResponse;
use WebView;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct PrintOperation(Object<ffi::WebKitPrintOperation, ffi::WebKitPrintOperationClass>);

    match fn {
        get_type => || ffi::webkit_print_operation_get_type(),
    }
}

impl PrintOperation {
    pub fn new(web_view: &WebView) -> PrintOperation {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_print_operation_new(web_view.to_glib_none().0))
        }
    }
}

pub trait PrintOperationExt {
    fn get_page_setup(&self) -> Option<gtk::PageSetup>;

    fn get_print_settings(&self) -> Option<gtk::PrintSettings>;

    fn print(&self);

    fn run_dialog<'a, P: IsA<gtk::Window> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) -> PrintOperationResponse;

    fn set_page_setup(&self, page_setup: &gtk::PageSetup);

    fn set_print_settings(&self, print_settings: &gtk::PrintSettings);

    fn get_property_web_view(&self) -> Option<WebView>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_create_custom_widget<F: Fn(&Self) -> PrintCustomWidget + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_failed<F: Fn(&Self, &Error) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_setup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_print_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_web_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PrintOperation> + IsA<glib::object::Object>> PrintOperationExt for O {
    fn get_page_setup(&self) -> Option<gtk::PageSetup> {
        unsafe {
            from_glib_none(ffi::webkit_print_operation_get_page_setup(self.to_glib_none().0))
        }
    }

    fn get_print_settings(&self) -> Option<gtk::PrintSettings> {
        unsafe {
            from_glib_none(ffi::webkit_print_operation_get_print_settings(self.to_glib_none().0))
        }
    }

    fn print(&self) {
        unsafe {
            ffi::webkit_print_operation_print(self.to_glib_none().0);
        }
    }

    fn run_dialog<'a, P: IsA<gtk::Window> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) -> PrintOperationResponse {
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib(ffi::webkit_print_operation_run_dialog(self.to_glib_none().0, parent.0))
        }
    }

    fn set_page_setup(&self, page_setup: &gtk::PageSetup) {
        unsafe {
            ffi::webkit_print_operation_set_page_setup(self.to_glib_none().0, page_setup.to_glib_none().0);
        }
    }

    fn set_print_settings(&self, print_settings: &gtk::PrintSettings) {
        unsafe {
            ffi::webkit_print_operation_set_print_settings(self.to_glib_none().0, print_settings.to_glib_none().0);
        }
    }

    fn get_property_web_view(&self) -> Option<WebView> {
        unsafe {
            let mut value = Value::from_type(<WebView as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "web-view".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_create_custom_widget<F: Fn(&Self) -> PrintCustomWidget + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> PrintCustomWidget + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "create-custom-widget",
                transmute(create_custom_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_failed<F: Fn(&Self, &Error) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Error) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "failed",
                transmute(failed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_finished<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "finished",
                transmute(finished_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_page_setup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::page-setup",
                transmute(notify_page_setup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_print_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::print-settings",
                transmute(notify_print_settings_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_web_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::web-view",
                transmute(notify_web_view_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn create_custom_widget_trampoline<P>(this: *mut ffi::WebKitPrintOperation, f: glib_ffi::gpointer) -> *mut ffi::WebKitPrintCustomWidget
where P: IsA<PrintOperation> {
    callback_guard!();
    let f: &&(Fn(&P) -> PrintCustomWidget + 'static) = transmute(f);
    f(&PrintOperation::from_glib_borrow(this).downcast_unchecked()).to_glib_full()
}

unsafe extern "C" fn failed_trampoline<P>(this: *mut ffi::WebKitPrintOperation, error: *mut glib_ffi::GError, f: glib_ffi::gpointer)
where P: IsA<PrintOperation> {
    callback_guard!();
    let f: &&(Fn(&P, &Error) + 'static) = transmute(f);
    f(&PrintOperation::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(error))
}

unsafe extern "C" fn finished_trampoline<P>(this: *mut ffi::WebKitPrintOperation, f: glib_ffi::gpointer)
where P: IsA<PrintOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PrintOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_page_setup_trampoline<P>(this: *mut ffi::WebKitPrintOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PrintOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PrintOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_print_settings_trampoline<P>(this: *mut ffi::WebKitPrintOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PrintOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PrintOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_web_view_trampoline<P>(this: *mut ffi::WebKitPrintOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PrintOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PrintOperation::from_glib_borrow(this).downcast_unchecked())
}
