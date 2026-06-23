use std::collections::HashMap;

use crate::events::UpdateInfo;
use crate::Error;
use crate::Result;
use crate::SparkleUpdaterExt;

macro_rules! get_updater {
    ($app:expr) => {
        match $app.sparkle_updater() {
            Some(updater) => updater,
            None => return Err(Error::UpdaterNotReady),
        }
    };
}

pub(crate) async fn check_for_updates<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    get_updater!(app).check_for_updates()
}

pub(crate) async fn check_for_updates_in_background<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    get_updater!(app).check_for_updates_in_background()
}

pub(crate) async fn can_check_for_updates<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    get_updater!(app).can_check_for_updates()
}

pub(crate) async fn current_version<R: Runtime>(app: AppHandle<R>) -> Result<String> {
    match app.sparkle_updater() {
        Some(updater) => updater.current_version(),
        None => Ok(app.package_info().version.to_string()),
    }
}

pub(crate) async fn feed_url<R: Runtime>(app: AppHandle<R>) -> Result<Option<String>> {
    get_updater!(app).feed_url()
}

pub(crate) async fn set_feed_url<R: Runtime>(app: AppHandle<R>, url: String) -> Result<()> {
    get_updater!(app).set_feed_url(&url)
}

pub(crate) async fn automatically_checks_for_updates<R: Runtime>(
    app: AppHandle<R>,
) -> Result<bool> {
    get_updater!(app).automatically_checks_for_updates()
}

pub(crate) async fn set_automatically_checks_for_updates<R: Runtime>(
    app: AppHandle<R>,
    enabled: bool,
) -> Result<()> {
    get_updater!(app).set_automatically_checks_for_updates(enabled)
}

pub(crate) async fn automatically_downloads_updates<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    get_updater!(app).automatically_downloads_updates()
}

pub(crate) async fn set_automatically_downloads_updates<R: Runtime>(
    app: AppHandle<R>,
    enabled: bool,
) -> Result<()> {
    get_updater!(app).set_automatically_downloads_updates(enabled)
}

pub(crate) async fn last_update_check_date<R: Runtime>(app: AppHandle<R>) -> Result<Option<f64>> {
    get_updater!(app).last_update_check_date()
}

pub(crate) async fn reset_update_cycle<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    get_updater!(app).reset_update_cycle()
}

pub(crate) async fn update_check_interval<R: Runtime>(app: AppHandle<R>) -> Result<f64> {
    get_updater!(app).update_check_interval()
}

pub(crate) async fn set_update_check_interval<R: Runtime>(
    app: AppHandle<R>,
    interval: f64,
) -> Result<()> {
    get_updater!(app).set_update_check_interval(interval)
}

pub(crate) async fn check_for_update_information<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    get_updater!(app).check_for_update_information()
}

pub(crate) async fn session_in_progress<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    get_updater!(app).session_in_progress()
}

pub(crate) async fn http_headers<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<HashMap<String, String>>> {
    get_updater!(app).http_headers()
}

pub(crate) async fn set_http_headers<R: Runtime>(
    app: AppHandle<R>,
    headers: Option<HashMap<String, String>>,
) -> Result<()> {
    get_updater!(app).set_http_headers(headers)
}

pub(crate) async fn user_agent_string<R: Runtime>(app: AppHandle<R>) -> Result<String> {
    get_updater!(app).user_agent_string()
}

pub(crate) async fn set_user_agent_string<R: Runtime>(
    app: AppHandle<R>,
    user_agent: String,
) -> Result<()> {
    get_updater!(app).set_user_agent_string(&user_agent)
}

pub(crate) async fn sends_system_profile<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    get_updater!(app).sends_system_profile()
}

pub(crate) async fn set_sends_system_profile<R: Runtime>(
    app: AppHandle<R>,
    sends: bool,
) -> Result<()> {
    get_updater!(app).set_sends_system_profile(sends)
}

pub(crate) async fn clear_feed_url_from_user_defaults<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<String>> {
    get_updater!(app).clear_feed_url_from_user_defaults()
}

pub(crate) async fn reset_update_cycle_after_short_delay<R: Runtime>(
    app: AppHandle<R>,
) -> Result<()> {
    get_updater!(app).reset_update_cycle_after_short_delay()
}

pub(crate) async fn allowed_channels<R: Runtime>(app: AppHandle<R>) -> Result<Option<Vec<String>>> {
    get_updater!(app).allowed_channels()
}

pub(crate) async fn set_allowed_channels<R: Runtime>(
    app: AppHandle<R>,
    channels: Option<Vec<String>>,
) -> Result<()> {
    get_updater!(app).set_allowed_channels(channels)
}

pub(crate) async fn feed_url_override<R: Runtime>(app: AppHandle<R>) -> Result<Option<String>> {
    get_updater!(app).feed_url_override()
}

pub(crate) async fn set_feed_url_override<R: Runtime>(
    app: AppHandle<R>,
    url: Option<String>,
) -> Result<()> {
    get_updater!(app).set_feed_url_override(url)
}

pub(crate) async fn feed_parameters<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<HashMap<String, String>>> {
    get_updater!(app).feed_parameters()
}

pub(crate) async fn set_feed_parameters<R: Runtime>(
    app: AppHandle<R>,
    params: Option<HashMap<String, String>>,
) -> Result<()> {
    get_updater!(app).set_feed_parameters(params)
}

pub(crate) async fn should_download_release_notes<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    get_updater!(app).should_download_release_notes()
}

pub(crate) async fn set_should_download_release_notes<R: Runtime>(
    app: AppHandle<R>,
    enabled: bool,
) -> Result<()> {
    get_updater!(app).set_should_download_release_notes(enabled)
}

pub(crate) async fn should_relaunch_application<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    get_updater!(app).should_relaunch_application()
}

pub(crate) async fn set_should_relaunch_application<R: Runtime>(
    app: AppHandle<R>,
    enabled: bool,
) -> Result<()> {
    get_updater!(app).set_should_relaunch_application(enabled)
}

pub(crate) async fn may_check_for_updates_config<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    get_updater!(app).may_check_for_updates_config()
}

pub(crate) async fn set_may_check_for_updates_config<R: Runtime>(
    app: AppHandle<R>,
    enabled: bool,
) -> Result<()> {
    get_updater!(app).set_may_check_for_updates_config(enabled)
}

pub(crate) async fn should_proceed_with_update<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    get_updater!(app).should_proceed_with_update()
}

pub(crate) async fn set_should_proceed_with_update<R: Runtime>(
    app: AppHandle<R>,
    enabled: bool,
) -> Result<()> {
    get_updater!(app).set_should_proceed_with_update(enabled)
}

pub(crate) async fn decryption_password<R: Runtime>(app: AppHandle<R>) -> Result<Option<String>> {
    get_updater!(app).decryption_password()
}

pub(crate) async fn set_decryption_password<R: Runtime>(
    app: AppHandle<R>,
    password: Option<String>,
) -> Result<()> {
    get_updater!(app).set_decryption_password(password)
}

pub(crate) async fn download_request_headers<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<HashMap<String, String>>> {
    get_updater!(app).download_request_headers()
}

pub(crate) async fn set_download_request_headers<R: Runtime>(
    app: AppHandle<R>,
    headers: Option<HashMap<String, String>>,
) -> Result<()> {
    get_updater!(app).set_download_request_headers(headers)
}

pub(crate) async fn last_found_update<R: Runtime>(app: AppHandle<R>) -> Result<Option<UpdateInfo>> {
    get_updater!(app).last_found_update()
}
