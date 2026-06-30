use std::cell::RefCell;
use std::collections::HashMap;

use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{define_class, msg_send, DeclaredClass, MainThreadMarker, MainThreadOnly};
use objc2_foundation::{
    NSArray, NSDictionary, NSError, NSMutableSet, NSMutableURLRequest, NSSet, NSString,
};
use sparklers_sys::SUAppcastItem;

use crate::events::{Event, SparkleError};

pub type EventCallback = Box<dyn Fn(Event<'_>)>;

pub struct DelegateIvars {
    event_callback: RefCell<EventCallback>,
    allowed_channels: RefCell<Option<Vec<String>>>,
    feed_url_override: RefCell<Option<String>>,
    feed_parameters: RefCell<HashMap<String, String>>,
    should_download_release_notes: RefCell<bool>,
    should_relaunch: RefCell<bool>,
    may_check_for_updates: RefCell<bool>,
    should_proceed_with_update: RefCell<bool>,
    decryption_password: RefCell<Option<String>>,
    download_request_headers: RefCell<HashMap<String, String>>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "SparklersDelegate"]
    #[ivars = DelegateIvars]
    pub struct SparkleDelegate;

    impl SparkleDelegate {
        #[unsafe(method(updater:didFinishLoadingAppcast:))]
        fn updater_did_finish_loading_appcast(
            &self,
            _updater: &NSObject,
            _appcast: &NSObject,
        ) {
            self.emit(Event::DidFinishLoadingAppCast);
        }

        #[unsafe(method(updater:didFindValidUpdate:))]
        fn updater_did_find_valid_update(
            &self,
            _updater: &NSObject,
            item: &SUAppcastItem,
        ) {
            // TODO: Reimplement this
            // *self.ivars().last_found_update.borrow_mut() = Some(update_info.clone());
            self.emit(Event::DidFindValidUpdate { item: item.into() });
        }

        #[unsafe(method(updaterDidNotFindUpdate:))]
        fn updater_did_not_find_update(&self, _updater: &NSObject) {
            self.emit(Event::DidNotFindUpdate);
        }

        #[unsafe(method(updater:willDownloadUpdate:withRequest:))]
        fn updater_will_download_update(
            &self,
            _updater: &NSObject,
            item: &SUAppcastItem,
            request: &NSMutableURLRequest,
        ) {
            for (key, value) in self.ivars().download_request_headers.borrow().iter() {
                let ns_value = NSString::from_str(value);
                let ns_field = NSString::from_str(key);

                request.setValue_forHTTPHeaderField(Some(&ns_value), &ns_field);
            }

            self.emit(Event::WillDownloadUpdate {
                item: item.into(),
                request,
            });
        }

        #[unsafe(method(updater:didDownloadUpdate:))]
        fn updater_did_download_update(
            &self,
            _updater: &NSObject,
            item: &SUAppcastItem,
        ) {
            self.emit(Event::DidDownloadUpdate { item: item.into() });
        }

        #[unsafe(method(updater:willInstallUpdate:))]
        fn updater_will_install_update(
            &self,
            _updater: &NSObject,
            item: &SUAppcastItem,
        ) {
            self.emit(Event::WillInstallUpdate{item:item.into() });
        }

        #[unsafe(method(updater:didAbortWithError:))]
        fn updater_did_abort_with_error(
            &self,
            _updater: &NSObject,
            ns_error: &NSError,
        ) {
            self.emit(Event::DidAbortWithError{error: ns_error.into() });
        }

        #[unsafe(method(updater:didFinishUpdateCycleForUpdateCheck:error:))]
        fn updater_did_finish_update_cycle(
            &self,
            _updater: &NSObject,
            update_check: isize,
            error: Option<&NSError>,
        ) {
            self.emit(Event::DidFinishUpdateCycle{
               kind: update_check.into(),
                error: error.map(SparkleError::from),
            });
        }

        #[unsafe(method(updater:failedToDownloadUpdate:error:))]
        fn updater_failed_to_download_update(
            &self,
            _updater: &NSObject,
            item: &SUAppcastItem,
            ns_error: &NSError,
        ) {
            self.emit(Event::FailedToDownloadUpdate{
                item: item.into(),
                error: ns_error.into(),
            });
        }

        #[unsafe(method(userDidCancelDownload:))]
        fn user_did_cancel_download(&self, _updater: &NSObject) {
            self.emit(Event::UserDidCancelDownload);
        }

        #[unsafe(method(updater:willExtractUpdate:))]
        fn updater_will_extract_update(&self, _updater: &NSObject, item: &SUAppcastItem) {
            self.emit(Event::WillExtractUpdate { item: item.into() });
        }

        #[unsafe(method(updater:didExtractUpdate:))]
        fn updater_did_extract_update(&self, _updater: &NSObject, item: &SUAppcastItem) {
            self.emit(Event::DidExtractUpdate { item: item.into() });
        }

        #[unsafe(method(updaterWillRelaunchApplication:))]
        fn updater_will_relaunch_application(&self, _updater: &NSObject) {
            self.emit(Event::WillRelaunchApplication);
        }

        #[unsafe(method(updater:userDidMakeChoice:forUpdate:state:))]
        fn updater_user_did_make_choice(
            &self,
            _updater: &NSObject,
            choice: isize,
            item: &SUAppcastItem,
            state: isize,
        ) {
            self.emit(Event::UserDidMakeChoice{
                item: item.into(),
                choice: choice.into(),
                state: state.into(),
            });
        }

        #[unsafe(method(updater:willScheduleUpdateCheckAfterDelay:))]
        fn updater_will_schedule_update_check(&self, _updater: &NSObject, delay_secs: f64) {
            self.emit(Event::WillScheduleUpdateCheck { delay_secs });
        }

        #[unsafe(method(updaterWillNotScheduleUpdateCheck:))]
        fn updater_will_not_schedule_update_check(&self, _updater: &NSObject) {
            self.emit(Event::WillNotScheduleUpdateCheck);
        }

        #[unsafe(method(updaterShouldPromptForPermissionToCheckForUpdates:))]
        fn updater_should_prompt_for_permission(&self, _updater: &NSObject) -> bool {
            true
        }

        #[unsafe(method(updater:willInstallUpdateOnQuit:immediateInstallationBlock:))]
        fn updater_will_install_update_on_quit(
            &self,
            _updater: &NSObject,
            item: &SUAppcastItem,
            _handler: &NSObject,
        ) -> bool {
            self.emit(Event::WillInstallUpdateOnQuit {
                item: item.into(),
            });

            // Returning `true` here means that Sparkle will continue the update loop,
            // `false` would mean that we want to handle the update immediately
            true
        }

        #[unsafe(method(allowedChannelsForUpdater:))]
        fn allowed_channels_for_updater(
            &self,
            _updater: &NSObject,
        ) -> *mut NSSet<NSString> {
            let channels = self.ivars().allowed_channels.borrow();
            // TODO: We shouldn't need `NSMutableSet` here
            match channels.as_ref() {
                Some(ch) => {
                    let set = NSMutableSet::<NSString>::new();
                    for channel in ch {
                        let ns_str = NSString::from_str(channel);
                        let _: () = unsafe { msg_send![&set, addObject: &*ns_str] };
                    }
                    Retained::autorelease_return(Retained::into_super(set))
                }
                None => std::ptr::null_mut(),
            }
        }

        #[unsafe(method(feedURLStringForUpdater:))]
        fn feed_url_string_for_updater(
            &self,
            _updater: &NSObject,
        ) -> *mut NSString {
            let url = self.ivars().feed_url_override.borrow();
            match url.as_ref() {
                Some(u) => Retained::autorelease_return(NSString::from_str(u)),
                None => std::ptr::null_mut(),
            }
        }

        #[unsafe(method(feedParametersForUpdater:sendingSystemProfile:))]
        fn feed_parameters_for_updater(
            &self,
            _updater: &NSObject,
            _sending_profile: bool,
        ) -> *mut NSArray<NSDictionary<NSString, NSString>> {
            let params = self.ivars().feed_parameters.borrow();
            let array =
                if params.is_empty() {
                    NSArray::new()
                } else {
                    let mut dicts: Vec<Retained<NSDictionary<NSString, NSString>>> = Vec::new();
                    for (key, value) in params.iter() {
                        let key_str = NSString::from_str("key");
                        let value_str = NSString::from_str("value");
                        let k = NSString::from_str(key);
                        let v = NSString::from_str(value);
                        let dict = NSDictionary::from_slices(
                            &[&*key_str, &*value_str],
                            &[&*k, &*v],
                        );
                        dicts.push(dict);
                    }
                    let refs: Vec<&NSDictionary<NSString, NSString>> =
                        dicts.iter().map(|d| d.as_ref()).collect();
                    NSArray::from_slice(&refs)
                };
            Retained::autorelease_return(array)
        }

        #[unsafe(method(updater:shouldDownloadReleaseNotesForUpdate:))]
        fn updater_should_download_release_notes(
            &self,
            _updater: &NSObject,
            _item: &SUAppcastItem,
        ) -> bool {
            *self.ivars().should_download_release_notes.borrow()
        }

        #[unsafe(method(updaterShouldRelaunchApplication:))]
        fn updater_should_relaunch_application(&self, _updater: &NSObject) -> bool {
            *self.ivars().should_relaunch.borrow()
        }

        #[unsafe(method(updater:mayPerformUpdateCheck:error:))]
        fn updater_may_perform_update_check(
            &self,
            _updater: &NSObject,
            _update_check: isize,
            _error: *mut *mut NSObject,
        ) -> bool {
            *self.ivars().may_check_for_updates.borrow()
        }

        #[unsafe(method(updater:shouldProceedWithUpdate:updateCheck:error:))]
        fn updater_should_proceed_with_update(
            &self,
            _updater: &NSObject,
            _item: &SUAppcastItem,
            _update_check: isize,
            _error: *mut *mut NSObject,
        ) -> bool {
            *self.ivars().should_proceed_with_update.borrow()
        }

        #[unsafe(method(decryptionPasswordForUpdater:))]
        fn decryption_password_for_updater(
            &self,
            _updater: &NSObject,
        ) -> *mut NSString {
            let password = self.ivars().decryption_password.borrow();
            match password.as_ref() {
                Some(p) => Retained::autorelease_return(NSString::from_str(p)),
                None => std::ptr::null_mut(),
            }
        }
    }
);

