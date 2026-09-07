use crate::soap::RpcWrite;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};
use uuid::Bytes;

//const PARAMETER_NAMES_NAMESPACE: &str = r#"GetParamete"#;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetParamterValues {
    #[serde(rename = "ParameterNames")]
    param_names: Vec<String>,
}

impl GetParamterValues {
    pub fn builder() -> GetParamterValuesBuilder {
        GetParamterValuesBuilder::default()
    }
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

        xml_writer
            .write_event(Event::Start(parameter_path))
            .unwrap();

        for path in self.param_names.iter() {
            xml_writer
                .write_event(Event::Start(BytesStart::new("string")))
                .unwrap();
            xml_writer
                .write_event(Event::Text(BytesText::new(path.as_str())))
                .unwrap();
            xml_writer
                .write_event(Event::End(BytesEnd::new("string")))
                .unwrap();
        }

        xml_writer
            .write_event(Event::End(BytesEnd::new("ParameterNames")))
            .unwrap();

        xml_writer
            .write_event(Event::End(BytesEnd::new("cwmp:GetParamterValues")))
            .unwrap();

        // Output - Generate Output
        // xml_writer.into_inner().into_inner()
        xml_writer
    }
}

#[derive(Default)]
pub struct GetParamterValuesBuilder {
    param_names: Vec<String>,
}

impl GetParamterValuesBuilder {
    pub fn add_param<S: Into<String>>(&mut self, param: S) {
        self.param_names.push(param.into());
    }

    pub fn param_names<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.param_names = names.into_iter().map(|name| name.into()).collect();
        self
    }
    pub fn build(self) -> GetParamterValues {
        GetParamterValues {
            param_names: self.param_names,
        }
    }
}
