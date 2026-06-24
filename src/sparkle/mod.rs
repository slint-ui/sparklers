mod delegate;

use std::collections::HashMap;
use std::ptr;
use std::sync::Arc;

use dispatch2::{run_on_main, MainThreadBound};
use log::warn;
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{msg_send, ClassType, MainThreadMarker};
use objc2_foundation::{NSBundle, NSDictionary, NSError, NSString, NSURL};
use sparklers_sys::updater::{SPUStandardUpdaterController, SPUUpdater};

use crate::{Error, Event, Result};
use delegate::SparkleDelegate;

fn is_valid_bundle() -> bool {
    unsafe {
        let bundle = NSBundle::mainBundle();
        let identifier: Option<Retained<NSString>> = msg_send![&bundle, bundleIdentifier];
        match identifier {
            Some(id) => {
                let id_str = id.to_string();
                !id_str.is_empty() && id_str != "com.apple.dt.Xcode.tool"
            },
            None => false,
        }
    }
}

fn init_on_main_thread(mtm: MainThreadMarker) -> Result<MainThreadBound<SparkleUpdater>> {
    check_info_plist_keys();

    let delegate = SparkleDelegate::new(mtm);

    let controller = unsafe {
        let alloc: objc2::rc::Allocated<SPUStandardUpdaterController> =
            objc2::msg_send![SPUStandardUpdaterController::class(), alloc];
        let delegate_obj: &NSObject = &delegate;
        SPUStandardUpdaterController::init_with_starting_updater(
            alloc,
            false,
            Some(delegate_obj),
            None,
        )
    };

    let updater: Retained<SPUUpdater> = controller.updater();
    let mut error: *mut NSError = ptr::null_mut();
    let success = updater.start_updater(&mut error);

    if !success {
        if !error.is_null() {
            let ns_error = unsafe { &*error };
            let description: Retained<NSString> =
                unsafe { objc2::msg_send![ns_error, localizedDescription] };
            return Err(Error::SparkleInit(description.to_string()));
        }
        return Err(Error::SparkleInit("Failed to start updater".to_string()));
    }

    Ok(MainThreadBound::new(SparkleUpdater { controller, delegate }, mtm))
}

const PLIST_KEY_VALIDATIONS: &[(&str, &str)] = &[
    ("SUPublicEDKey", "Sparkle will not be able to verify update signatures."),
    ("SUFeedURL", "You must set a feed URL before checking for updates."),
];

