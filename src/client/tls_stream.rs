use futures_util::io::{AsyncRead, AsyncWrite};
use crate::client::Config;

#[cfg(feature = "rustls")]
mod rustls_tls_stream;

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
mod native_tls_stream;

#[cfg(all(feature = "vendored-openssl", not(feature = "rustls"), not(feature = "native-tls")))]
mod opentls_tls_stream;

#[cfg(feature = "rustls")]
pub(crate) use rustls_tls_stream::TlsStream;

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
pub(crate) use native_tls_stream::TlsStream;

#[cfg(all(feature = "vendored-openssl", not(feature = "rustls"), not(feature = "native-tls")))]
pub(crate) use opentls_tls_stream::TlsStream;

#[cfg(feature = "rustls")]
pub(crate) async fn create_tls_stream<S: AsyncRead + AsyncWrite + Unpin + Send>(
    config: &Config,
    stream: S,
) -> crate::Result<TlsStream<S>> {
    TlsStream::new(config, stream).await
}

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
pub(crate) async fn create_tls_stream<S: AsyncRead + AsyncWrite + Unpin + Send>(
    config: &Config,
    stream: S,
) -> crate::Result<TlsStream<S>> {
    native_tls_stream::create_tls_stream(config, stream).await
}

#[cfg(all(feature = "vendored-openssl", not(feature = "rustls"), not(feature = "native-tls")))]
pub(crate) async fn create_tls_stream<S: AsyncRead + AsyncWrite + Unpin + Send>(
    config: &Config,
    stream: S,
) -> crate::Result<TlsStream<S>> {
    opentls_tls_stream::create_tls_stream(config, stream).await
}
