// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
mod authentication_request;
#[cfg(any(feature = "v2_2", feature = "dox"))]
pub use self::authentication_request::AuthenticationRequestExt;
#[cfg(any(feature = "v2_2", feature = "dox"))]
pub use self::authentication_request::{
    AuthenticationRequest, AuthenticationRequestClass, NONE_AUTHENTICATION_REQUEST,
};

mod back_forward_list;
pub use self::back_forward_list::BackForwardListExt;
pub use self::back_forward_list::{BackForwardList, BackForwardListClass, NONE_BACK_FORWARD_LIST};

mod back_forward_list_item;
pub use self::back_forward_list_item::BackForwardListItemExt;
pub use self::back_forward_list_item::{
    BackForwardListItem, BackForwardListItemClass, NONE_BACK_FORWARD_LIST_ITEM,
};

#[cfg(any(feature = "v2_8", feature = "dox"))]
mod color_chooser_request;
#[cfg(any(feature = "v2_8", feature = "dox"))]
pub use self::color_chooser_request::ColorChooserRequestExt;
#[cfg(any(feature = "v2_8", feature = "dox"))]
pub use self::color_chooser_request::{
    ColorChooserRequest, ColorChooserRequestClass, NONE_COLOR_CHOOSER_REQUEST,
};

mod context_menu;
pub use self::context_menu::ContextMenuExt;
pub use self::context_menu::{ContextMenu, ContextMenuClass, NONE_CONTEXT_MENU};

mod context_menu_item;
pub use self::context_menu_item::ContextMenuItemExt;
pub use self::context_menu_item::{ContextMenuItem, ContextMenuItemClass, NONE_CONTEXT_MENU_ITEM};

mod cookie_manager;
pub use self::cookie_manager::CookieManagerExt;
pub use self::cookie_manager::{CookieManager, CookieManagerClass, NONE_COOKIE_MANAGER};

mod download;
pub use self::download::DownloadExt;
pub use self::download::{Download, DownloadClass, NONE_DOWNLOAD};

#[cfg(any(feature = "v2_10", feature = "dox"))]
mod editor_state;
#[cfg(any(feature = "v2_10", feature = "dox"))]
pub use self::editor_state::EditorStateExt;
#[cfg(any(feature = "v2_10", feature = "dox"))]
pub use self::editor_state::{EditorState, EditorStateClass, NONE_EDITOR_STATE};

mod favicon_database;
pub use self::favicon_database::FaviconDatabaseExt;
pub use self::favicon_database::{FaviconDatabase, FaviconDatabaseClass, NONE_FAVICON_DATABASE};

mod file_chooser_request;
pub use self::file_chooser_request::FileChooserRequestExt;
pub use self::file_chooser_request::{
    FileChooserRequest, FileChooserRequestClass, NONE_FILE_CHOOSER_REQUEST,
};

mod find_controller;
pub use self::find_controller::FindControllerExt;
pub use self::find_controller::{FindController, FindControllerClass, NONE_FIND_CONTROLLER};

mod form_submission_request;
pub use self::form_submission_request::FormSubmissionRequestExt;
pub use self::form_submission_request::{
    FormSubmissionRequest, FormSubmissionRequestClass, NONE_FORM_SUBMISSION_REQUEST,
};

mod geolocation_permission_request;
pub use self::geolocation_permission_request::{
    GeolocationPermissionRequest, GeolocationPermissionRequestClass,
    NONE_GEOLOCATION_PERMISSION_REQUEST,
};

mod hit_test_result;
pub use self::hit_test_result::HitTestResultExt;
pub use self::hit_test_result::{HitTestResult, HitTestResultClass, NONE_HIT_TEST_RESULT};

#[cfg(any(feature = "v2_10", feature = "dox"))]
mod install_missing_media_plugins_permission_request;
#[cfg(any(feature = "v2_10", feature = "dox"))]
pub use self::install_missing_media_plugins_permission_request::InstallMissingMediaPluginsPermissionRequestExt;
#[cfg(any(feature = "v2_10", feature = "dox"))]
pub use self::install_missing_media_plugins_permission_request::{
    InstallMissingMediaPluginsPermissionRequest, InstallMissingMediaPluginsPermissionRequestClass,
    NONE_INSTALL_MISSING_MEDIA_PLUGINS_PERMISSION_REQUEST,
};

mod navigation_policy_decision;
pub use self::navigation_policy_decision::NavigationPolicyDecisionExt;
pub use self::navigation_policy_decision::{
    NavigationPolicyDecision, NavigationPolicyDecisionClass, NONE_NAVIGATION_POLICY_DECISION,
};

#[cfg(any(feature = "v2_8", feature = "dox"))]
mod notification;
#[cfg(any(feature = "v2_8", feature = "dox"))]
pub use self::notification::NotificationExt;
#[cfg(any(feature = "v2_8", feature = "dox"))]
pub use self::notification::{Notification, NotificationClass, NONE_NOTIFICATION};

mod notification_permission_request;
pub use self::notification_permission_request::{
    NotificationPermissionRequest, NotificationPermissionRequestClass,
    NONE_NOTIFICATION_PERMISSION_REQUEST,
};