impl SparkleDelegate {
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm);
        let this = this.set_ivars(DelegateIvars {
            event_callback: RefCell::new(Box::new(|_| {})),
            allowed_channels: RefCell::new(None),
            feed_url_override: RefCell::new(None),
            feed_parameters: Default::default(),
            should_download_release_notes: RefCell::new(true),
            should_relaunch: RefCell::new(true),
            may_check_for_updates: RefCell::new(true),
            should_proceed_with_update: RefCell::new(true),
            decryption_password: RefCell::new(None),
            // last_found_update: RefCell::new(None),
            download_request_headers: Default::default(),
        });
        unsafe { msg_send![super(this), init] }
    }

    pub fn set_event_callback(&self, callback: EventCallback) {
        *self.ivars().event_callback.borrow_mut() = callback;
    }

    fn emit(&self, event: Event) {
        (self.ivars().event_callback.borrow())(event)
    }

    pub fn allowed_channels(&self) -> Option<Vec<String>> {
        self.ivars().allowed_channels.borrow().clone()
    }

    pub fn set_allowed_channels(&self, channels: Option<Vec<String>>) {
        *self.ivars().allowed_channels.borrow_mut() = channels;
    }

    pub fn feed_url_override(&self) -> Option<String> {
        self.ivars().feed_url_override.borrow().clone()
    }

    pub fn set_feed_url_override(&self, url: Option<String>) {
        *self.ivars().feed_url_override.borrow_mut() = url;
    }

    pub fn feed_parameters(&self) -> HashMap<String, String> {
        self.ivars().feed_parameters.borrow().clone()
    }

    pub fn set_feed_parameters(&self, params: HashMap<String, String>) {
        *self.ivars().feed_parameters.borrow_mut() = params;
    }

    pub fn should_download_release_notes(&self) -> bool {
        *self.ivars().should_download_release_notes.borrow()
    }

    pub fn set_should_download_release_notes(&self, enabled: bool) {
        *self.ivars().should_download_release_notes.borrow_mut() = enabled;
    }

    pub fn should_relaunch(&self) -> bool {
        *self.ivars().should_relaunch.borrow()
    }

    pub fn set_should_relaunch(&self, enabled: bool) {
        *self.ivars().should_relaunch.borrow_mut() = enabled;
    }

    pub fn may_check_for_updates(&self) -> bool {
        *self.ivars().may_check_for_updates.borrow()
    }

    pub fn set_may_check_for_updates(&self, enabled: bool) {
        *self.ivars().may_check_for_updates.borrow_mut() = enabled;
    }

    pub fn should_proceed_with_update(&self) -> bool {
        *self.ivars().should_proceed_with_update.borrow()
    }

    pub fn set_should_proceed_with_update(&self, enabled: bool) {
        *self.ivars().should_proceed_with_update.borrow_mut() = enabled;
    }

    pub fn decryption_password(&self) -> Option<String> {
        self.ivars().decryption_password.borrow().clone()
    }

    pub fn set_decryption_password(&self, password: Option<String>) {
        *self.ivars().decryption_password.borrow_mut() = password;
    }

    pub fn download_request_headers(&self) -> HashMap<String, String> {
        self.ivars().download_request_headers.borrow().clone()
    }

    pub fn set_download_request_headers(&self, headers: HashMap<String, String>) {
        *self.ivars().download_request_headers.borrow_mut() = headers;
    }
}
