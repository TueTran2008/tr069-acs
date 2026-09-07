use crate::cwmp_msg::ParameterValueStruct;
use crate::soap::RpcWrite;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};
use uuid::Bytes;

#[derive(Debug, Deserialize, Serialize)]
pub struct SetParamterValues {
    #[serde(rename = "ParameterList")]
    param_names: Vec<ParameterValueStruct>,

    #[serde(rename = "ParameterKey")]
    parameter_key: String,
}

impl<'a, W> RpcWrite<'a, W> for SetParamterValues
where
    W: std::io::Write,
{
    fn build_message(&'a self, xml_writer: &'a mut Writer<W>) -> &'a mut Writer<W> {}
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