mod permission_request;
pub use self::permission_request::PermissionRequestExt;
pub use self::permission_request::{PermissionRequest, NONE_PERMISSION_REQUEST};

mod plugin;
pub use self::plugin::PluginExt;
pub use self::plugin::{Plugin, PluginClass, NONE_PLUGIN};

mod policy_decision;
pub use self::policy_decision::PolicyDecisionExt;
pub use self::policy_decision::{PolicyDecision, PolicyDecisionClass, NONE_POLICY_DECISION};

#[cfg(any(feature = "v2_16", feature = "dox"))]
mod print_custom_widget;
#[cfg(any(feature = "v2_16", feature = "dox"))]
pub use self::print_custom_widget::PrintCustomWidgetExt;
#[cfg(any(feature = "v2_16", feature = "dox"))]
pub use self::print_custom_widget::{
    PrintCustomWidget, PrintCustomWidgetClass, NONE_PRINT_CUSTOM_WIDGET,
};

mod print_operation;
pub use self::print_operation::PrintOperationExt;
pub use self::print_operation::{PrintOperation, PrintOperationClass, NONE_PRINT_OPERATION};

mod response_policy_decision;
pub use self::response_policy_decision::ResponsePolicyDecisionExt;
pub use self::response_policy_decision::{
    ResponsePolicyDecision, ResponsePolicyDecisionClass, NONE_RESPONSE_POLICY_DECISION,
};

mod security_manager;
pub use self::security_manager::SecurityManagerExt;
pub use self::security_manager::{SecurityManager, SecurityManagerClass, NONE_SECURITY_MANAGER};

mod settings;
pub use self::settings::SettingsExt;
pub use self::settings::{Settings, SettingsClass, NONE_SETTINGS};

mod uri_request;
pub use self::uri_request::URIRequestExt;
pub use self::uri_request::{URIRequest, URIRequestClass, NONE_URI_REQUEST};

mod uri_response;
pub use self::uri_response::URIResponseExt;
pub use self::uri_response::{URIResponse, URIResponseClass, NONE_URI_RESPONSE};

mod uri_scheme_request;
pub use self::uri_scheme_request::URISchemeRequestExt;
pub use self::uri_scheme_request::{
    URISchemeRequest, URISchemeRequestClass, NONE_URI_SCHEME_REQUEST,
};

#[cfg(any(feature = "v2_6", feature = "dox"))]
mod user_content_manager;
#[cfg(any(feature = "v2_6", feature = "dox"))]
pub use self::user_content_manager::UserContentManagerExt;
#[cfg(any(feature = "v2_6", feature = "dox"))]
pub use self::user_content_manager::{
    UserContentManager, UserContentManagerClass, NONE_USER_CONTENT_MANAGER,
};

mod user_media_permission_request;
pub use self::user_media_permission_request::UserMediaPermissionRequestExt;
pub use self::user_media_permission_request::{
    UserMediaPermissionRequest, UserMediaPermissionRequestClass, NONE_USER_MEDIA_PERMISSION_REQUEST,
};

mod web_context;
pub use self::web_context::WebContextExt;
pub use self::web_context::{WebContext, WebContextClass, NONE_WEB_CONTEXT};

mod web_inspector;
pub use self::web_inspector::WebInspectorExt;
pub use self::web_inspector::{WebInspector, WebInspectorClass, NONE_WEB_INSPECTOR};

mod web_resource;
pub use self::web_resource::WebResourceExt;
pub use self::web_resource::{WebResource, WebResourceClass, NONE_WEB_RESOURCE};

mod web_view;
pub use self::web_view::WebViewExt;
pub use self::web_view::{WebView, WebViewClass, NONE_WEB_VIEW};

mod web_view_base;
pub use self::web_view_base::{WebViewBase, WebViewBaseClass, NONE_WEB_VIEW_BASE};

#[cfg(any(feature = "v2_10", feature = "dox"))]
mod website_data_manager;
#[cfg(any(feature = "v2_10", feature = "dox"))]
pub use self::website_data_manager::WebsiteDataManagerExt;
#[cfg(any(feature = "v2_10", feature = "dox"))]
pub use self::website_data_manager::{
    WebsiteDataManager, WebsiteDataManagerClass, NONE_WEBSITE_DATA_MANAGER,
};

mod window_properties;
pub use self::window_properties::WindowPropertiesExt;
pub use self::window_properties::{
    WindowProperties, WindowPropertiesClass, NONE_WINDOW_PROPERTIES,
};

#[cfg(any(feature = "v2_2", feature = "dox"))]
mod credential;
#[cfg(any(feature = "v2_2", feature = "dox"))]
pub use self::credential::Credential;

mod javascript_result;
pub use self::javascript_result::JavascriptResult;

mod mime_info;
pub use self::mime_info::MimeInfo;

#[cfg(any(feature = "v2_6", feature = "dox"))]
mod navigation_action;
#[cfg(any(feature = "v2_6", feature = "dox"))]
pub use self::navigation_action::NavigationAction;

