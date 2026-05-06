use thiserror::Error;

pub type Result<T> = core::result::Result<T, AppError>;
/// Macro to quickly create EspError from an ESP_ERR_ constant.

#[derive(Error, Debug)]
pub enum AppError {}
