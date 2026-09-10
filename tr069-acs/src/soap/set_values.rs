use crate::cwmp_msg::{ParameterList, ParameterValueStruct};
use crate::soap::RpcWrite;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};
use uuid::Bytes;

const SET_PARAMETER_VALUES: &str = "cwmp:SetParameterValues";
const PARAMETER_LIST: &str = "ParameterList";
const NAME: &str = "Name";
const VALUE: &str = "Value";

#[derive(Debug, Deserialize, Serialize)]
pub struct SetParamterValues {
    #[serde(rename = "ParameterList")]
    param_names: ParameterList,

    #[serde(rename = "ParameterKey")]
    parameter_key: String,
}

impl<'a, W> RpcWrite<'a, W> for SetParamterValues
where
    W: std::io::Write,
{
    fn build_message(&'a self, xml_writer: &'a mut Writer<W>) -> &'a mut Writer<W> {
        let rpc_start = BytesStart::new(SET_PARAMETER_VALUES);
        let rpc_stop = BytesEnd::new(SET_PARAMETER_VALUES);

        xml_writer.write_event(Event::Start(rpc_start)).unwrap();

        // --- ParameterPath
        let arr_value = format!("xsd:string[{}]", self.param_names.parameter_struct.len());
        let mut parameter_path = BytesStart::new(PARAMETER_LIST);

        parameter_path.push_attribute(("soap-enc:arrayType", arr_value.as_str()));

        xml_writer
            .write_event(Event::Start(parameter_path))
            .unwrap();

        for path in self.param_names.parameter_struct.iter() {
            xml_writer
                .write_event(Event::Start(BytesStart::new(NAME)))
                .unwrap();
            xml_writer
                .write_event(Event::Text(BytesText::new(path.name.as_ref().unwrap())))
                .unwrap();
            xml_writer
                .write_event(Event::End(BytesEnd::new(NAME)))
                .unwrap();

            let mut parameter_value = BytesStart::new(PARAMETER_LIST);
            parameter_value.push_attribute((
                "xsi:type",
                path.value
                    .as_ref()
                    .unwrap()
                    .xsi_type
                    .as_ref()
                    .unwrap()
                    .as_str(),
            ));
            xml_writer
                .write_event(Event::Start(parameter_value))
                .unwrap();

            xml_writer
                .write_event(Event::Text(BytesText::new(
                    path.value
                        .as_ref()
                        .unwrap()
                        .value
                        .as_ref()
                        .unwrap()
                        .as_str(),
                )))
                .unwrap();
            xml_writer
                .write_event(Event::End(BytesEnd::new(VALUE)))
                .unwrap();
        }

        xml_writer
            .write_event(Event::End(BytesEnd::new(PARAMETER_LIST)))
            .unwrap();

        xml_writer.write_event(Event::End(rpc_stop)).unwrap();
        xml_writer
    }
}

impl SetParamterValues {
    pub fn builder() -> SetParamterValuesBuilder {
        SetParamterValuesBuilder::default()
    }
}

#[derive(Default)]
pub struct SetParamterValuesBuilder {
    param_names: Vec<ParameterValueStruct>,
    parameter_key: String,
}
