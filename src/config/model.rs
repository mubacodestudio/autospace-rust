use async_graphql::Object;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub uid: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: Option<String>,
    pub image: Option<String>,
}

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

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Admin {
    pub uid: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Credentials {
    pub uid: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AuthProvider {
    pub uid: String,
    pub auth_type: AuthProviderType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AuthProviderType {
    Google,
    Credentials,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Customer {
    pub uid: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub display_name: Option<String>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Manager {
    pub uid: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub display_name: Option<String>,
    pub company_id: Option<i32>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Valet {
    pub uid: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub display_name: String,
    pub image: Option<String>,
    pub licence_id: String,
    pub company_id: Option<i32>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Company {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Garage {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub images: Vec<String>,
    pub company_id: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Address {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub address: String,
    pub lat: f64,
    pub lng: f64,
    pub garage_id: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Slot {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub display_name: Option<String>,
    pub price_per_hour: f64,
    pub length: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub slot_type: SlotType,
    pub garage_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SlotType {
    Car,
    Heavy,
    Bike,
    Bicycle,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Booking {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub price_per_hour: Option<f64>,
    pub total_price: Option<f64>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub vehicle_number: String,
    pub phone_number: Option<String>,
    pub passcode: Option<String>,
    pub status: BookingStatus,
    pub slot_id: i32,
    pub customer_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BookingStatus {
    Booked,
    ValetAssignedForCheckIn,
    ValetPickedUp,
    CheckedIn,
    ValetAssignedForCheckOut,
    CheckedOut,
    ValetReturned,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ValetAssignment {
    pub booking_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pickup_lat: Option<f64>,
    pub pickup_lng: Option<f64>,
    pub return_lat: Option<f64>,
    pub return_lng: Option<f64>,
    pub pickup_valet_id: Option<String>,
    pub return_valet_id: Option<String>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct BookingTimeline {
    pub id: i32,
    pub timestamp: DateTime<Utc>,
    pub status: BookingStatus,
    pub booking_id: i32,
    pub valet_id: Option<String>,
    pub manager_id: Option<String>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Review {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub rating: i32,
    pub comment: Option<String>,
    pub customer_id: String,
    pub garage_id: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Verification {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub verified: bool,
    pub admin_id: String,
    pub garage_id: i32,
}
