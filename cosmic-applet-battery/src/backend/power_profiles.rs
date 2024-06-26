//! # DBus interface proxy for: `org.freedesktop.UPower.PowerProfiles`
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
//! Source: `powerprofilesdaemon.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.UPower.PowerProfiles",
    default_path = "/org/freedesktop/UPower/PowerProfiles",
    assume_defaults = true
)]
trait PowerProfiles {
    /// HoldProfile method
    fn hold_profile(&self, profile: &str, reason: &str, application_id: &str) -> zbus::Result<u32>;

    /// ReleaseProfile method
    fn release_profile(&self, cookie: u32) -> zbus::Result<()>;

    /// ProfileReleased signal
    #[dbus_proxy(signal)]
    fn profile_released(&self, cookie: u32) -> zbus::Result<()>;

    /// Actions property
    #[dbus_proxy(property)]
    fn actions(&self) -> zbus::Result<Vec<String>>;

    /// ActiveProfile property
    #[dbus_proxy(property)]
    fn active_profile(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_active_profile(&self, value: &str) -> zbus::Result<()>;

    /// ActiveProfileHolds property
    #[dbus_proxy(property)]
    fn active_profile_holds(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// PerformanceDegraded property
    #[dbus_proxy(property)]
    fn performance_degraded(&self) -> zbus::Result<String>;

    /// PerformanceInhibited property
    #[dbus_proxy(property)]
    fn performance_inhibited(&self) -> zbus::Result<String>;

    /// Profiles property
    #[dbus_proxy(property)]
    fn profiles(
        &self,
    ) -> zbus::Result<Vec<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>>;

    /// Version property
    #[dbus_proxy(property)]
    fn version(&self) -> zbus::Result<String>;
}
