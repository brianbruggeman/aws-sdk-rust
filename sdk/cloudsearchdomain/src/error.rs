// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct SearchError {
    pub kind: SearchErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum SearchErrorKind {
    SearchException(crate::error::SearchException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            SearchErrorKind::SearchException(_inner) => _inner.fmt(f),
            SearchErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for SearchError {
    fn code(&self) -> Option<&str> {
        SearchError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl SearchError {
    pub fn new(kind: SearchErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: SearchErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: SearchErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display as implemented
    // by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_search_exception(&self) -> bool {
        matches!(&self.kind, SearchErrorKind::SearchException(_))
    }
}
impl std::error::Error for SearchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            SearchErrorKind::SearchException(_inner) => Some(_inner),
            SearchErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct SuggestError {
    pub kind: SuggestErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum SuggestErrorKind {
    SearchException(crate::error::SearchException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for SuggestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            SuggestErrorKind::SearchException(_inner) => _inner.fmt(f),
            SuggestErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for SuggestError {
    fn code(&self) -> Option<&str> {
        SuggestError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl SuggestError {
    pub fn new(kind: SuggestErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: SuggestErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: SuggestErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display as implemented
    // by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_search_exception(&self) -> bool {
        matches!(&self.kind, SuggestErrorKind::SearchException(_))
    }
}
impl std::error::Error for SuggestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            SuggestErrorKind::SearchException(_inner) => Some(_inner),
            SuggestErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct UploadDocumentsError {
    pub kind: UploadDocumentsErrorKind,
    pub(crate) meta: smithy_types::Error,
}
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UploadDocumentsErrorKind {
    DocumentServiceException(crate::error::DocumentServiceException),
    /// An unexpected error, eg. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for UploadDocumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            UploadDocumentsErrorKind::DocumentServiceException(_inner) => _inner.fmt(f),
            UploadDocumentsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl smithy_types::retry::ProvideErrorKind for UploadDocumentsError {
    fn code(&self) -> Option<&str> {
        UploadDocumentsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<smithy_types::retry::ErrorKind> {
        None
    }
}
impl UploadDocumentsError {
    pub fn new(kind: UploadDocumentsErrorKind, meta: smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: UploadDocumentsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    pub fn generic(err: smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: UploadDocumentsErrorKind::Unhandled(err.into()),
        }
    }

    // Consider if this should actually be `Option<Cow<&str>>`. This would enable us to use display as implemented
    // by std::Error to generate a message in that case.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    pub fn meta(&self) -> &smithy_types::Error {
        &self.meta
    }

    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    pub fn is_document_service_exception(&self) -> bool {
        matches!(
            &self.kind,
            UploadDocumentsErrorKind::DocumentServiceException(_)
        )
    }
}
impl std::error::Error for UploadDocumentsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            UploadDocumentsErrorKind::DocumentServiceException(_inner) => Some(_inner),
            UploadDocumentsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>Information about any problems encountered while processing an upload request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DocumentServiceException {
    /// <p>The return status of a document upload request, <code>error</code> or <code>success</code>.</p>
    pub status: std::option::Option<std::string::String>,
    /// <p>The description of the errors returned by the document service.</p>
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DocumentServiceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DocumentServiceException");
        formatter.field("status", &self.status);
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl DocumentServiceException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for DocumentServiceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DocumentServiceException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for DocumentServiceException {}
/// See [`DocumentServiceException`](crate::error::DocumentServiceException)
pub mod document_service_exception {
    /// A builder for [`DocumentServiceException`](crate::error::DocumentServiceException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<std::string::String>,
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The return status of a document upload request, <code>error</code> or <code>success</code>.</p>
        pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
            self.status = Some(input.into());
            self
        }
        pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status = input;
            self
        }
        /// <p>The description of the errors returned by the document service.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`DocumentServiceException`](crate::error::DocumentServiceException)
        pub fn build(self) -> crate::error::DocumentServiceException {
            crate::error::DocumentServiceException {
                status: self.status,
                message: self.message,
            }
        }
    }
}
impl DocumentServiceException {
    /// Creates a new builder-style object to manufacture [`DocumentServiceException`](crate::error::DocumentServiceException)
    pub fn builder() -> crate::error::document_service_exception::Builder {
        crate::error::document_service_exception::Builder::default()
    }
}

/// <p>Information about any problems encountered while processing a search request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct SearchException {
    /// <p>A description of the error returned by the search service.</p>
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for SearchException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SearchException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl SearchException {
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for SearchException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SearchException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for SearchException {}
/// See [`SearchException`](crate::error::SearchException)
pub mod search_exception {
    /// A builder for [`SearchException`](crate::error::SearchException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>A description of the error returned by the search service.</p>
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`SearchException`](crate::error::SearchException)
        pub fn build(self) -> crate::error::SearchException {
            crate::error::SearchException {
                message: self.message,
            }
        }
    }
}
impl SearchException {
    /// Creates a new builder-style object to manufacture [`SearchException`](crate::error::SearchException)
    pub fn builder() -> crate::error::search_exception::Builder {
        crate::error::search_exception::Builder::default()
    }
}