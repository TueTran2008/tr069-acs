use crate::model::base::{self, DbBmc};
use crate::model::{Error, Result};
use crate::{ctx::Ctx, model::ModelManager};
use chrono::{DateTime, Utc};
use modql::field::{Fields, HasSeaFields};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, prelude::FromRow};

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Device {
    pub id: i64,
    pub manufacture: Option<String>,
    pub oui: Option<String>,
    pub product_class: Option<String>,
    pub serial_number: String,
    //pub last_inform: Option<DateTime<Utc>>,
}

#[derive(Fields, Clone)]
pub struct DeviceForCreate {
    pub device_identify: String,
}

impl DeviceForCreate {
    pub fn new(
        serial_number: String,
        oui: Option<String>,
        product_class: Option<String>,
        manufacture: Option<String>,
    ) -> Self {
        let temp = format!(
            "{}-{}-{}-{}",
            serial_number,
            oui.unwrap_or("".to_string()),
            product_class.unwrap_or("".to_string()),
            manufacture.unwrap_or("".to_string())
        );
        Self {
            device_identify: temp,
        }
    }
}
/// Marker trait
pub trait DeviceBy: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

// region:    --- UserBmc

pub struct DeviceBmc;

impl DbBmc for DeviceBmc {
    const TABLE: &'static str = "devices";
}

impl DeviceBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, device_c: DeviceForCreate) -> Result<i64> {
        //Create device row
        //
        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;
        let device_id = base::create::<Self, _>(ctx, &mm, device_c.clone())
            .await
            .map_err(|model_err| {
                Error::resolve_unique_violation(
                    model_err,
                    Some(|table: &str, constraint: &str| {
                        if table == "devices" && constraint.contains("device_id") {
                            Some(Error::DeviceAlreadyExist {
                                device_identify: device_c.device_identify.clone(),
                            })
                        } else {
                            None // Error::UniqueViolation will be created by resolve_unique_violation
                        }
                    }),
                )
            })?;
        // Commit the transaction
        mm.dbx().commit_txn().await?;
        Ok(device_id)
    }
}
