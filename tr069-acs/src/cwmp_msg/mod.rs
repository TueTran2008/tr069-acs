use quick_xml::de::*;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tracing::{error, info, level_filters::LevelFilter, trace};

use crate::telemetry::{get_subscriber, init_subscriber};

#[derive(Debug)]
pub enum CwmpMsg {
    InformMsg(Inform),
}

#[derive(Debug, Deserialize)]
enum EventStruct {
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

#[derive(Debug, Default, Deserialize)]
#[serde(rename = "pascal_case")]
struct DeviceIDStruct {
    // #[serde(rename = "Manufacturer")]
    manufacturer: Option<String>,
    // #[serde(rename = "OUI")]
    oui: Option<String>,
    // #[serde(rename = "ProductClass")]
    product_class: Option<String>,
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
    value: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ParameterValueStruct {
    name: Option<String>,
    //This is the value the Parameter is to be set. The CPE
    //MUST treat string-valued Parameter values as casesensitive.
    value: Option<AnySimpleType>,
}

#[derive(Debug, Default, Deserialize)]
struct Inform {
    #[serde(rename = "DeviceID")]
    device_id: DeviceIDStruct,

    #[serde(rename = "Event")]
    event: Vec<Option<EventStruct>>,

    #[serde(rename = "MaxEnvelopes")]
    max_envelopes: u32,

    #[serde(rename = "CurrentTime")]
    current_time: u32,

    #[serde(rename = "RetryCount")]
    retry_count: u32,

    #[serde(rename = "ParameterList")]
    parameter_list: Option<Vec<Option<ParameterValueStruct>>>,
}

pub trait HandleCwmpMessage {
    fn parse(xml: &str) -> Self;
}

impl HandleCwmpMessage for Inform {
    fn parse(xml: &str) -> Self {
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        reader.config_mut().trim_text(true);
        loop {
            // NOTE: this is the generic case when we don't know about the input BufRead.
            // when the input is a &str or a &[u8], we don't actually need to use another
            // buffer, we could directly call `reader.read_event()`
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,

                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"soap:Envelope" => {
                        trace!("Soap envelope value {:?}", e.attributes());
                    }
                    _ => {
                        // trace!(
                        //     "attributes values: {:?}",
                        //     e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                        // );
                        trace!("Test soap message {:?}", e);
                    }
                },
                Ok(Event::Text(_e)) => (),

                // There are several other `Event`s we do not consider here
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        Inform::default()
    }
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
    GetParameterValuesResponse,
}

#[derive(Deserialize, Debug)]
struct Body {
    #[serde(rename = "$value")]
    msg_type: CWMPMsg,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "Envelope")]
struct Envelope {
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

static TRACING: OnceLock<()> = OnceLock::new();

fn spawn_log() {
    if TRACING.get().is_none() {
        let _tracing = TRACING.get_or_init(|| {
            let test_sub = get_subscriber("tr069-server-test".into(), LevelFilter::TRACE.into());
            init_subscriber(test_sub);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize_soap_xml() {
        spawn_log();
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
                                              <CommandKey></CommandKey>
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
