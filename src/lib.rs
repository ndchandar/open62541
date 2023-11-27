mod client;
mod error;
pub mod ua;

pub use self::{
    client::{Client, ClientBuilder},
    error::Error,
};

/// Transparent wrapper for OPC UA data type.
///
/// # Safety
///
/// We require that it must be possible to transmute between the type that implements `DataType` and
/// the wrapped type [`Self::Inner`]. Therefore, `#[repr(transparent)]` must be used when one wishes
/// to implement `DataType`.
pub(crate) unsafe trait DataType {
    /// Inner type.
    ///
    /// We require that it must be possible to transmute between the inner type and the wrapper type
    /// that implements `DataType`. This implies that `#[repr(transparent)]` must be set on any type
    /// that implements the `DataType` trait.
    type Inner;

    fn data_type() -> *const open62541_sys::UA_DataType;

    #[allow(dead_code)]
    #[must_use]
    fn as_ref(&self) -> &Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
        //
        // SAFETY: Dereferencing the pointer is allowed because of this transmutability.
        unsafe { &*(self as *const Self).cast::<Self::Inner>() }
    }

    #[allow(dead_code)]
    #[must_use]
    fn as_mut(&mut self) -> &mut Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
        //
        // SAFETY: Dereferencing the pointer is allowed because of this transmutability.
        unsafe { &mut *(self as *mut Self).cast::<Self::Inner>() }
    }

    #[allow(dead_code)]
    #[must_use]
    fn as_ptr(&self) -> *const Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
        (self as *const Self).cast::<Self::Inner>()
    }

    #[allow(dead_code)]
    #[must_use]
    fn as_mut_ptr(&mut self) -> *mut Self::Inner {
        // This transmutes the value into the inner type through `cast()`. Types that implement this
        // trait guarantee that we can transmute between them and their inner type, so this is okay.
        (self as *mut Self).cast::<Self::Inner>()
    }

    fn data_type_ref() -> &'static open62541_sys::UA_DataType {
        unsafe { Self::data_type().as_ref() }.unwrap()
    }
}
