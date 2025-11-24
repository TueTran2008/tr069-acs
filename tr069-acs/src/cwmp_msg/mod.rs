pub mod consts;
pub mod session;

use crate::{
    cwmp_msg::consts::{SOAP_CWMP_NP, SOAP_ENC_NP, SOAP_XSD_NP, SOAP_XSI_NP},
    telemetry::{get_subscriber, init_subscriber},
};
use std::{collections::HashMap, sync::OnceLock};
use tracing::level_filters::LevelFilter;
// use yaserde::{YaDeserialize, YaSerialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Debug)]
pub enum CpeRPC {
    Inform,
    GetRPCMethodsResponse,
    SetParameterValuesResponse,
    GetParameterValuesResponse,
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
}

// #[derive(YaSerialize, YaDeserialize)]
// enum EventCode {
//     Event0BootStrap,
//     Event1Boot,
//     Event2Periodic,
//     Event3Schedule,
//     Event4ValueChange,
//     Event5Kicked,
//     Event6ConnectionRequest,
//     Event7TransferComplete,
//     Event8DiagnosticComplete,
//     Event9RequestDownload,
//     Event10AutonomousTransferComplete,
//     Event11DUStateChangeComplete,
//     Event12AutonomousDUStateChangeComplete,
//     Event13Wakeup,
//     Event14Heartbeat,
//     EventMReboot,
//     EventMScheduleInform,
//     EventMDownload,
//     EventMScheduleDownload,
//     EventMUpload,
//     EventMChangeDUState,
//     EventMVendorMethod,
//     EventMVendorEvent,
// }

#[derive(Debug, YaSerialize, YaDeserialize)]
struct CommandKey {
    value: Option<String>,
}

#[derive(Debug, YaSerialize, YaDeserialize, Default)]
struct EventStruct {
    // #[yaserde(rename = "@arrayType")]
    // nb_of_event: Option<String>,
    #[yaserde(rename = "EventCode")]
    event_code: Option<String>,
    #[yaserde(rename = "CommandKey")]
    command_key: Option<String>,
}

#[derive(Debug, YaSerialize, YaDeserialize, Default)]
struct EventList {
    #[yaserde(rename = "@arrayType")]
    nb_of_event: Option<String>,

    #[yaserde(rename = "EventStruct")]
    event_struct: Vec<EventStruct>,
}

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
struct DeviceIDStruct {
    #[yaserde(rename = "Manufacturer")]
    manufacturer: Option<String>,

    #[yaserde(rename = "OUI")]
    oui: Option<String>,

    #[yaserde(rename = "ProductClass")]
    product_class: Option<String>,

    #[yaserde(rename = "SerialNumber")]
    serial_number: Option<String>,
}

//The value of an element defined to be of type “anySimpleType” MAY be of any simple data type,
// including (but not limited to) any of the other types listed in this table.
// Following the SOAP specification [12], elements specified as being of type “anySimpleType” MUST
// include a type attribute to indicate the actual type of the element. For example:
// <ParameterValueStruct>
//  <Name>Device.DeviceInfo.ProvisioningCode</Name>
//  <Value xsi:type="xsd:string">code12345</Value>
// </ParameterValueStruct>
// The namespaces xsi and xsd used above are as defined in [12].
#[derive(Debug, YaSerialize, YaDeserialize, Default)]
struct AnySimpleType {
    #[yaserde(rename = "@type")]
    xsi_type: Option<String>,

    #[yaserde(rename = "$text")]
    value: Option<String>,
}

#[derive(Debug, YaSerialize, YaDeserialize, Default)]
struct ParameterValueStruct {
    #[yaserde(rename = "Name")]
    name: Option<String>,
    //This is the value the Parameter is to be set. The CPE
    //MUST treat string-valued Parameter values as casesensitive.
    #[yaserde(rename = "Value")]
    value: Option<AnySimpleType>,
}

#[derive(Debug, YaSerialize, YaDeserialize, Default)]
struct ParameterList {
    #[yaserde(rename = "ParameterValueStruct")]
    parameter_struct: Vec<ParameterValueStruct>,

    #[yaserde(rename = "@arrayType")]
    nb_of_parameter: Option<String>,
}

