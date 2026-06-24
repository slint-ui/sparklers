use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{MainThreadOnly, extern_class, extern_methods};
use objc2_foundation::{NSDate, NSDictionary, NSError, NSString, NSURL};

extern_class!(
    /// A controller class that instantiates a `SPUUpdater` and allows binding UI to its updater settings.
    ///
    /// This class can be instantiated in a nib or created programmatically using `-initWithUpdaterDelegate:userDriverDelegate:` or `-initWithStartingUpdater:updaterDelegate:userDriverDelegate:`.
    ///
    /// The controller’s updater targets the application’s main bundle and uses Sparkle’s standard user interface. Typically, this class is used by sticking it as a custom [`NSObject`] subclass in an Interface Builder nib (probably in MainMenu) but it works well programmatically too.
    ///
    /// The controller creates an `SPUUpdater` instance using a `SPUStandardUserDriver` and allows hooking up the check for updates action and handling menu item validation. It also allows hooking up the updater’s and user driver’s delegates.
    ///
    /// If you need more control over what bundle you want to update, or you want to provide a custom user interface (via `SPUUserDriver`), please use `SPUUpdater` directly instead.
    ///
    /// This class must be used on the main thread.
    ///
    /// [Original documentation][original-docs]
    ///
    /// [original-docs]: https://sparkle-project.github.io/documentation/api-reference/Classes/SPUStandardUpdaterController.html
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "SPUStandardUpdaterController"]
    #[derive(Debug)]
    pub struct SPUStandardUpdaterController;
);

impl SPUStandardUpdaterController {
    extern_methods!(
        /// Create a new `SPUStandardUpdaterController` programmatically allowing you to specify
        /// whether or not to start the updater immediately.
        ///
        /// You can specify whether or not you want to start the updater immediately. If you do not
        /// start the updater, you must invoke -startUpdater at a later time to start it.
        ///
        /// Note the `updaterDelegate` and `userDriverDelegate` are weakly referenced, so you are
        /// responsible for keeping them alive.
        ///
        /// [Original documentation][original-docs]
        ///
        /// [original-docs]: https://sparkle-project.github.io/documentation/api-reference/Classes/SPUStandardUpdaterController.html#/c:objc(cs)SPUStandardUpdaterController(im)initWithStartingUpdater:updaterDelegate:userDriverDelegate:
        #[unsafe(method(initWithStartingUpdater:updaterDelegate:userDriverDelegate:))]
        pub fn init_with_starting_updater(
            this: objc2::rc::Allocated<Self>,
            starting_updater: bool,
            updater_delegate: Option<&NSObject>,
            user_driver_delegate: Option<&NSObject>,
        ) -> Retained<Self>;

        /// Accessible property for the updater. Some properties on the updater can be binded via
        /// KVO
        ///
        /// When instantiated from a nib, don’t perform update checks before the application has
        /// finished launching in a MainMenu nib (i.e `applicationDidFinishLaunching:`) or before
        /// the corresponding window/view controller has been loaded (i.e, `windowDidLoad` or
        /// `viewDidLoad`). The updater is not guaranteed to be started yet before these points.
        ///
        /// [Original documentation][original-docs]
        ///
        /// [original-docs]: https://sparkle-project.github.io/documentation/api-reference/Classes/SPUStandardUpdaterController.html#/c:objc(cs)SPUStandardUpdaterController(py)updater
        #[unsafe(method(updater))]
        pub fn updater(&self) -> Retained<SPUUpdater>;

        /// Explicitly checks for updates and displays a progress dialog while doing so.
        ///
        /// This method is meant for a main menu item. Connect any `NSMenuItem` to this action in Interface Builder or programmatically, and Sparkle will check for updates and report back its findings verbosely when it is invoked.
        ///
        /// When the target/action of the menu item is set to this controller and this method, this controller also handles enabling/disabling the menu item by checking `-[SPUUpdater canCheckForUpdates]`
        ///
        /// This action checks updates by invoking `-[SPUUpdater checkForUpdates]`
        ///
        /// [Original docomentation][original-docs]
        ///
        /// [original-docs]: https://sparkle-project.github.io/documentation/api-reference/Classes/SPUStandardUpdaterController.html#/c:objc(cs)SPUStandardUpdaterController(im)checkForUpdates:
        #[unsafe(method(checkForUpdates:))]
        pub fn check_for_updates(&self, sender: Option<&NSObject>);
    );
}

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "SPUUpdater"]
    #[derive(Debug)]
    pub struct SPUUpdater;
);

impl SPUUpdater {
    extern_methods!(
        #[unsafe(method(canCheckForUpdates))]
        pub fn can_check_for_updates(&self) -> bool;

        #[unsafe(method(checkForUpdates))]
        pub fn check_for_updates(&self);

        #[unsafe(method(checkForUpdatesInBackground))]
        pub fn check_for_updates_in_background(&self);

        #[unsafe(method(feedURL))]
        pub fn feed_url(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(setFeedURL:))]
        pub fn set_feed_url(&self, url: Option<&NSURL>);

        #[unsafe(method(automaticallyChecksForUpdates))]
        pub fn automatically_checks_for_updates(&self) -> bool;

        #[unsafe(method(setAutomaticallyChecksForUpdates:))]
        pub fn set_automatically_checks_for_updates(&self, enabled: bool);

        #[unsafe(method(automaticallyDownloadsUpdates))]
        pub fn automatically_downloads_updates(&self) -> bool;

        #[unsafe(method(setAutomaticallyDownloadsUpdates:))]
        pub fn set_automatically_downloads_updates(&self, enabled: bool);

        #[unsafe(method(lastUpdateCheckDate))]
        pub fn last_update_check_date(&self) -> Option<Retained<NSDate>>;

        #[unsafe(method(resetUpdateCycle))]
        pub fn reset_update_cycle(&self);

        #[unsafe(method(updateCheckInterval))]
        pub fn update_check_interval(&self) -> f64;

        #[unsafe(method(setUpdateCheckInterval:))]
        pub fn set_update_check_interval(&self, interval: f64);

        #[unsafe(method(checkForUpdateInformation))]
        pub fn check_for_update_information(&self);

        #[unsafe(method(sessionInProgress))]
        pub fn session_in_progress(&self) -> bool;

        #[unsafe(method(httpHeaders))]
        pub fn http_headers(&self) -> Option<Retained<NSDictionary<NSString, NSString>>>;

        #[unsafe(method(setHttpHeaders:))]
        pub fn set_http_headers(&self, headers: Option<&NSDictionary<NSString, NSString>>);

        #[unsafe(method(userAgentString))]
        pub fn user_agent_string(&self) -> Retained<NSString>;

        #[unsafe(method(setUserAgentString:))]
        pub fn set_user_agent_string(&self, user_agent: &NSString);

        #[unsafe(method(sendsSystemProfile))]
        pub fn sends_system_profile(&self) -> bool;

        #[unsafe(method(setSendsSystemProfile:))]
        pub fn set_sends_system_profile(&self, sends: bool);

        #[unsafe(method(clearFeedURLFromUserDefaults))]
        pub fn clear_feed_url_from_user_defaults(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(resetUpdateCycleAfterShortDelay))]
        pub fn reset_update_cycle_after_short_delay(&self);

        #[unsafe(method(startUpdater:))]
        pub fn start_updater(&self, error: *mut *mut NSError) -> bool;

        #[unsafe(method(setDelegate:))]
        pub fn set_delegate(&self, delegate: Option<&NSObject>);
    );
}
