use async_graphql::Object;
use chrono::{DateTime, Utc};

use crate::config::model::User;

#[Object]
impl User {
    async fn uid(&self) -> String {
        self.uid.clone()
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    async fn name(&self) -> Option<String> {
        self.name.clone()
    }

    async fn image(&self) -> Option<String> {
        self.image.clone()
    }
}
