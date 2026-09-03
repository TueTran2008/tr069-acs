use crate::soap::RpcWrite;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};

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
