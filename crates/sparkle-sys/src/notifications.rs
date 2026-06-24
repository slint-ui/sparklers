use objc2_foundation::NSString;

unsafe extern "C" {
    pub static SUUpdaterDidFindValidUpdateNotification: &'static NSString;
    pub static SUUpdaterDidFinishLoadingAppCastNotification: &'static NSString;
    pub static SUUpdaterDidNotFindUpdateNotification: &'static NSString;
    pub static SUUpdaterWillRestartNotification: &'static NSString;
}
