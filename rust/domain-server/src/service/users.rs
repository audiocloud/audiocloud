use anyhow::{anyhow, bail};
use argon2::{Algorithm, Argon2, Params, Version};
use lazy_static::lazy_static;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

use api::user::{CreateUserRequest, CreateUserResponse, DeleteUserResponse, UpdateUserRequest, UpdateUserResponse, UserSpec, UserSummary};
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
           .map(|user| UserSummary { id: user.id })
           .collect())
  }

  pub async fn create_user(&self, id: String, create: CreateUserRequest) -> Result<CreateUserResponse> {
    let salt = create_salt();
    let hash = PASSWORD_HASHER.hash_password(create.password.as_bytes(), &salt)
                              .map_err(|err| anyhow!("Failed to hash password: {err}"))?;
    let user = UserSpec { id:       id.clone(),
                          password: hash.to_string(), };
    self.nats.user_spec.put(BucketKey::new(&user.id), user).await?;

    Ok(CreateUserResponse { id })
  }

  pub async fn update_user(&self, id: String, update: UpdateUserRequest) -> Result<UpdateUserResponse> {
    let mut updated = false;
    if let Some(new_password) = update.set_password {
      let salt = create_salt();
      let hash = PASSWORD_HASHER.hash_password(new_password.as_bytes(), &salt)
                                .map_err(|err| anyhow!("Failed to hash password: {err}"))?;
      let user = UserSpec { id:       id.clone(),
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

  pub async fn check_user_login(&self, id: String, password: String) -> Result {
    let Some(user) = self.nats.user_spec.get(BucketKey::new(&id)).await? else { bail!("User {id} not found") };
    let Ok(hash) = PasswordHash::new(&user.password) else { bail!("User {id} has invalid password hash") };
    let Ok(_) = hash.verify_password(&[&*PASSWORD_HASHER as &dyn PasswordVerifier], password.as_bytes()) else { bail!("User {id} has invalid password") };

    Ok(())
  }
}

fn create_salt() -> SaltString {
  SaltString::generate(rand::thread_rng())
}
