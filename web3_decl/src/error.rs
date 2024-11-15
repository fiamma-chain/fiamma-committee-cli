use std::{
    collections::HashMap,
    error, fmt,
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll},
};

use jsonrpsee::core::ClientError;
use pin_project_lite::pin_project;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Web3Error {
    #[error("Internal error")]
    InternalError,
    #[error("Invalid register's public key {0}")]
    InvalidRegisterPublicKey(String),
    #[error("RegisterNodeError error {0}")]
    RegisterNodeError(String),
    #[error("ChallengeNodeError error {0}")]
    ChallengeNodeError(String),
    #[error("DisproveError error {0}")]
    DisproveError(String),
    #[error("RegisterCircuitError error {0}")]
    RegisterCircuitError(String),
}

/// Client RPC error with additional details: the method name and arguments of the called method.
///
/// The wrapped error can be accessed using [`AsRef`].
#[derive(Debug)]
pub struct EnrichedClientError {
    inner_error: ClientError,
    method: &'static str,
    args: HashMap<&'static str, String>,
}

/// Alias for a result with enriched client RPC error.
pub type EnrichedClientResult<T> = Result<T, EnrichedClientError>;

impl EnrichedClientError {
    /// Wraps the specified `inner_error`.
    pub fn new(inner_error: ClientError, method: &'static str) -> Self {
        Self {
            inner_error,
            method,
            args: HashMap::new(),
        }
    }

    /// Creates an error wrapping [`RpcError::Custom`].
    pub fn custom(message: impl Into<String>, method: &'static str) -> Self {
        Self::new(ClientError::Custom(message.into()), method)
    }

    /// Adds a tracked argument for this error.
    #[must_use]
    pub fn with_arg(mut self, name: &'static str, value: &dyn fmt::Debug) -> Self {
        self.args.insert(name, format!("{value:?}"));
        self
    }
}

impl AsRef<ClientError> for EnrichedClientError {
    fn as_ref(&self) -> &ClientError {
        &self.inner_error
    }
}

impl error::Error for EnrichedClientError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.inner_error)
    }
}

impl fmt::Display for EnrichedClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugArgs<'a>(&'a HashMap<&'static str, String>);

        impl fmt::Debug for DebugArgs<'_> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("(")?;
                for (i, (name, value)) in self.0.iter().enumerate() {
                    write!(formatter, "{name}={value}")?;
                    if i + 1 < self.0.len() {
                        formatter.write_str(", ")?;
                    }
                }
                formatter.write_str(")")
            }
        }

        write!(
            formatter,
            "JSON-RPC request {}{:?} failed: {}",
            self.method,
            DebugArgs(&self.args),
            self.inner_error
        )
    }
}

pin_project! {
    /// Contextual information about an RPC. Returned by [`ClientRpcContext::rpc_context()`]. The context is eventually converted
    /// to a result with [`EnrichedClientError`] error type.
    #[derive(Debug)]
    pub struct ClientCallWrapper<'a, F> {
        #[pin]
        inner: F,
        method: &'static str,
        args: HashMap<&'static str, &'a (dyn fmt::Debug + Send + Sync)>,
    }
}

impl<'a, T, F> ClientCallWrapper<'a, F>
where
    F: Future<Output = Result<T, ClientError>>,
{
    /// Adds a tracked argument for this context.
    #[must_use]
    pub fn with_arg(
        mut self,
        name: &'static str,
        value: &'a (dyn fmt::Debug + Send + Sync),
    ) -> Self {
        self.args.insert(name, value);
        self
    }
}

impl<T, F> Future for ClientCallWrapper<'_, F>
where
    F: Future<Output = Result<T, ClientError>>,
{
    type Output = Result<T, EnrichedClientError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let projection = self.project();
        match projection.inner.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(value)) => Poll::Ready(Ok(value)),
            Poll::Ready(Err(err)) => {
                let err = EnrichedClientError {
                    inner_error: err,
                    method: projection.method,
                    // `mem::take()` is safe to use: by contract, a `Future` shouldn't be polled after completion
                    args: mem::take(projection.args)
                        .into_iter()
                        .map(|(name, value)| (name, format!("{value:?}")))
                        .collect(),
                };
                Poll::Ready(Err(err))
            }
        }
    }
}

/// Extension trait allowing to add context to client RPC calls. Can be used on any future resolving to `Result<_, ClientError>`.
pub trait ClientRpcContext: Sized {
    /// Adds basic context information: the name of the invoked RPC method.
    fn rpc_context(self, method: &'static str) -> ClientCallWrapper<'static, Self>;
}

impl<T, F> ClientRpcContext for F
where
    F: Future<Output = Result<T, ClientError>>,
{
    fn rpc_context(self, method: &'static str) -> ClientCallWrapper<'static, Self> {
        ClientCallWrapper {
            inner: self,
            method,
            args: HashMap::new(),
        }
    }
}
