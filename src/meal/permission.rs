use crate::auth::model::RequestUser;
use crate::error::{Error, Result};

use super::model::{Meal, MealQuery};

impl Meal {
    pub async fn can_view(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_create(user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_update(&self, request_user: &RequestUser) -> Result<()> {
        if self.user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }

    pub async fn can_delete(&self, request_user: &RequestUser) -> Result<()> {
        if self.user_id == request_user.id || request_user.is_superuser {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}

impl MealQuery {
    pub async fn can_view(&self, user: &RequestUser) -> Result<()> {
        if user.is_active {
            Ok(())
        } else {
            Err(Error::Forbidden)
        }
    }
}
