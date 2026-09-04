use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum Tr069FaultCode {
    MethodNotSupported = 9000,
    RequestDenied = 9001,
    InternalError = 9002,
    InvalidArguments = 9003,
    ResourcesExceeded = 9004,
    InvalidParameterName = 9005,
    InvalidParameterType = 9006,
    InvalidParameterValue = 9007,
    NonWritableParameter = 9008,
    NotificationRequestRejected = 9009,
    DownloadFailure = 9010,
    UploadFailure = 9011,
    FileTransferAuthFailure = 9012,
    UnsupportedProtocol = 9013,

    /// Any code not defined by TR-069
    Unknown(u16),
}

impl IntoResponse for Tr069FaultCode {
    fn into_response(self) -> Response {
        let body = match self {
            Tr069FaultCode::InternalError => "Internal Error hehehe",
            _ => "Error that hasn't been define",
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "Fault")]
pub struct Fault {
    #[serde(rename = "faultcode")]
    fault_code: Option<String>,
    #[serde(rename = "faultstring")]
    fault_string: Option<String>,

    #[serde(rename = "detail")]
    fault_detail: Option<FaultStruct>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "Fault")]
pub struct FaultStruct {
    #[serde(rename = "FaultCode")]
    fault_code: Option<Tr069FaultCode>,
    #[serde(rename = "FaultString")]
    fault_string: Option<String>,
}