#[derive(Debug, YaSerialize, YaDeserialize, Default)]
struct Inform {
    #[yaserde(rename = "DeviceId")]
    device_id: DeviceIDStruct,

    #[yaserde(rename = "Event")]
    event: EventList,

    #[yaserde(rename = "MaxEnvelopes")]
    max_envelopes: u32,

    #[yaserde(rename = "CurrentTime")]
    current_time: String,

    #[yaserde(rename = "RetryCount")]
    retry_count: u32,

    #[yaserde(rename = "ParameterList")]
    parameter_list: Vec<ParameterList>,
}

#[derive(Debug, YaSerialize, YaDeserialize)]
pub struct InformResponse {
    #[yaserde(rename = "MaxEnvelopes")]
    max_envelopes: u32,
}

#[derive(YaSerialize, Debug, YaDeserialize)]
#[yaserde(rename = "cwmp:ID")]
struct ID {
    #[yaserde(rename = "@mustUnderstand")]
    must_understand: Option<String>,
    #[yaserde(rename = "$text")]
    text: Option<String>,
}

#[derive(YaSerialize, Debug, YaDeserialize)]
struct Header {
    #[yaserde(rename = "ID")]
    id: ID,
}

#[derive(YaSerialize, Default, YaDeserialize, Debug)]
pub enum CWMPMsg {
    #[default]
    DefaultMsg,
    Inform(Inform),
    InformResponse(InformResponse),
    GetRPCMethodsResponse,
    SetParameterValuesResponse,
    GetParameterValuesResponse,
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
}

#[derive(YaSerialize, Debug, YaDeserialize)]
struct Body {
    #[yaserde(rename = "$value")]
    msg_type: CWMPMsg,
}

#[derive(YaSerialize, Debug, YaDeserialize)]
#[yaserde(rename = "soap-env:Envelope")]
pub struct Envelope {
    #[yaserde(rename = "@xmlns:soap-enc")]
    soap_enc: Option<String>,

    #[yaserde[rename = "@xmlns:soap-env"]]
    soap_env: Option<String>,

    #[yaserde[rename = "@xmlns:xsd"]]
    xsd: Option<String>,

    #[yaserde(rename = "@xmlns:xsi")]
    xsi: Option<String>,

    #[yaserde(rename = "@xmlns:cwmp")]
    cwmp: Option<String>,

    #[yaserde(rename = "Header")]
    header: Option<Header>,

    #[yaserde(rename = "Body")]
    body: Option<Body>,
    // #[yaserde(flatten)]
    // pub attrs: std::collections::HashMap<String, String>,
}

impl Body {
    pub fn new(msg_body: CWMPMsg) -> Self {
        Body { msg_type: msg_body }
    }
}
impl Default for Header {
    fn default() -> Self {
        let default_id = ID {
            must_understand: Some(String::from("1")),
            text: None,
        };
        Header { id: default_id }
    }
}
// impl Envelope {
//     pub fn get_rpc_type(self) -> CWMPMsg {
//         self.body.msg_type
//     }
// }

impl Envelope {
    pub fn new(msg_body: CWMPMsg) -> Self {
        Self {
            cwmp: Some(String::from(SOAP_CWMP_NP)),
            soap_enc: Some(String::from(SOAP_ENC_NP)),
            xsi: Some(String::from(SOAP_ENC_NP)),
            xsd: Some(String::from(SOAP_XSD_NP)),
            soap_env: Some(String::from(SOAP_XSI_NP)),
            header: Some(Header::default()),
            body: Some(Body { msg_type: msg_body }),
            // attrs: HashMap::new(),
        }
    }
}
impl Default for InformResponse {
    fn default() -> Self {
        Self { max_envelopes: 1 }
    }
}

static TRACING: OnceLock<()> = OnceLock::new();

fn spawn_log() {
    if TRACING.get().is_none() {
        let _tracing = TRACING.get_or_init(|| {
            let test_sub = get_subscriber("tr069-server-test".into(), LevelFilter::TRACE.into());
            init_subscriber(test_sub);
        });
    }
}

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
    #[test]
    fn test_deserialize_soap_xml() {
        // spawn_log();
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

        let soap_env: Envelope = yaserde::de::from_str(xml).unwrap();
        // trace!("soap evelope test {:?}", soap_env);
    }
}