#[cfg(any(feature = "v2_16", feature = "dox"))]
mod network_proxy_settings;
#[cfg(any(feature = "v2_16", feature = "dox"))]
pub use self::network_proxy_settings::NetworkProxySettings;

#[cfg(any(feature = "v2_16", feature = "dox"))]
mod security_origin;
#[cfg(any(feature = "v2_16", feature = "dox"))]
pub use self::security_origin::SecurityOrigin;

#[cfg(any(feature = "v2_6", feature = "dox"))]
mod user_script;
#[cfg(any(feature = "v2_6", feature = "dox"))]
pub use self::user_script::UserScript;

#[cfg(any(feature = "v2_6", feature = "dox"))]
mod user_style_sheet;
#[cfg(any(feature = "v2_6", feature = "dox"))]
pub use self::user_style_sheet::UserStyleSheet;

#[cfg(any(feature = "v2_12", feature = "dox"))]
mod web_view_session_state;
#[cfg(any(feature = "v2_12", feature = "dox"))]
pub use self::web_view_session_state::WebViewSessionState;

mod enums;
#[cfg(any(feature = "v2_2", feature = "dox"))]
pub use self::enums::AuthenticationScheme;
pub use self::enums::CacheModel;
pub use self::enums::ContextMenuAction;
pub use self::enums::CookieAcceptPolicy;
pub use self::enums::CookiePersistentStorage;
#[cfg(any(feature = "v2_2", feature = "dox"))]
pub use self::enums::CredentialPersistence;
pub use self::enums::DownloadError;
pub use self::enums::FaviconDatabaseError;
#[cfg(any(feature = "v2_16", feature = "dox"))]
pub use self::enums::HardwareAccelerationPolicy;
pub use self::enums::InsecureContentEvent;
pub use self::enums::JavascriptError;
pub use self::enums::LoadEvent;
pub use self::enums::NavigationType;
pub use self::enums::NetworkError;
#[cfg(any(feature = "v2_16", feature = "dox"))]
pub use self::enums::NetworkProxyMode;
pub use self::enums::PluginError;
pub use self::enums::PolicyDecisionType;
pub use self::enums::PolicyError;
pub use self::enums::PrintError;
pub use self::enums::PrintOperationResponse;
#[cfg(any(feature = "v2_4", feature = "dox"))]
pub use self::enums::ProcessModel;
pub use self::enums::SaveMode;
pub use self::enums::ScriptDialogType;
pub use self::enums::SnapshotError;
pub use self::enums::SnapshotRegion;
pub use self::enums::TLSErrorsPolicy;
#[cfg(any(feature = "v2_6", feature = "dox"))]
pub use self::enums::UserContentInjectedFrames;
#[cfg(any(feature = "v2_6", feature = "dox"))]
pub use self::enums::UserScriptInjectionTime;
#[cfg(any(feature = "v2_6", feature = "dox"))]
pub use self::enums::UserStyleLevel;

mod flags;
#[cfg(any(feature = "v2_10", feature = "dox"))]
pub use self::flags::EditorTypingAttributes;
pub use self::flags::FindOptions;
pub use self::flags::HitTestResultContext;
pub use self::flags::SnapshotOptions;
#[cfg(any(feature = "v2_16", feature = "dox"))]
pub use self::flags::WebsiteDataTypes;

#[doc(hidden)]
pub mod traits {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    pub use super::AuthenticationRequestExt;
    pub use super::BackForwardListExt;
    pub use super::BackForwardListItemExt;
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    pub use super::ColorChooserRequestExt;
    pub use super::ContextMenuExt;
    pub use super::ContextMenuItemExt;
    pub use super::CookieManagerExt;
    pub use super::DownloadExt;
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    pub use super::EditorStateExt;
    pub use super::FaviconDatabaseExt;
    pub use super::FileChooserRequestExt;
    pub use super::FindControllerExt;
    pub use super::FormSubmissionRequestExt;
    pub use super::HitTestResultExt;
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    pub use super::InstallMissingMediaPluginsPermissionRequestExt;
    pub use super::NavigationPolicyDecisionExt;
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    pub use super::NotificationExt;
    pub use super::PermissionRequestExt;
    pub use super::PluginExt;
    pub use super::PolicyDecisionExt;
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub use super::PrintCustomWidgetExt;
    pub use super::PrintOperationExt;
    pub use super::ResponsePolicyDecisionExt;
    pub use super::SecurityManagerExt;
    pub use super::SettingsExt;
    pub use super::URIRequestExt;
    pub use super::URIResponseExt;
    pub use super::URISchemeRequestExt;
    #[cfg(any(feature = "v2_6", feature = "dox"))]
    pub use super::UserContentManagerExt;
    pub use super::UserMediaPermissionRequestExt;
    pub use super::WebContextExt;
    pub use super::WebInspectorExt;
    pub use super::WebResourceExt;
    pub use super::WebViewExt;
    #[cfg(any(feature = "v2_10", feature = "dox"))]
    pub use super::WebsiteDataManagerExt;
    pub use super::WindowPropertiesExt;
}
