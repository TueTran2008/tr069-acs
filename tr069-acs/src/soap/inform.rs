use crate::cwmp_msg::{DeviceIDStruct, EventList, ParameterList};
use crate::soap::RpcWrite;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};

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
