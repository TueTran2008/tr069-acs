pub mod get_names;
pub mod get_values;
pub mod session;

//use crate::telemetry::{get_subscriber, init_subscriber};
use axum::extract::{FromRequest, Request};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{async_trait, http};
use kameo::Reply;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
//use quick_xml::name::{Namespace, NamespaceResolver, ResolveResult};
use quick_xml::{de::*, NsReader, Writer};
use serde::{Deserialize, Serialize};
//use std::convert::Infallible;
use std::io::{Cursor, Write};
use std::panic;

use crate::cwmp_msg::get_names::GetParamterNames;

//pub const ENC_NP: &str = "soap-enc";
//pub const ENV_NP: &str = "soap-env";
//pub const CWMP_NP: &str = "cwmp";
//
//pub const SOAP_ENV_NP: &str = r#"http://schemas.xmlsoap.org/soap/envelope/"#;
pub const SOAP_ENC_NP: &str = r#"http://schemas.xmlsoap.org/soap/encoding/"#;
pub const SOAP_CWMP_NP: &str = r#"urn:dslforum-org:cwmp-1-0"#;
pub const SOAP_XSD_NP: &str = r#"http://www.w3.org/2001/XMLSchema"#;
pub const SOAP_XSI_NP: &str = r#"http://www.w3.org/2001/XMLSchema-instance"#;

/// Define methods use to build soap message
pub(crate) trait RpcWrite<'a, W> {
    fn build_message(&'a self, xml_writer: &'a mut Writer<W>) -> &'a mut Writer<W>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
//#[derive(Debug)]
//pub enum CpeRPC {
//    Inform,
//    GetRPCMethodsResponse,
//    SetParameterValuesResponse,
//    GetParameterValuesResponse,
//    GetParameterNamesResponse,
//    SetParameterAttributesResponse,
//    GetParameterAttributesResponse,
//    AddObjectResponse,
//    DeleteObjectResponse,
//    RebootResponse,
//    DownloadResponse,
//    ScheduleDownloadResponse,
//    UploadResponse,
//    FactoryResetResponse,
//    TransferComplete,
//    AutonomousTransferComplete,
//    RequestDownload,
//    DUStateChangeComplete,
//    GetQueuedTransfersResponse,
//    SetVouchersResponse,
//    GetOptionsResponse,
//    ScheduleInformResponse,
//    GetAllQueuedEventsResponse,
//}

//#[derive(Debug, Deserialize)]
//enum EventCode {
//    Event0BootStrap,
//    Event1Boot,
//    Event2Periodic,
//    Event3Schedule,
//    Event4ValueChange,
//    Event5Kicked,
//    Event6ConnectionRequest,
//    Event7TransferComplete,
//    Event8DiagnosticComplete,
//    Event9RequestDownload,
//    Event10AutonomousTransferComplete,
//    Event11DUStateChangeComplete,
//    Event12AutonomousDUStateChangeComplete,
//    Event13Wakeup,
//    Event14Heartbeat,
//    EventMReboot,
//    EventMScheduleInform,
//    EventMDownload,
//    EventMScheduleDownload,
//    EventMUpload,
//    EventMChangeDUState,
//    EventMVendorMethod,
//    EventMVendorEvent,
//}

#[derive(Debug, Deserialize, Serialize)]
struct CommandKey {
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EventStruct {
    // #[serde(rename = "@arrayType")]
    // nb_of_event: Option<String>,
    #[serde(rename = "EventCode")]
    event_code: Option<String>,
    #[serde(rename = "CommandKey")]
    command_key: Option<String>,
}

#[derive(Debug, Deserialize, Default, Serialize)]
struct EventList {
    #[serde(rename = "@arrayType")]
    nb_of_event: Option<String>,

