//! Update a user.
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{Auth0, Auth0RequestBuilder};
use crate::users::User;

/// Update a user.
/// Some considerations:
///
/// * The properties of the new object will replace the old ones.
/// * The metadata fields are an exception to this rule (`user_metadata` and `app_metadata`). These
/// properties are merged instead of being replaced but be careful, the merge only occurs on the
/// first level.
/// * If you are updating `email`, `email_verified`, `phone_number`, `phone_verified`, `username` or
/// `password` of a secondary identity, you need to specify the connection property too.
/// * If you are updating `email` or `phone_number` you can specify, optionally, the `client_id`
/// property.
/// * Updating `email_verified` is not supported for enterprise and passwordless sms connections.
/// * Updating the `blocked` to `false` does not affect the user's blocked state from an excessive
/// amount of incorrectly provided credentials. Use the "Unblock a user" endpoint from the
/// "User Blocks" API to change the user's state.
///
/// # Scopes
/// * `update:users`
/// * `update:users_app_metadata`
#[derive(Serialize)]
pub struct UserUpdate<'a, A, U> {
  #[serde(skip_serializing)]
  client: &'a Auth0,

  #[serde(skip_serializing)]
  user_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  blocked: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  email: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  email_verified: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  phone_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  phone_verified: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  given_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  family_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  nickname: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  picture: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  password: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  connection: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  client_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  verify_email: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  verify_phone_number: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  app_metadata: Option<A>,
  #[serde(skip_serializing_if = "Option::is_none")]
  user_metadata: Option<U>,
}

impl<'a, A, U> UserUpdate<'a, A, U> {
  /// Create update user request.
  pub fn new(client: &'a Auth0, id: &str) -> Self {
    Self {
      client,
      
      user_id: id.to_owned(),
      blocked: None,
      email: None,
      email_verified: None,
      phone_number: None,
      phone_verified: None,
      given_name: None,
      family_name: None,
      name: None,
      nickname: None,
      picture: None,
      verify_email: None,
      verify_phone_number: None,
      password: None,
      connection: None,
      client_id: None,
      app_metadata: None,
      user_metadata: None,
    }
  }

  /// ID of the user which can be used when interacting with other APIs.
  pub fn user_id(&mut self, id: &str) -> &mut Self {
    self.user_id = id.to_owned();
    self
  }

  /// Whether this user was blocked by an administrator (true) or not (false).
  pub fn blocked(&mut self, blocked: bool) -> &mut Self {
    self.blocked = Some(blocked);
    self
  }

  /// Email address of this user.
  pub fn email(&mut self, email: &str) -> &mut Self {
    self.email = Some(email.to_owned());
    self
  }

  /// Whether this email address is verified (true) or unverified (false). If set to false the user
  /// will not receive a verification email unless `verify_email` is set to true.
  pub fn email_verified(&mut self, email_verified: bool) -> &mut Self {
    self.email_verified = Some(email_verified);
    self
  }

  /// The user's phone number (following the E.164 recommendation), only valid for users from SMS
  /// connections.
  pub fn phone_number(&mut self, phone_number: &str) -> &mut Self {
    self.phone_number = Some(phone_number.to_owned());
    self
  }

  /// Whether this phone number has been verified (true) or not (false).
  pub fn phone_verified(&mut self, phone_verified: bool) -> &mut Self {
    self.phone_verified = Some(phone_verified);
    self
  }

  /// Given name/first name/forename of this user.
  pub fn given_name(&mut self, given_name: &str) -> &mut Self {
    self.given_name = Some(given_name.to_owned());
    self
  }

  /// Family name/last name/surname of this user.
  pub fn family_name(&mut self, family_name: &str) -> &mut Self {
    self.family_name = Some(family_name.to_owned());
    self
  }

  /// Name of this user.
  pub fn name(&mut self, name: &str) -> &mut Self {
    self.name = Some(name.to_owned());
    self
  }

  /// Preferred nickname or alias of this user.
  pub fn nickname(&mut self, nickname: &str) -> &mut Self {
    self.nickname = Some(nickname.to_owned());
    self
  }

  /// URL to picture, photo, or avatar of this user.
  pub fn picture(&mut self, picture: &str) -> &mut Self {
    self.picture = Some(picture.to_owned());
    self
  }

  /// Whether this user will receive a verification email after creation (true) or no email (false).
  /// Overrides behavior of `email_verified` parameter.
  pub fn verify_email(&mut self, verify_email: bool) -> &mut Self {
    self.verify_email = Some(verify_email);
    self
  }

  /// Whether this user will receive a text after changing the phone number (true) or no text
  /// (false). Only valid when changing phone number.
  pub fn verify_phone_number(&mut self, verify_phone_number: bool) -> &mut Self {
    self.verify_phone_number = Some(verify_phone_number);
    self
  }

  /// New password for this user (mandatory for non-SMS connections).
  pub fn password(&mut self, password: &str) -> &mut Self {
    self.password = Some(password.to_owned());
    self
  }

  /// ID of the connection this user should be created in.
  pub fn connection(&mut self, connection: &str) -> &mut Self {
    self.connection = Some(connection.to_owned());
    self
  }

  /// Auth0 client ID. Only valid when updating email address.
  pub fn client_id(&mut self, client_id: &str) -> &mut Self {
    self.client_id = Some(client_id.to_owned());
    self
  }

  /// User metadata to which this user has read-only access.
  pub fn app_metadata(&mut self, app_metadata: A) -> &mut Self {
    self.app_metadata = Some(app_metadata);
    self
  }

  /// User metadata to which this user has read/write access.
  pub fn user_metadata(&mut self, user_metadata: U) -> &mut Self {
    self.user_metadata = Some(user_metadata);
    self
  }
}

impl<'a, A, U> AsRef<Auth0> for UserUpdate<'a, A, U> {
  fn as_ref(&self) -> &Auth0 {
    self.client
  }
}

impl<
  'a,
  A: Serialize + DeserializeOwned,
  U: Serialize + DeserializeOwned,
> Auth0RequestBuilder for UserUpdate<'a, A, U>
{
  type Response = User<A, U>;

  fn build<F>(&self, factory: F) -> RequestBuilder
    where
      F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::DELETE, &format!("api/v2/users/{}", self.user_id)).json(self)
  }
}
