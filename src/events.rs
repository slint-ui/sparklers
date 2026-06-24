use std::ops::Deref;

use objc2::{msg_send, rc::Retained};
use objc2_foundation::{NSError, NSMutableURLRequest, NSNumber, NSString, NSURL};
use sparklers_sys::SUAppcastItem;

#[derive(Debug, Copy, Clone)]
pub struct AppcastItemRef<'a> {
    inner: &'a SUAppcastItem,
}

impl<'a> From<&'a SUAppcastItem> for AppcastItemRef<'a> {
    fn from(value: &'a SUAppcastItem) -> Self {
        Self { inner: value }
    }
}

fn url_to_string(url: &NSURL) -> String {
    let abs: Option<Retained<NSString>> = unsafe { msg_send![url, absoluteString] };
    abs.map(|s| s.to_string()).unwrap_or_default()
}

fn number_to_f64(num: &NSNumber) -> f64 {
    unsafe { msg_send![num, doubleValue] }
}

impl AppcastItemRef<'_> {
    pub fn version(&self) -> String {
        self.inner.display_version_string().to_string()
    }

    pub fn release_notes(&self) -> Option<String> {
        self.inner.item_description().map(|s| s.to_string())
    }

    pub fn title(&self) -> Option<String> {
        self.inner.title().map(|s| s.to_string())
    }

    pub fn release_notes_url(&self) -> Option<String> {
        self.inner.release_notes_url().map(|u| url_to_string(&u))
    }

    pub fn info_url(&self) -> Option<String> {
        self.inner.info_url().map(|u| url_to_string(&u))
    }

    pub fn minimum_system_version(&self) -> Option<String> {
        self.inner.minimum_system_version().map(|s| s.to_string())
    }

    pub fn channel(&self) -> Option<String> {
        self.inner.channel().map(|s| s.to_string())
    }

    pub fn date(&self) -> Option<f64> {
        self.inner.date().map(|d| {
            let seconds: f64 = unsafe { msg_send![&d, timeIntervalSince1970] };

            seconds * 1000.0
        })
    }

    pub fn is_critical(&self) -> bool {
        self.inner.is_critical_update()
    }

    pub fn is_major_upgrade(&self) -> bool {
        self.inner.is_major_upgrade()
    }

    pub fn is_information_only(&self) -> bool {
        self.inner.is_information_only_update()
    }

    pub fn maximum_system_version(&self) -> Option<String> {
        self.inner.maximum_system_version().map(|s| s.to_string())
    }

    pub fn minimum_os_version_ok(&self) -> bool {
        self.inner.minimum_operating_system_version_is_ok()
    }

    pub fn maximum_os_version_ok(&self) -> bool {
        self.inner.maximum_operating_system_version_is_ok()
    }

    pub fn installation_type(&self) -> String {
        self.inner.installation_type().to_string()
    }

    pub fn phased_rollout_interval(&self) -> Option<f64> {
        self.inner.phased_rollout_interval().map(|n| number_to_f64(&n))
    }

    pub fn full_release_notes_url(&self) -> Option<String> {
        self.inner.full_release_notes_url().map(|u| url_to_string(&u))
    }

    pub fn minimum_autoupdate_version(&self) -> Option<String> {
        self.inner.minimum_autoupdate_version().map(|s| s.to_string())
    }

    pub fn ignore_skipped_upgrades_below_version(&self) -> Option<String> {
        self.inner.ignore_skipped_upgrades_below_version().map(|s| s.to_string())
    }

    pub fn date_string(&self) -> Option<String> {
        self.inner.date_string().map(|s| s.to_string())
    }

    pub fn item_description_format(&self) -> Option<String> {
        self.inner.item_description_format().map(|s| s.to_string())
    }
}

impl Deref for AppcastItemRef<'_> {
    type Target = SUAppcastItem;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SparkleErrorRef<'a> {
    inner: &'a NSError,
}

impl SparkleErrorRef<'_> {
    pub fn message(&self) -> String {
        self.inner.localizedDescription().to_string()
    }

    pub fn code(&self) -> isize {
        self.inner.code()
    }

    pub fn domain(&self) -> String {
        self.inner.domain().to_string()
    }
}

impl<'a> From<&'a NSError> for SparkleErrorRef<'a> {
    fn from(value: &'a NSError) -> Self {
        Self { inner: value }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UpdateCheckKind {
    UserInitiated,
    Background,
    Other,
}

impl From<isize> for UpdateCheckKind {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::UserInitiated,
            1 => Self::Background,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UserChoice {
    Skip,
    Install,
    Dismiss,
}

impl From<isize> for UserChoice {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Skip,
            1 => Self::Install,
            // TODO: Is a glob match correct here? That's what the Tauri plugin did.
            _ => Self::Dismiss,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UserChoiceState {
    NotDownloaded,
    Downloaded,
    Installing,
}

impl From<isize> for UserChoiceState {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::NotDownloaded,
            1 => Self::Downloaded,
            // TODO: Is a glob match correct here? That's what the Tauri plugin did.
            _ => Self::Installing,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event<'a> {
    DidFindValidUpdate { item: AppcastItemRef<'a> },
    DidFinishLoadingAppCast,
    DidNotFindUpdate,
    WillRestart,
    WillDownloadUpdate { item: AppcastItemRef<'a>, request: &'a NSMutableURLRequest },
    DidDownloadUpdate { item: AppcastItemRef<'a> },
    WillInstallUpdate { item: AppcastItemRef<'a> },
    DidAbortWithError { error: SparkleErrorRef<'a> },
    DidFinishUpdateCycle { kind: UpdateCheckKind, error: Option<SparkleErrorRef<'a>> },
    FailedToDownloadUpdate { item: AppcastItemRef<'a>, error: SparkleErrorRef<'a> },
    UserDidCancelDownload,
    WillExtractUpdate { item: AppcastItemRef<'a> },
    DidExtractUpdate { item: AppcastItemRef<'a> },
    WillRelaunchApplication,
    UserDidMakeChoice { item: AppcastItemRef<'a>, choice: UserChoice, state: UserChoiceState },
    WillScheduleUpdateCheck { delay_secs: f64 },
    WillNotScheduleUpdateCheck,
    WillInstallUpdateOnQuit { item: AppcastItemRef<'a> },
}
