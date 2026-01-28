use crate::cwmp_msg::RpcWrite;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};

//const PARAMETER_NAMES_NAMESPACE: &str = r#"GetParamete"#;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetParamterValues {
    #[serde(rename = "ParameterNames")]
    param_names: Vec<String>,
}

impl<'a, W: std::io::Write> RpcWrite<'a, W> for GetParamterValues {
    fn build_message(&'a self, xml_writer: &'a mut Writer<W>) -> &'a mut Writer<W> {
        // --- <cwmp:InformResponse>
        let get_values = BytesStart::new("cwmp:GetParamterValues");
        xml_writer.write_event(Event::Start(get_values)).unwrap();

        // --- ParameterPath
        let arr_value = format!("xsd:string[{}]", self.param_names.len());
        let mut parameter_path = BytesStart::new("ParameterNames");

        parameter_path.push_attribute(("soap-enc:arrayType", arr_value.as_str()));

        //.push_attribute(("soap-enc:arrayType", arr_value.as_str()));
        xml_writer
            .write_event(Event::Start(parameter_path))
            .unwrap();
        //let ele = Event::Start(Vgc)
        //xml_writer.wi
        //xml_writer
        //    .write_event(Event::Text(BytesText::new(&self.param_path)))
        //    .unwrap();

        xml_writer
            .write_event(Event::End(BytesEnd::new("ParameterPath")))
            .unwrap();

        // --- Next Level
        //let next_level = BytesStart::new("NextLevel");
        //xml_writer.write_event(Event::Start(next_level)).unwrap();
        //xml_writer
        //    .write_event(Event::Text(BytesText::new(&self.next_level.to_string())))
        //    .unwrap();
        //
        //xml_writer
        //    .write_event(Event::End(BytesEnd::new("NextLevel")))
        //    .unwrap();
        //
        xml_writer
            .write_event(Event::End(BytesEnd::new("cwmp:GetParamterValues")))
            .unwrap();

        // Output - Generate Output
        // xml_writer.into_inner().into_inner()
        xml_writer
    }
}
