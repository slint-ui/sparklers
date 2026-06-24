use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};
use objc2_foundation::{NSDate, NSNumber, NSString, NSURL};

pub mod keys;
pub mod notifications;
pub mod updater;

extern_class!(
    // NOT `MainThreadOnly` - see `@unchecked Sendable` here https://sparkle-project.org/documentation/api-reference/Classes/SUAppcastItem.html
    #[unsafe(super(NSObject))]
    #[name = "SUAppcastItem"]
    #[derive(Debug)]
    pub struct SUAppcastItem;
);

// SAFETY: see `@unchecked Sendable` here https://sparkle-project.org/documentation/api-reference/Classes/SUAppcastItem.html
unsafe impl Send for SUAppcastItem {}
// TODO: This doesn't seem correct
unsafe impl Sync for SUAppcastItem {}

impl SUAppcastItem {
    extern_methods!(
        #[unsafe(method(displayVersionString))]
        pub fn display_version_string(&self) -> Retained<NSString>;

        #[unsafe(method(versionString))]
        pub fn version_string(&self) -> Retained<NSString>;

        #[unsafe(method(itemDescription))]
        pub fn item_description(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(fileURL))]
        pub fn file_url(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(contentLength))]
        pub fn content_length(&self) -> u64;

        #[unsafe(method(title))]
        pub fn title(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(releaseNotesURL))]
        pub fn release_notes_url(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(infoURL))]
        pub fn info_url(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(minimumSystemVersion))]
        pub fn minimum_system_version(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(isCriticalUpdate))]
        pub fn is_critical_update(&self) -> bool;

        #[unsafe(method(isMajorUpgrade))]
        pub fn is_major_upgrade(&self) -> bool;

        #[unsafe(method(channel))]
        pub fn channel(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(isInformationOnlyUpdate))]
        pub fn is_information_only_update(&self) -> bool;

        #[unsafe(method(date))]
        pub fn date(&self) -> Option<Retained<NSDate>>;

        #[unsafe(method(maximumSystemVersion))]
        pub fn maximum_system_version(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(minimumOperatingSystemVersionIsOK))]
        pub fn minimum_operating_system_version_is_ok(&self) -> bool;

        #[unsafe(method(maximumOperatingSystemVersionIsOK))]
        pub fn maximum_operating_system_version_is_ok(&self) -> bool;

        #[unsafe(method(installationType))]
        pub fn installation_type(&self) -> Retained<NSString>;

        #[unsafe(method(phasedRolloutInterval))]
        pub fn phased_rollout_interval(&self) -> Option<Retained<NSNumber>>;

        #[unsafe(method(fullReleaseNotesURL))]
        pub fn full_release_notes_url(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(minimumAutoupdateVersion))]
        pub fn minimum_autoupdate_version(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(ignoreSkippedUpgradesBelowVersion))]
        pub fn ignore_skipped_upgrades_below_version(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(dateString))]
        pub fn date_string(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(itemDescriptionFormat))]
        pub fn item_description_format(&self) -> Option<Retained<NSString>>;
    );
}
