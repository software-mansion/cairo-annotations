use cairo_lang_sierra::debug_info::DebugInfo;
use serde::de::DeserializeOwned;

pub trait TryFromDebugInfo: Sized {
    type Error;

    /// Attempt to create an instance of the implementing type from the provided sierra `DebugInfo`.
    /// # Errors
    ///
    /// This function will return an error if the conversion from `DebugInfo` to the implementing type fails.
    fn try_from_debug_info(sierra_debug_info: &DebugInfo) -> Result<Self, Self::Error>;
}

/// Enum representing the possible errors that can occur when trying to create an annotation from sierra debug information.
#[derive(Debug)]
pub enum AnnotationsError {
    /// Error indicating that the namespace is missing from the annotations. Contains the missing namespace.
    MissingNamespace(String),
    /// Error indicating that the deserialization of the annotation failed. Contains the deserialization error.
    DeserializationError(serde_json::Error),
}

pub(crate) trait Namespace {
    const NAMESPACE: &'static str;
}

impl<T> TryFromDebugInfo for T
where
    T: Namespace + DeserializeOwned,
{
    type Error = AnnotationsError;

    fn try_from_debug_info(sierra_debug_info: &DebugInfo) -> Result<Self, Self::Error> {
        let value = sierra_debug_info
            .annotations
            .get(Self::NAMESPACE)
            .ok_or_else(|| AnnotationsError::MissingNamespace(Self::NAMESPACE.into()))?;

        serde_json::from_value(value.clone()).map_err(AnnotationsError::DeserializationError)
    }
}
