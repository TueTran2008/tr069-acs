use crate::cwmp_msg::RpcWrite;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetParamterNames {
    #[serde(rename = "ParameterPath")]
    param_path: String,
    #[serde(rename = "NextLevel")]
    next_level: bool,
}

impl GetParamterNames {
    pub fn new(param_path: String, next_level: bool) -> Self {
        Self {
            param_path,
            next_level,
        }
    }
}

impl<'a, W: std::io::Write> RpcWrite<'a, W> for GetParamterNames {
    fn build_message(&'a self, xml_writer: &'a mut Writer<W>) -> &'a mut Writer<W> {
        // --- <cwmp:InformResponse>
        let inform_res = BytesStart::new("cwmp:GetParamterNames");
        xml_writer.write_event(Event::Start(inform_res)).unwrap();

        // --- ParameterPath
        let parameter_path = BytesStart::new("ParameterPath");
        xml_writer
            .write_event(Event::Start(parameter_path))
            .unwrap();
        xml_writer
            .write_event(Event::Text(BytesText::new(&self.param_path)))
            .unwrap();

        xml_writer
            .write_event(Event::End(BytesEnd::new("ParameterPath")))
            .unwrap();

        // --- Next Level
        let next_level = BytesStart::new("NextLevel");
        xml_writer.write_event(Event::Start(next_level)).unwrap();
        xml_writer
            .write_event(Event::Text(BytesText::new(&self.next_level.to_string())))
            .unwrap();

        xml_writer
            .write_event(Event::End(BytesEnd::new("NextLevel")))
            .unwrap();

        xml_writer
            .write_event(Event::End(BytesEnd::new("cwmp:GetParamterNames")))
            .unwrap();

        // Output - Generate Output
        // xml_writer.into_inner().into_inner()
        xml_writer
    }
}
