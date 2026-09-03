use crate::soap::RpcWrite;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Writer;
use serde::{Deserialize, Serialize};

//const PARAMETER_NAMES_NAMESPACE: &str = r#"GetParamete"#;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRPCMethods {}

impl<'a, W> RpcWrite<'a, W> for GetRPCMethods
where
    W: std::io::Write,
{
    fn build_message(&'a self, xml_writer: &'a mut Writer<W>) -> &'a mut Writer<W> {
        // --- <cwmp:GetRPCMethods>
        let rpc_start = BytesStart::new("cwmp:GetRPCMethods");
        xml_writer.write_event(Event::Start(rpc_start)).unwrap();
        xml_writer
            .write_event(Event::End(BytesEnd::new("cwmp:GetRPCMethods")))
            .unwrap();
        xml_writer
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRPCMethodsResponse {
    #[serde(rename = "MethodList")]
    method_list: Vec<String>,
}
