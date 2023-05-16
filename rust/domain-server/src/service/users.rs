use anyhow::{anyhow, bail};
use argon2::{Algorithm, Argon2, Params, Version};
use lazy_static::lazy_static;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

use api::user::{
  DeleteUserResponse, RegisterUserRequest, RegisterUserResponse, UpdateUserRequest, UpdateUserResponse, UserSpec, UserSummary,
};
use api::BucketKey;

use super::{Result, Service};

lazy_static! {
  static ref PASSWORD_HASHER: Argon2<'static> = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default());
}

impl Service {
  pub async fn list_users(&self) -> Result<Vec<UserSummary>> {
    Ok(self.nats
           .user_spec
           .scan("*")
           .await?
           .into_values()
           .map(|user| UserSummary { id:    user.id,
                                     email: user.email, })
           .collect())
  }

  pub async fn get_user(&self, id: &str) -> Result<Option<UserSpec>> {
    Ok(self.nats.user_spec.get(BucketKey::new(id)).await?)
  }

  pub async fn register_user(&self, id: String, create: RegisterUserRequest) -> Result<RegisterUserResponse> {
    let salt = create_salt();
    let hash = PASSWORD_HASHER.hash_password(create.password.as_bytes(), &salt)
                              .map_err(|err| anyhow!("Failed to hash password: {err}"))?;
    let user = UserSpec { id:       id.clone(),
                          email:    create.email,
                          password: hash.to_string(), };
    self.nats.user_spec.put(BucketKey::new(&user.id), user).await?;

    Ok(RegisterUserResponse { id })
  }

  pub async fn update_user(&self, id: String, update: UpdateUserRequest) -> Result<UpdateUserResponse> {
    let mut updated = false;
    let existing = self.get_user(&id).await?.ok_or_else(|| anyhow!("User {id} not found"))?;

    if let Some(new_password) = update.set_password {
      let salt = create_salt();
      let hash = PASSWORD_HASHER.hash_password(new_password.as_bytes(), &salt)
                                .map_err(|err| anyhow!("Failed to hash password: {err}"))?;
      let user = UserSpec { id:       id.clone(),
                            email:    existing.email.clone(),
                            password: hash.to_string(), };

      self.nats.user_spec.put(BucketKey::new(&user.id), user).await?;
      updated = true;
    }

    Ok(UpdateUserResponse { id, updated })
  }

  pub async fn delete_user(&self, id: String) -> Result<DeleteUserResponse> {
    self.nats.user_spec.delete(BucketKey::new(&id)).await?;

    Ok(DeleteUserResponse { id, deleted: true })
  }

  pub async fn check_user_login(&self, id: &str, password: &str) -> Result<UserSpec> {
    let Some(user) = self.nats.user_spec.get(BucketKey::new(id)).await? else { bail!("User {id} not found") };
    let Ok(hash) = PasswordHash::new(&user.password) else { bail!("User {id} has invalid password hash") };
    let Ok(_) = hash.verify_password(&[&*PASSWORD_HASHER as &dyn PasswordVerifier], password.as_bytes()) else { bail!("Invalid password for user {id}") };

    Ok(user)
  }
}

fn create_salt() -> SaltString {
  SaltString::generate(rand::thread_rng())
}
