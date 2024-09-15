pub mod user;

use async_graphql::{Context, Object, Result};
use sqlx::PgPool;

use crate::config::model::User;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> Result<String> {
        Ok("test from autospace Rust".to_string())
    }

    async fn user_count(&self, ctx: &Context<'_>) -> Result<i64> {
        let pool = ctx.data::<PgPool>()?;
        let (count,) = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM User")
            .fetch_one(pool)
            .await?;
        Ok(count)
    }

    async fn get_all_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data::<PgPool>()?;
        let users = sqlx::query_as::<_, User>("SELECT * FROM public.User")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }
}
