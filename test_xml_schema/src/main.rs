fn main() {
    use quick_xml::reader::Reader;
    use xml_schema_generator::{into_struct, Options};

    let xml = r#""<?xml version="1.0" encoding="UTF-8"?>\x0a<soap-env:Envelope xmlns:soap-enc="http://schemas.xmlsoap.org/soap/encoding/" xmlns:soap-env="http://schemas.xmlsoap.org/soap/envelope/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:cwmp="urn:dslforum-org:cwmp-1-0"><soap-env:Header><cwmp:ID soap-env:mustUnderstand="1">6jiyaw7n</cwmp:ID></soap-env:Header><soap-env:Body><cwmp:Inform><DeviceId><Manufacturer>Huawei Technologies Co., Ltd.</Manufacturer><OUI>202BC1</OUI><ProductClass>BM632w</ProductClass><SerialNumber>000000</SerialNumber></DeviceId><Event soap-enc:arrayType="cwmp:EventStruct[1]"><EventStruct><EventCode>2 PERIODIC</EventCode><CommandKey/></EventStruct></Event><MaxEnvelopes>1</MaxEnvelopes><CurrentTime>2025-11-18T14:19:09.541Z</CurrentTime><RetryCount>0</RetryCount><ParameterList soap-enc:arrayType="cwmp:ParameterValueStruct[16]"><ParameterValueStruct><Name>InternetGatewayDevice.DeviceInfo.SpecVersion</Name><Value xsi:type="xsd:string">1</Value></ParameterValueStruct><ParameterValueStruct><Name>InternetGatewayDevice.DeviceInfo.HardwareVersion</Name><Value xsi:type="xsd:string">40501</Value></ParameterValueStruct><ParameterValueStruct><Name>InternetGatewayDevice.DeviceInfo.SoftwareVersion</Name><Value xsi:type="xsd:string">V100R001IRQC56B017</Value></ParameterValueStruct><ParameterValueStruct><Name>InternetGatewayDevice.DeviceInfo.ProvisioningCode</Name><Value xsi:type="xsd:string"/></ParameterValueStruct><ParameterValueStruct><Name>InternetGatewayDevice.ManagementServer.ParameterKey</Name><Value xsi:type="xsd:string"/></ParameterValueStruct><ParameterValueStruct><Name>InternetGatewayDevice.ManagementServer.ConnectionRequestURL</Name><Value xsi:type="xsd:string">http://127.0.0.1:48071/</Value></ParameterValueStruct><ParameterValueStruct><Name>InternetGatewayDevice.WANDevice.1.WANConnectionDevice.1.WANIPConnection.1.ExternalIPAddress</Name><Value xsi:type="xsd:string">172.3.89.139</Value></ParameterValueStruct></ParameterList></cwmp:Inform></soap-env:Body></soap-env:Envelope>"#;
    let mut reader = Reader::from_str(xml);

    if let Ok(root) = into_struct(&mut reader) {
        let struct_as_string = root.to_serde_struct(&Options::quick_xml_de());
        println!("{}", struct_as_string);
        // save this result as a .rs file and use it to (de)serialize an XML document with quick_xml::de::from_str(xml)
    }

    // you can even parse additional compatible xml files to extend the structure to match those files as well
    // see examples/parse_multiple_xml_rs
}
