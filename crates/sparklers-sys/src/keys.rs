use objc2_foundation::NSString;

unsafe extern "C" {
    pub static SPULatestAppcastItemFoundKey: &'static NSString;
    pub static SPUNoUpdateFoundReasonKey: &'static NSString;
    pub static SPUNoUpdateFoundUserInitiatedKey: &'static NSString;
    pub static SUSparkleErrorDomain: &'static NSString;
    pub static SUSystemProfilerApplicationNameKey: &'static NSString;
    pub static SUSystemProfilerApplicationVersionKey: &'static NSString;
    pub static SUSystemProfilerCPU64bitKey: &'static NSString;
    pub static SUSystemProfilerCPUCountKey: &'static NSString;
    pub static SUSystemProfilerCPUFrequencyKey: &'static NSString;
    pub static SUSystemProfilerCPUSubtypeKey: &'static NSString;
    pub static SUSystemProfilerCPUTypeKey: &'static NSString;
    pub static SUSystemProfilerHardwareModelKey: &'static NSString;
    pub static SUSystemProfilerMemoryKey: &'static NSString;
    pub static SUSystemProfilerOperatingSystemVersionKey: &'static NSString;
    pub static SUSystemProfilerPreferredLanguageKey: &'static NSString;
    pub static SUUpdaterAppcastItemNotificationKey: &'static NSString;
    pub static SUUpdaterAppcastNotificationKey: &'static NSString;
}
