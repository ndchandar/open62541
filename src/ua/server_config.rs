use std::{fmt, mem::MaybeUninit};

use open62541_sys::{UA_ServerConfig, UA_ServerConfig_clean, UA_ServerConfig_setDefault};

use crate::{ua, Error};

pub(crate) struct ServerConfig(Option<UA_ServerConfig>);

impl ServerConfig {
    /// Creates wrapper by taking ownership of value.
    ///
    /// When `Self` is dropped, allocations held by the inner type are cleaned up.
    ///
    /// # Safety
    ///
    /// Ownership of the value passes to `Self`. This must only be used for values that are not
    /// contained within other values that may be dropped.
    #[must_use]
    pub(crate) const unsafe fn from_raw(src: UA_ServerConfig) -> Self {
        Self(Some(src))
    }

    /// Gives up ownership and returns value.
    ///
    /// The returned value must be re-wrapped with [`from_raw()`], cleared manually, or copied into
    /// an owning value (like [`UA_Server`]) to free internal allocations and not leak memory.
    ///
    /// [`from_raw()`]: Self::from_raw
    /// [`UA_Server`]: open62541_sys::UA_Server
    #[must_use]
    pub(crate) fn into_raw(mut self) -> UA_ServerConfig {
        self.0.take().expect("should have server config")
    }

    /// Creates wrapper initialized with defaults.
    ///
    /// This initializes the value and makes all attributes well-defined. Additional attributes may
    /// need to be initialized for the value to be actually useful afterwards.
    pub(crate) const fn init() -> Self {
        let inner = MaybeUninit::<UA_ServerConfig>::zeroed();
        // SAFETY: Zero-initialized memory is a valid server config.
        let inner = unsafe { inner.assume_init() };
        // SAFETY: We pass a value without pointers to it into `Self`.
        unsafe { Self::from_raw(inner) }
    }

    /// Returns exclusive reference to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_mut(&mut self) -> &mut UA_ServerConfig {
        // PANIC: The inner object can only be unset when ownership has been given away.
        self.0.as_mut().expect("should have server config")
    }

    /// Returns mutable pointer to value.
    ///
    /// # Safety
    ///
    /// The value is owned by `Self`. Ownership must not be given away, in whole or in parts. This
    /// may happen when `open62541` functions are called that take ownership of values by pointer.
    #[must_use]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut UA_ServerConfig {
        // PANIC: The inner object can only be unset when ownership has been given away.
        self.0.as_mut().expect("should have server config")
    }
}

impl Drop for ServerConfig {
    fn drop(&mut self) {
        // Check if we still hold the server config. If not, we need not clean up: the ownership has
        // passed to the server that was created from this config.
        if let Some(mut inner) = self.0.take() {
            unsafe { UA_ServerConfig_clean(&mut inner) }
        }
    }
}

impl fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerConfig").finish_non_exhaustive()
    }
}

impl Default for ServerConfig {
    /// Creates a server config on the default port 4840 with no server certificate.
    fn default() -> Self {
        let mut config = Self::init();

        // Set custom logger first. This is necessary because the same logger instance is used as-is
        // inside derived attributes such as `eventLoop`, `certificateVerification`, etc.
        {
            let config = unsafe { config.as_mut() };
            // We assign a logger only on default-initialized config objects: we cannot know whether
            // an existing configuration is still referenced in another attribute or structure, thus
            // we could not (safely) free it anyway.
            debug_assert!(config.logging.is_null());
            // Create logger configuration. Ownership of the `UA_Logger` instance passes to `config`
            // at this point.
            config.logging = crate::logger();
        }

        // Set remaining attributes to their default values. This also copies the logger as laid out
        // above to other attributes inside `config` (cleaned up by `UA_ServerConfig_clean()`).
        let status_code =
            ua::StatusCode::new(unsafe { UA_ServerConfig_setDefault(config.as_mut_ptr()) });
        // PANIC: The only possible errors here are out-of-memory.
        Error::verify_good(&status_code).expect("should set default server config");

        config
    }
}
