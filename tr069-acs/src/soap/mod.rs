pub mod fault;
pub mod get_names;
pub mod get_rpc;
pub mod get_values;
pub mod inform;
use quick_xml::Writer;
/// Define methods use to build soap message
pub(crate) trait RpcWrite<'a, W>
where
    W: std::io::Write,
{
    fn build_message(&'a self, xml_writer: &'a mut Writer<W>) -> &'a mut Writer<W>;
}
