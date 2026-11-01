use crate::soap::RpcWrite;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetParameterNames {
    #[serde(rename = "ParameterPath")]
    param_path: String,

    #[serde(rename = "NextLevel")]
    next_level: bool,
}

impl GetParameterNames {
    pub fn new(param_path: String, next_level: bool) -> Self {
        Self {
            param_path,
            next_level,
        }
    }

    pub fn builder() -> GetParameterNamesBuilder {
        GetParameterNamesBuilder::default()
    }
}

impl<'a, W: std::io::Write> RpcWrite<'a, W> for GetParameterNames {
    fn build_message(&'a self, xml_writer: &'a mut Writer<W>) -> &'a mut Writer<W> {
        // --- <cwmp:InformResponse>
        let inform_res = BytesStart::new("cwmp:GetParameterNames");
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
            .write_event(Event::Text(BytesText::new(
                &u8::from(self.next_level).to_string(),
            )))
            .unwrap();

        xml_writer
            .write_event(Event::End(BytesEnd::new("NextLevel")))
            .unwrap();

        xml_writer
            .write_event(Event::End(BytesEnd::new("cwmp:GetParameterNames")))
            .unwrap();

        // Output - Generate Output
        // xml_writer.into_inner().into_inner()
        xml_writer
    }
}

#[derive(Default)]
pub struct GetParameterNamesBuilder {
    param_path: String,
    next_level: bool,
}

impl GetParameterNamesBuilder {
    pub fn new(param_path: String, next_level: bool) -> Self {
        Self {
            param_path,
            next_level,
        }
    }
    pub fn param_path<S: Into<String>>(mut self, param_path: S) -> Self {
        self.param_path = param_path.into();
        self
    }

    pub fn next_level(mut self, next_level: bool) -> Self {
        self.next_level = next_level;
        self
    }

    pub fn build(self) -> GetParameterNames {
        GetParameterNames {
            param_path: self.param_path,
            next_level: self.next_level,
        }
    }
}