fn check_info_plist_keys() {
    unsafe {
        let bundle = NSBundle::mainBundle();
        let info_dict: Option<Retained<NSDictionary>> = msg_send![&bundle, infoDictionary];

        if let Some(dict) = info_dict {
            for (key_name, warning) in PLIST_KEY_VALIDATIONS {
                let key = NSString::from_str(key_name);
                let value: Option<Retained<NSObject>> = msg_send![&dict, objectForKey: &*key];
                if value.is_none() {
                    warn!("{} not found in Info.plist. {}", key_name, warning);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SparkleConfig {
    pub version: String,
}

pub trait GetSparkleConfig {
    fn sparkle_config(&self) -> SparkleConfig;
}

impl<C> GetSparkleConfig for C
where
    C: AsRef<SparkleConfig>,
{
    fn sparkle_config(&self) -> SparkleConfig {
        self.as_ref().clone()
    }
}

pub struct Sparkle<C = SparkleConfig> {
    updater: MainThreadBound<SparkleUpdater>,
    config: C,
}

impl<C> Sparkle<C>
where
    C: Send + 'static,
{
    pub fn new(config: C) -> Result<Option<Self>> {
        if !is_valid_bundle() {
            warn!(
                "Sparkle updater disabled: not running inside a valid macOS bundle. This is \
                 expected during development. Sparkle will work in release builds."
            );
            return Ok(None);
        }

        run_on_main(|mtm| init_on_main_thread(mtm))
            .map(move |updater| Self { config, updater })
            .map(Some)
    }
}

pub struct SparkleUpdater {
    controller: Retained<SPUStandardUpdaterController>,
    delegate: Retained<SparkleDelegate>,
}

// TODO: Probably unnecessary?
impl<C> Sparkle<C>
where
    C: GetSparkleConfig,
{
    pub fn current_version(&self) -> Result<String> {
        Ok(self.config.sparkle_config().version)
    }
}

impl<C> Sparkle<C> {
    fn dispatch<T, F>(&self, f: F) -> T
    where
        T: Send,
        F: FnOnce(&SPUStandardUpdaterController) -> T + Send,
    {
        self.updater.get_on_main(|updater| f(&updater.controller))
    }

    fn dispatch_delegate<T, F>(&self, f: F) -> T
    where
        T: Send,
        F: FnOnce(&SparkleDelegate) -> T + Send,
    {
        self.updater.get_on_main(|updater| f(&updater.delegate))
    }

    pub fn check_for_updates(&self) -> Result<()> {
        self.dispatch(|c| c.check_for_updates(None));
        Ok(())
    }

    pub fn check_for_updates_in_background(&self) -> Result<()> {
        self.dispatch(|c| c.updater().check_for_updates_in_background());
        Ok(())
    }

    pub fn can_check_for_updates(&self) -> Result<bool> {
        Ok(self.dispatch(|c| c.updater().can_check_for_updates()))
    }

    pub fn feed_url(&self) -> Result<Option<String>> {
        Ok(self.dispatch(|c| {
            c.updater().feed_url().and_then(|url| {
                let abs: Option<Retained<NSString>> =
                    unsafe { objc2::msg_send![&url, absoluteString] };
                abs.map(|s| s.to_string())
            })
        }))
    }

    pub fn set_feed_url(&self, url: &str) -> Result<()> {
        url::Url::parse(url).map_err(|_| Error::InvalidFeedUrl(url.to_string()))?;
        let url_string = url.to_string();

        self.dispatch(move |c| {
            let ns_string = NSString::from_str(&url_string);
            let ns_url: Option<Retained<NSURL>> =
                unsafe { objc2::msg_send![NSURL::class(), URLWithString: &*ns_string] };
            if let Some(url) = ns_url {
                c.updater().set_feed_url(Some(&url));
            }
        });
        Ok(())
    }

    pub fn automatically_checks_for_updates(&self) -> Result<bool> {
        Ok(self.dispatch(|c| c.updater().automatically_checks_for_updates()))
    }

    pub fn set_automatically_checks_for_updates(&self, enabled: bool) -> Result<()> {
        self.dispatch(|c| c.updater().set_automatically_checks_for_updates(enabled));
        Ok(())
    }

    pub fn automatically_downloads_updates(&self) -> Result<bool> {
        Ok(self.dispatch(|c| c.updater().automatically_downloads_updates()))
    }

    pub fn set_automatically_downloads_updates(&self, enabled: bool) -> Result<()> {
        self.dispatch(|c| c.updater().set_automatically_downloads_updates(enabled));
        Ok(())
    }

    pub fn last_update_check_date(&self) -> Result<Option<f64>> {
        Ok(self.dispatch(|c| {
            c.updater().last_update_check_date().map(|date| {
                let seconds: f64 = unsafe { objc2::msg_send![&date, timeIntervalSince1970] };
                seconds * 1000.0
            })
        }))
    }

    pub fn reset_update_cycle(&self) -> Result<()> {
        self.dispatch(|c| c.updater().reset_update_cycle());
        Ok(())
    }

    pub fn update_check_interval(&self) -> Result<f64> {
        Ok(self.dispatch(|c| c.updater().update_check_interval()))
    }

    pub fn set_update_check_interval(&self, interval: f64) -> Result<()> {
        self.dispatch(|c| c.updater().set_update_check_interval(interval));
        Ok(())
    }

    pub fn check_for_update_information(&self) -> Result<()> {
        self.dispatch(|c| c.updater().check_for_update_information());
        Ok(())
    }

    pub fn session_in_progress(&self) -> Result<bool> {
        Ok(self.dispatch(|c| c.updater().session_in_progress()))
    }

    pub fn http_headers(&self) -> Result<Option<HashMap<String, String>>> {
        Ok(self.dispatch(|c| {
            c.updater().http_headers().map(|dict| {
                let mut map = HashMap::new();
                let count: usize = unsafe { objc2::msg_send![&dict, count] };
                if count > 0 {
                    let keys: Retained<objc2_foundation::NSArray<NSString>> =
                        unsafe { objc2::msg_send![&dict, allKeys] };
                    for i in 0..count {
                        let key: &NSString = unsafe { objc2::msg_send![&keys, objectAtIndex: i] };
                        let value: Option<Retained<NSString>> =
                            unsafe { objc2::msg_send![&dict, objectForKey: key] };
                        if let Some(v) = value {
                            map.insert(key.to_string(), v.to_string());
                        }
                    }
                }
                map
            })
        }))
    }

    pub fn set_http_headers(&self, headers: Option<HashMap<String, String>>) -> Result<()> {
        self.dispatch(move |c| {
            let ns_dict = headers.map(|h| {
                let keys: Vec<Retained<NSString>> =
                    h.keys().map(|k| NSString::from_str(k)).collect();
                let values: Vec<Retained<NSString>> =
                    h.values().map(|v| NSString::from_str(v)).collect();
                let key_refs: Vec<&NSString> = keys.iter().map(|k| k.as_ref()).collect();
                let value_refs: Vec<&NSString> = values.iter().map(|v| v.as_ref()).collect();
                NSDictionary::from_slices(&key_refs, &value_refs)
            });
            c.updater().set_http_headers(ns_dict.as_deref());
        });
        Ok(())
    }

    pub fn user_agent_string(&self) -> Result<String> {
        Ok(self.dispatch(|c| c.updater().user_agent_string().to_string()))
    }

    pub fn set_user_agent_string(&self, user_agent: &str) -> Result<()> {
        let ua = user_agent.to_string();
        self.dispatch(move |c| {
            let ns_string = NSString::from_str(&ua);
            c.updater().set_user_agent_string(&ns_string);
        });
        Ok(())
    }

    pub fn sends_system_profile(&self) -> Result<bool> {
        Ok(self.dispatch(|c| c.updater().sends_system_profile()))
    }

    pub fn set_sends_system_profile(&self, sends: bool) -> Result<()> {
        self.dispatch(|c| c.updater().set_sends_system_profile(sends));
        Ok(())
    }

    pub fn clear_feed_url_from_user_defaults(&self) -> Result<Option<String>> {
        Ok(self.dispatch(|c| {
            c.updater().clear_feed_url_from_user_defaults().and_then(|url| {
                let abs: Option<Retained<NSString>> =
                    unsafe { objc2::msg_send![&url, absoluteString] };
                abs.map(|s| s.to_string())
            })
        }))
    }

    pub fn reset_update_cycle_after_short_delay(&self) -> Result<()> {
        self.dispatch(|c| c.updater().reset_update_cycle_after_short_delay());
        Ok(())
    }

    pub fn allowed_channels(&self) -> Result<Option<Vec<String>>> {
        Ok(self.dispatch_delegate(|d| d.allowed_channels()))
    }

    pub fn set_allowed_channels(&self, channels: Option<Vec<String>>) -> Result<()> {
        self.dispatch_delegate(|d| d.set_allowed_channels(channels));
        Ok(())
    }

    pub fn feed_url_override(&self) -> Result<Option<String>> {
        Ok(self.dispatch_delegate(|d| d.feed_url_override()))
    }

    pub fn set_feed_url_override(&self, url: Option<String>) -> Result<()> {
        self.dispatch_delegate(|d| d.set_feed_url_override(url));
        Ok(())
    }

    pub fn feed_parameters(&self) -> Result<Option<HashMap<String, String>>> {
        Ok(self.dispatch_delegate(|d| d.feed_parameters()))
    }

    pub fn set_feed_parameters(&self, params: Option<HashMap<String, String>>) -> Result<()> {
        self.dispatch_delegate(|d| d.set_feed_parameters(params));
        Ok(())
    }

    pub fn should_download_release_notes(&self) -> Result<bool> {
        Ok(self.dispatch_delegate(|d| d.should_download_release_notes()))
    }

    pub fn set_should_download_release_notes(&self, enabled: bool) -> Result<()> {
        self.dispatch_delegate(|d| d.set_should_download_release_notes(enabled));
        Ok(())
    }

    pub fn should_relaunch_application(&self) -> Result<bool> {
        Ok(self.dispatch_delegate(|d| d.should_relaunch()))
    }

    pub fn set_should_relaunch_application(&self, enabled: bool) -> Result<()> {
        self.dispatch_delegate(|d| d.set_should_relaunch(enabled));
        Ok(())
    }

    pub fn may_check_for_updates_config(&self) -> Result<bool> {
        Ok(self.dispatch_delegate(|d| d.may_check_for_updates()))
    }

    pub fn set_may_check_for_updates_config(&self, enabled: bool) -> Result<()> {
        self.dispatch_delegate(|d| d.set_may_check_for_updates(enabled));
        Ok(())
    }

    pub fn should_proceed_with_update(&self) -> Result<bool> {
        Ok(self.dispatch_delegate(|d| d.should_proceed_with_update()))
    }

    pub fn set_should_proceed_with_update(&self, enabled: bool) -> Result<()> {
        self.dispatch_delegate(|d| d.set_should_proceed_with_update(enabled));
        Ok(())
    }

    pub fn decryption_password(&self) -> Result<Option<String>> {
        Ok(self.dispatch_delegate(|d| d.decryption_password()))
    }

    pub fn set_decryption_password(&self, password: Option<String>) -> Result<()> {
        self.dispatch_delegate(|d| d.set_decryption_password(password));
        Ok(())
    }

    pub fn set_event_callback(&self, callback: impl Fn(Event<'_>) + Send + Sync + 'static) {
        self.dispatch_delegate(|d| d.set_event_callback(Arc::new(callback)))
    }

    pub fn download_request_headers(&self) -> Result<Option<HashMap<String, String>>> {
        Ok(self.dispatch_delegate(|d| d.download_request_headers()))
    }

    pub fn set_download_request_headers(
        &self,
        headers: Option<HashMap<String, String>>,
    ) -> Result<()> {
        self.dispatch_delegate(|d| d.set_download_request_headers(headers));
        Ok(())
    }
}
