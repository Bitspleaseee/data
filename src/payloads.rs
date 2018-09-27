//! Contains useful template payloads

use std::ops::{Deref, DerefMut};

/// A payload which must be present, but empty
///
/// # Examples
///
/// **For the examples it is assumed that the payload is used in a surrounding
/// type**
///
/// The following JSON object is parsed as a valid request because there is an
/// empty payload.
///
/// ```json
/// {
///     "type": "SOME_TYPE",
///     "payload": {}
/// }
/// ```
///
/// However this JSON object is parsed as a invalid request because `payload`
/// is missing.
///
/// ```json
/// {
///     "type": "SOME_TYPE"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "payload")]
pub struct EmptyPayloadStrict {}

/// A payload which can either be present and empty or not present
///
/// # Examples
///
/// **For the examples it is assumed that the payload is used in a surrounding
/// type**
///
/// The following JSON object is parsed as a valid request because there is an
/// empty payload.
///
/// ```json
/// {
///     "type": "SOME_TYPE",
///     "payload": {}
/// }
/// ```
///
/// This JSON object is also parsed as a valid request even though the field is
/// missing.
///
/// ```json
/// {
///     "type": "SOME_TYPE"
/// }
/// ```
pub type EmptyPayload = Option<EmptyPayloadStrict>;

/// Represents a payload that also contains a authorization token
///
/// This payload is generic for both the inner type and the token type, this
/// means that the consumer can provide the types they want. The sole purpose
/// of this structure is to provide a simple way of wrapping an existing type
/// with a token.
///
/// This wrapper is mainly intended for internal use as the token will be
/// stored in the session cookies.
///
/// NB! The type that is wrapped cannot contain a field named `token`
/// (`#[serde(rename="...")]` could be used to circument this)
///
/// # JSON structure
///
/// ```json
/// "payload": {
///     "token": "<any-type-of-token>",
///     "field1": "field1 on the inner type",
///     "field2": "field2 on the inner type",
///     /* ... */
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "payload")]
pub struct AuthPayload<Inner, Token> {
    token: Token,
    #[serde(flatten)]
    inner: Inner,
}

impl<Inner, Token> AuthPayload<Inner, Token> {
    pub fn new(i: impl Into<Inner>, t: impl Into<Token>) -> AuthPayload<Inner, Token> {
        AuthPayload {
            inner: i.into(),
            token: t.into(),
        }
    }

    /// Get a reference to the token of the payload
    pub fn token(&self) -> &Token {
        &self.token
    }

    /// Set the token of a payload
    pub fn set_token(&mut self, t: impl Into<Token>) -> Token {
        std::mem::replace(&mut self.token, t.into())
    }

    /// Turn the payload into its inner type
    pub fn into_inner(self) -> (Inner, Token) {
        (self.inner, self.token)
    }
}

impl<Inner, Token> Deref for AuthPayload<Inner, Token> {
    type Target = Inner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Inner, Token> DerefMut for AuthPayload<Inner, Token> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}