    #[serde(rename = "EventStruct")]
    event_struct: Vec<EventStruct>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct DeviceIDStruct {
    #[serde(rename = "Manufacturer")]
    manufacturer: Option<String>,

    #[serde(rename = "OUI")]
    oui: Option<String>,

    #[serde(rename = "ProductClass")]
    product_class: Option<String>,

    #[serde(rename = "SerialNumber")]
    serial_number: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnySimpleType {
    //The value of an element defined to be of type “anySimpleType” MAY be of any simple data type,
    // including (but not limited to) any of the other types listed in this table.
    // Following the SOAP specification [12], elements specified as being of type “anySimpleType” MUST
    // include a type attribute to indicate the actual type of the element. For example:
    // <ParameterValueStruct>
    //  <Name>Device.DeviceInfo.ProvisioningCode</Name>
    //  <Value xsi:type="xsd:string">code12345</Value>
    // </ParameterValueStruct>
    // The namespaces xsi and xsd used above are as defined in [12].
    #[serde(rename = "@type")]
    xsi_type: Option<String>,

    #[serde(rename = "$text")]
    value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ParameterValueStruct {
    #[serde(rename = "Name")]
    name: Option<String>,
    //This is the value the Parameter is to be set. The CPE
    //MUST treat string-valued Parameter values as casesensitive.
    #[serde(rename = "Value")]
    value: Option<AnySimpleType>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ParameterList {
    #[serde(rename = "ParameterValueStruct")]
    parameter_struct: Vec<ParameterValueStruct>,

    #[serde(rename = "@arrayType")]
    nb_of_parameter: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Inform {
    #[serde(rename = "DeviceId")]
    device_id: DeviceIDStruct,

    #[serde(rename = "Event")]
    event: EventList,

    #[serde(rename = "MaxEnvelopes")]
    max_envelopes: u32,

    #[serde(rename = "CurrentTime")]
    current_time: String,

    #[serde(rename = "RetryCount")]
    retry_count: u32,

    #[serde(rename = "ParameterList")]
    parameter_list: Vec<ParameterList>,
}

impl Inform {
    // Get serial_number of the inform message
    pub fn get_sn(&self) -> Option<&String> {
        if let Some(ref sn) = self.device_id.serial_number {
            Some(sn)
        } else {
            None
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
struct ID {
    #[serde(rename = "@mustUnderstand")]
    must_understand: Option<String>,

    #[serde(rename = "$text")]
    value: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Header {
    #[serde(rename = "ID")]
    id: ID,
}

impl Header {
    pub fn new(msg_id: &str) -> Self {
        let default_id = ID {
            must_understand: Some(String::from("1")),
            value: Some(String::from(msg_id)),
        };
        Header { id: default_id }
    }
}

impl Default for Header {
    fn default() -> Self {
        let default_id = ID {
            must_understand: Some(String::from("1")),
            value: Some(String::from("magic_number")),
        };
        Header { id: default_id }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformResponse {
    #[serde(rename = "MaxEnvelopes")]
    pub max_envelopes: u32,
}

impl Default for InformResponse {
    fn default() -> Self {
        Self { max_envelopes: 1 }
    }
}

impl<'a, W: std::io::Write> RpcWrite<'a, W> for InformResponse {
    fn build_message(&'a self, xml_writer: &'a mut Writer<W>) -> &'a mut Writer<W> {
        // let mut xml_writer = Writer::new(Cursor::new(Vec::new()));

        // --- <cwmp:InformResponse>
        let inform_res = BytesStart::new("cwmp:InformResponse");
        xml_writer.write_event(Event::Start(inform_res)).unwrap();

        // --- MaxEnvelopes
        let max_envelopes = BytesStart::new("MaxEnvelopes");
        xml_writer.write_event(Event::Start(max_envelopes)).unwrap();
        xml_writer
            .write_event(Event::Text(BytesText::new("1")))
            .unwrap();

        // -- Close MaxEnvelopes

        xml_writer
            .write_event(Event::End(BytesEnd::new("MaxEnvelopes")))
            .unwrap();
        xml_writer
            .write_event(Event::End(BytesEnd::new("cwmp:InformResponse")))
            .unwrap();

        // Output - Generate Output
        // xml_writer.into_inner().into_inner()
        xml_writer
    }
}

impl Envelope {
    pub fn get_msg_id(&self) -> Option<&String> {
        //if let Some(ref msg_id) = self.header.id.value {
        //    msg_id
        //} else {
        //    tracing::warn!("This message doesn't have cwmp ID");
        //    ""
        //}
        //if let Some(ref msg_id) = self.header.and_then(Vgc)
        let msg_id = self
            .header
            .as_ref()
            .and_then(|msg_id| msg_id.id.value.as_ref());
        msg_id
    }

    // Should call this after initializeing header
    pub fn set_msg_id(&mut self, msg_id: &String) {
        //self.header.id.value = Some(String::from(msg_id));
        //self.header
        //    .and_then(|header| header.id.value = Some(String::from(msg_id)));
        if let Some(ref mut header) = self.header {
            header.id.value = Some(String::from(msg_id));
        } else {
            tracing::error!("Header is empty => Cannot set the new Message ID for Header");
        }
    }

    pub fn create_xml(&self) -> Option<Vec<u8>> {
        let mut xml_writer = Writer::new(Cursor::new(Vec::new()));

        xml_writer
            .write_event(Event::Decl(BytesDecl::new(
                r#"1.0"#,
                Some(r#"UTF-8"#),
                None,
            )))
            .expect("Failed to write XML declaration");
        xml_writer
            .create_element("soap-env:Envelope")
            .with_attributes(vec![
                (
                    "xmlns:soap-enc",
                    "http://schemas.xmlsoap.org/soap/encoding/",
                ),
                (
                    "xmlns:soap-env",
                    "http://schemas.xmlsoap.org/soap/envelope/",
                ),
                ("xmlns:xsd", "http://www.w3.org/2001/XMLSchema"),
                ("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"),
                ("xmlns:cwmp", "urn:dslforum-org:cwmp-1-0"),
            ])
            .write_inner_content(|xml| {
                if let Some(ref text) = self.header.as_ref().unwrap().id.value {
                    let header_random_str = text.as_str();
                    //let must_understand = "soap-env:mustUnderstand=\"1\"";
                    let _ = xml
                        .create_element("soap-env:Header")
                        .write_inner_content(|xml| {
                            xml.create_element("cwmp:ID")
                                .with_attribute(("soap-env:mustUnderstand", "1"))
                                .write_text_content(BytesText::new(header_random_str))
                                .unwrap();
                            Ok(())
                        });

                    match &self.body.as_ref().unwrap().msg_type {
                        CWMPMsg::InformResponse(msg) => {
                            msg.build_message(xml);
                        }
                        _ => panic!("Has implement message build for this type"),
                    }
                    Ok(())
                } else {
                    // None
                    // Err(std)
                    panic!("Failed to construct inner content of SOAP Message");
                }
            })
            // .write_text_content(BytesText::new(""))
            // .write_empty()
            .unwrap();

        Some(xml_writer.into_inner().into_inner())
    }
}

impl IntoResponse for Envelope {
    fn into_response(self) -> Response {
        (
            [(
                http::header::CONTENT_TYPE,
                HeaderValue::from_static(r#"text/xml; charset=\"utf-8\""#),
            )],
            String::from_utf8(self.create_xml().unwrap()).unwrap(),
        )
            .into_response()
    }
}

//fn xml_content_type<B> {
//
//}
#[derive(Deserialize, Debug, Serialize)]
pub enum CWMPMsg {
    Inform(Inform),
    InformResponse(InformResponse),
    GetRPCMethodsResponse,
    SetParameterValuesResponse,
    GetParameterValuesResponse,
    GetParameterNames(GetParamterNames),
    GetParameterNamesResponse,
    SetParameterAttributesResponse,
    GetParameterAttributesResponse,
    AddObjectResponse,
    DeleteObjectResponse,
    RebootResponse,
    DownloadResponse,
    ScheduleDownloadResponse,
    UploadResponse,
    FactoryResetResponse,
    TransferComplete,
    AutonomousTransferComplete,
    RequestDownload,
    DUStateChangeComplete,
    GetQueuedTransfersResponse,
    SetVouchersResponse,
    GetOptionsResponse,
    ScheduleInformResponse,
    GetAllQueuedEventsResponse,
    EmptyRPC,
}

#[derive(Deserialize, Debug, Serialize)]
struct Body {
    #[serde(rename = "$value")]
    msg_type: CWMPMsg,
}

#[derive(Deserialize, Debug, Serialize, Reply)]
#[serde(rename = "Envelope")]
pub struct Envelope {
    #[serde(rename = "@xmlns:cwmp")]
    cwmp: Option<String>,

    #[serde(rename = "@xmlns:soap-enc")]
    soap_enc: Option<String>,

    #[serde(rename = "@xmlns:xsi")]
    xsi: Option<String>,

    #[serde[rename = "@xmlns:xsd"]]
    xsd: Option<String>,

    #[serde[rename = "@xmlns:soap-env"]]
    soap_env: Option<String>,

    #[serde(rename = "Header")]
    header: Option<Header>,

    #[serde(rename = "Body")]
    body: Option<Body>,
    // Empty body of the HTTP request
    //#[serde(skip)]
    //is_empty: bool,
}

impl Envelope {
    pub fn new(msg_id: &str, msg_body: CWMPMsg) -> Self {
        Self {
            cwmp: Some(String::from(SOAP_CWMP_NP)),
            soap_enc: Some(String::from(SOAP_ENC_NP)),
            xsi: Some(String::from(SOAP_ENC_NP)),
            xsd: Some(String::from(SOAP_XSD_NP)),
            soap_env: Some(String::from(SOAP_XSI_NP)),
            header: Some(Header::new(msg_id)),
            body: Some(Body { msg_type: msg_body }), // attrs: HashMap::new(),
        }
    }

    pub fn get_msg_body(&self) -> Option<&CWMPMsg> {
        self.body.as_ref().map(|body| &body.msg_type)
    }
    pub fn new_empty() -> Self {
        Self {
            cwmp: None,
            soap_enc: None,
            xsi: None,
            xsd: None,
            soap_env: None,
            header: None,
            body: None,
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.header.is_some() {
            return false;
        }

        if self.body.is_some() {
            return false;
        }

        return true;
    }
}

#[async_trait]
impl<S> FromRequest<S> for Envelope
where
    S: Send + Sync,
{
    type Rejection = Tr069FaultCode;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Todo: Check if body is empty ->
        let body = String::from_request(req, state).await.unwrap();
        if body.is_empty() {
            tracing::warn!("Body is empty, maybe CWMP Client is empty");
            Ok(Envelope::new_empty())
        } else {
            let envelope: Result<Envelope, DeError> = quick_xml::de::from_str(&body);
            match envelope {
                Ok(soap_msg) => Ok(soap_msg),
                Err(_) => {
                    tracing::error!("Cannot deserialize the in-coming message to xml");
                    Err(Tr069FaultCode::InternalError)
                }
            }
        }
    }
}

//impl Envelope {
//    pub fn get_rpc_type(&self) {}
//}

// fn parse_xml(xml: &str) {
//     use quick_xml::reader::NsReader;
//     let mut reader = NsReader::from_reader(xml.as_bytes());
//     // let mut buf = Vec::new();
//     let mut txt = Vec::new();
//
//     // let mut envelope = Envelope {
//     //     xsi: None,
//     //     xsd: None,
//     //     cwmp: None,
//     //     body: None,
//     //     header: None,
//     //     soap_enc: None,
//     //     soap_env: None
//     // }
//     reader.config_mut().trim_text(true);
//     // reader.read_resolved_event().unwrap();
//     // reader.read_resolved_event().unwrap();
//
//     // let ns = reader.prefixes().collect::<Vec<_>>();
//     // tracing::info!("namespace: {:#?}", ns);
//     loop {
//         match reader.read_resolved_event().unwrap() {
//             (ns, Event::Start(e)) => {
//                 // tracing::info!("namespace: {:#?} , debug {:#?}", ns, e);
//                 if let ResolveResult::Bound(namespace) = ns {
//                     // let prefix = reader.prefixes();
//
//                     tracing::info!("Found namespace prefix {:#?} - {:#?}", namespace., e);
//                 }
//             }
//             (_, Event::Start(_)) => unreachable!(),
//             (_, Event::Text(e)) => {
//                 txt.push(e.decode().unwrap().into_owned());
//                 tracing::info!("log text {:#?}", txt);
//             }
//             (_, Event::Eof) => break,
//             _ => (),
//         }
//     }
// }
pub trait HandleCwmpMessage {
    fn parse(xml: &str) -> Self;
}

// impl HandleCwmpMessage for Inform {
//     fn parse(xml: &str) -> Self {
//         Form
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;
    use tracing::{level_filters::LevelFilter, trace};

    static TRACING: OnceLock<()> = OnceLock::new();

    fn spawn_log() {
        if TRACING.get().is_none() {
            let _tracing = TRACING.get_or_init(|| {
                let test_sub = get_subscriber("tr069-server-test".into(), LevelFilter::INFO.into());
                init_subscriber(test_sub);
            });
        }
    }

    #[test]
    fn test_deserialize_soap_xml() {
        spawn_log();
        // let xml = "<?xml version="1.0" encoding="UTF-8"?>\x0a<soap-env:Envelope xmlns:soap-enc="http://schemas.xmlsoap.org/soap/encoding/" xmlns:soap-env="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:cwmp="urn:dslforum-org:cwmp-1-0"><soap-env:Header><cwmp:ID soap-env:mustUnderstand="1">rr8q3um5</cwmp:ID></soap-env:Header><soap-env:Body><cwmp:InformResponse><MaxEnvelopes>1</MaxEnvelopes></cwmp:InformResponse></soap-env:Body></soap-env:Envelope>";
        let xml = r#"
                                    <soap-env:Envelope xmlns:soap-env="http://schemas.xmlsoap.org/soap/envelope/"
                                                       xmlns:soap-enc="http://schemas.xmlsoap.org/soap/encoding/"
                                                       xmlns:xsd="http://www.w3.org/2001/XMLSchema"
                                                       xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
                                                       xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
                                      <soap-env:Header>
                                        <cwmp:ID soap-env:mustUnderstand="1">1234</cwmp:ID>
                                      </soap-env:Header>
                                      <soap-env:Body>
                                        <cwmp:Inform>
                                          <DeviceId>
                                            <Manufacturer>ExampleCorp</Manufacturer>
                                            <OUI>001A2B</OUI>
                                            <ProductClass>RouterX100</ProductClass>
                                            <SerialNumber>SN123456789</SerialNumber>
                                          </DeviceId>
                                          <Event soap-enc:arrayType="cwmp:EventStruct[1]">
                                            <EventStruct>
                                              <EventCode>0 BOOTSTRAP</EventCode>
                                              <CommandKey>Darwin command</CommandKey>
                                            </EventStruct>
                                          </Event>
                                          <MaxEnvelopes>1</MaxEnvelopes>
                                          <CurrentTime>2025-10-01T05:00:00Z</CurrentTime>
                                          <RetryCount>0</RetryCount>
                                          <ParameterList soap-enc:arrayType="cwmp:ParameterValueStruct[2]">
                                            <ParameterValueStruct>
                                              <Name>InternetGatewayDevice.DeviceSummary</Name>
                                              <Value xsi:type="xsd:string">InternetGatewayDevice:1.0[](Baseline:1, EthernetLAN:1, WiFi:1)</Value>
                                            </ParameterValueStruct>
                                            <ParameterValueStruct>
                                              <Name>InternetGatewayDevice.ManagementServer.ConnectionRequestURL</Name>
                                              <Value xsi:type="xsd:string">http://192.168.1.1:7547/</Value>
                                            </ParameterValueStruct>
                                          </ParameterList>
                                        </cwmp:Inform>
                                      </soap-env:Body>
                                    </soap-env:Envelope>
        "#;
        let msg_body = InformResponse { max_envelopes: 1 };
        let res = Envelope::new("dummy", CWMPMsg::InformResponse(msg_body));
        let xml = res.create_xml().unwrap();
        // let res = Envelope::build_message();
        // let dbg = String::from_utf8(res).unwrap();
        // tracing::info!("{}", dbg);
        // parse_xml(xml);
        // let soap_env: Envelope = quick_xml::de::from_str(xml).unwrap();
        tracing::info!("soap evelope test {}", String::from_utf8(xml).unwrap());
    }
}
