//! Errors

#[cfg(feature = "client")]
use crate::{AccessToken, RefreshToken};

/// General errors for talking with twitch, used in [`AppAccessToken::get_app_access_token`](crate::tokens::AppAccessToken::get_app_access_token)
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[cfg(feature = "client")]
#[non_exhaustive]
pub enum AppAccessTokenError<RE: std::error::Error + Send + Sync + 'static> {
    /// request for token failed
    Request(#[source] RE),
    /// could not parse response when getting app access token
    RequestParseError(#[from] crate::RequestParseError),
}

/// Errors for [AccessToken::validate_token][crate::AccessTokenRef::validate_token] and [UserToken::from_response][crate::tokens::UserToken::from_response]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum ValidationError<RE: std::error::Error + Send + Sync + 'static> {
    /// token is not authorized for use
    NotAuthorized,
    /// could not parse response when validating token
    RequestParseError(#[from] crate::RequestParseError),
    /// failed to request validation
    Request(#[source] RE),
    /// given token is not of the correct token type: {0}
    InvalidToken(&'static str),
}

impl ValidationError<std::convert::Infallible> {
    /// Convert this error from a infallible to another
    pub fn into_other<RE: std::error::Error + Send + Sync + 'static>(self) -> ValidationError<RE> {
        match self {
            ValidationError::NotAuthorized => ValidationError::NotAuthorized,
            ValidationError::RequestParseError(e) => ValidationError::RequestParseError(e),
            ValidationError::InvalidToken(s) => ValidationError::InvalidToken(s),
            ValidationError::Request(_) => unreachable!(),
        }
    }
}

/// Error kinds for [UserToken::from_refresh_token][crate::UserToken::from_refresh_token] and [UserToken::UserToken::from_existing_or_refresh_token][crate::UserToken::from_existing_or_refresh_token]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
#[cfg(feature = "client")]
pub enum RetrieveTokenError<RE: std::error::Error + Send + Sync + 'static> {
    /// could not validate token
    ValidationError {
        /// Error validating the token
        #[source]
        error: ValidationError<RE>,
        /// Access token passed to the function
        access_token: AccessToken,
        /// Refresh token passed to the function
        refresh_token: Option<RefreshToken>,
    },
    /// could not refresh token
    RefreshTokenError {
        /// Error refreshing the token
        #[source]
        error: RefreshTokenError<RE>,
        /// Refresh token passed to the function
        refresh_token: RefreshToken,
    },
}

/// Errors for [AccessToken::revoke_token][crate::AccessTokenRef::revoke_token]
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
#[cfg(feature = "client")]
pub enum RevokeTokenError<RE: std::error::Error + Send + Sync + 'static> {
    /// could not parse response when revoking token
    RequestParseError(#[from] crate::RequestParseError),
    /// failed to do revokation
    RequestError(#[source] RE),
}

/// Errors for [TwitchToken::refresh_token][crate::TwitchToken::refresh_token]
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
#[cfg(feature = "client")]
pub enum RefreshTokenError<RE: std::error::Error + Send + Sync + 'static> {
    /// request when refreshing token failed
    RequestError(#[source] RE),
    /// could not parse response when refreshing token.
    RequestParseError(#[from] crate::RequestParseError),
    /// no client secret found
    // TODO: Include this in doc
    // A client secret is needed to request a refreshed token.
    NoClientSecretFound,
    /// no refresh token found
    NoRefreshToken,
    /// no expiration found on new token
    NoExpiration,
}

/// Errors for [`UserTokenBuilder::get_user_token`](crate::tokens::UserTokenBuilder::get_user_token) and [`UserToken::mock_token`](crate::tokens::UserToken::mock_token)
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
#[cfg(feature = "client")]
pub enum UserTokenExchangeError<RE: std::error::Error + Send + Sync + 'static> {
    /// request for user token failed
    RequestError(#[source] RE),
    /// could not parse response when getting user token
    RequestParseError(#[from] crate::RequestParseError),
    /// state CSRF does not match when exchanging user token
    StateMismatch,
    /// could not get validation for user token
    ValidationError(#[from] ValidationError<RE>),
}

/// Errors for [ImplicitUserTokenBuilder::get_user_token][crate::tokens::ImplicitUserTokenBuilder::get_user_token]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
#[cfg(feature = "client")]
pub enum ImplicitUserTokenExchangeError<RE: std::error::Error + Send + Sync + 'static> {
    // FIXME: should be TwitchTokenErrorResponse
    /// twitch returned an error: {error:?} - {description:?}
    TwitchError {
        /// Error type
        error: Option<String>,
        /// Description of error
        description: Option<String>,
    },
    /// state CSRF does not match
    StateMismatch,
    /// could not get validation for token
    ValidationError(#[from] ValidationError<RE>),
}
/// Errors for [`DeviceUserTokenBuilder`][crate::tokens::DeviceUserTokenBuilder]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
#[non_exhaustive]
#[cfg(feature = "client")]
pub enum DeviceUserTokenExchangeError<RE: std::error::Error + Send + Sync + 'static> {
    /// request for exchange token failed
    DeviceExchangeRequestError(#[source] RE),
    /// could not parse response when getting exchange token
    DeviceExchangeParseError(#[source] crate::RequestParseError),
    /// request for user token failed
    TokenRequestError(#[source] RE),
    /// could not parse response when getting user token
    TokenParseError(#[source] crate::RequestParseError),
    /// could not get validation for token
    ValidationError(#[from] ValidationError<RE>),
    /// no device code found, exchange not started
    NoDeviceCode,
    /// the device code has expired
    Expired,
}

#[cfg(feature = "client")]
impl<RE: std::error::Error + Send + Sync + 'static> DeviceUserTokenExchangeError<RE> {
    /// Check if the error is due to the authorization request being pending
    pub fn is_pending(&self) -> bool {
        matches!(self, DeviceUserTokenExchangeError::TokenParseError(
                crate::RequestParseError::TwitchError(crate::id::TwitchTokenErrorResponse {
                    message,
                    ..
                }),
            ) if message == "authorization_pending")
    }
}
