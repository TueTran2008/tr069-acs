pub mod session;

use crate::telemetry::{get_subscriber, init_subscriber};
use quick_xml::de::*;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tracing::{level_filters::LevelFilter, trace};

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

#[derive(Debug, Deserialize)]
enum EventCode {
    Event0BootStrap,
    Event1Boot,
    Event2Periodic,
    Event3Schedule,
    Event4ValueChange,
    Event5Kicked,
    Event6ConnectionRequest,
    Event7TransferComplete,
    Event8DiagnosticComplete,
    Event9RequestDownload,
    Event10AutonomousTransferComplete,
    Event11DUStateChangeComplete,
    Event12AutonomousDUStateChangeComplete,
    Event13Wakeup,
    Event14Heartbeat,
    EventMReboot,
    EventMScheduleInform,
    EventMDownload,
    EventMScheduleDownload,
    EventMUpload,
    EventMChangeDUState,
    EventMVendorMethod,
    EventMVendorEvent,
}

#[derive(Debug, Deserialize)]
struct CommandKey {
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EventStruct {
    // #[serde(rename = "@arrayType")]
    // nb_of_event: Option<String>,
    #[serde(rename = "EventCode")]
    event_code: Option<String>,
    #[serde(rename = "CommandKey")]
    command_key: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct EventList {
    #[serde(rename = "@arrayType")]
    nb_of_event: Option<String>,

    #[serde(rename = "EventStruct")]
    event_struct: Vec<EventStruct>,
}

#[derive(Debug, Default, Deserialize)]
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

//The value of an element defined to be of type “anySimpleType” MAY be of any simple data type,
// including (but not limited to) any of the other types listed in this table.
// Following the SOAP specification [12], elements specified as being of type “anySimpleType” MUST
// include a type attribute to indicate the actual type of the element. For example:
// <ParameterValueStruct>
//  <Name>Device.DeviceInfo.ProvisioningCode</Name>
//  <Value xsi:type="xsd:string">code12345</Value>
// </ParameterValueStruct>
// The namespaces xsi and xsd used above are as defined in [12].
#[derive(Debug, Deserialize)]
struct AnySimpleType {
    #[serde(rename = "@type")]
    xsi_type: Option<String>,

    #[serde(rename = "$text")]
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ParameterValueStruct {
    #[serde(rename = "Name")]
    name: Option<String>,
    //This is the value the Parameter is to be set. The CPE
    //MUST treat string-valued Parameter values as casesensitive.
    #[serde(rename = "Value")]
    value: Option<AnySimpleType>,
}

#[derive(Debug, Deserialize)]
struct ParameterList {
    #[serde(rename = "ParameterValueStruct")]
    parameter_struct: Vec<ParameterValueStruct>,

    #[serde(rename = "@arrayType")]
    nb_of_parameter: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct Inform {
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

#[derive(Deserialize, Debug)]
struct ID {
    #[serde(rename = "@mustUnderstand")]
    must_understand: Option<String>,
}
#[derive(Deserialize, Debug)]
struct Header {
    #[serde(rename = "ID")]
    id: ID,
}

#[derive(Deserialize, Debug)]
enum CWMPMsg {
    Inform(Inform),
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

#[derive(Deserialize, Debug)]
struct Body {
    #[serde(rename = "$value")]
    msg_type: CWMPMsg,
}

#[derive(Deserialize, Debug)]
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
    header: Header,

    #[serde(rename = "Body")]
    body: Body,
}

impl Envelope {
    pub fn get_rpc_type(&self) {}
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

        let soap_env: Envelope = quick_xml::de::from_str(xml).unwrap();
        trace!("soap evelope test {:?}", soap_env);
    }
}
