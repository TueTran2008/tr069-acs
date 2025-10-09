use std::sync::OnceLock;
use tokio;
use tracing::{info, level_filters::LevelFilter};

use crate::{
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use http_body_util::BodyExt;

static TRACING: OnceLock<()> = OnceLock::new();

fn spawn_log() {
    if TRACING.get().is_none() {
        let _tracing = TRACING.get_or_init(|| {
            let test_sub = get_subscriber("tr069-server-test".into(), LevelFilter::INFO.into());
            init_subscriber(test_sub);
        });
    }
}

// use http_body_util::BodyExt;
pub async fn cwmp_session_handle(msg: String) {
    info!("{}", msg);
}

async fn contruct_test_acs() -> u16 {
    // let app = Router::new()
    //     .route("/", post(cwmp_session_handle))
    //     // .route("/", post(cwmp_session_handle))
    //     .layer(middleware::from_fn(print_request_response));
    //
    // // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    info!("listening on {}", port);
    let server = run(listener);
    tokio::spawn(server);
    port
}

pub async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    info!("Print request in in middleware response {:?}", res);

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{direction} body = {body:?}");
    }

    Ok(bytes)
}

#[tokio::test]
async fn test_send_inform_message() {
    spawn_log();
    let port = contruct_test_acs().await;
    const INFORM_MESSAGE: &str = r#"
                                    <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
                                                   xmlns:cwmp="urn:dslforum-org:cwmp-1-0">
                                      <soap:Header>
                                        <cwmp:ID soap:mustUnderstand="true">1</cwmp:ID>
                                      </soap:Header>
                                      <soap:Body>
                                        <cwmp:Inform>
                                          <DeviceId>
                                            <Manufacturer>ExampleCo</Manufacturer>
                                            <OUI>001A2B</OUI>
                                            <ProductClass>RouterX</ProductClass>
                                            <SerialNumber>123456789</SerialNumber>
                                          </DeviceId>
                                          <Event>
                                            <EventStruct>
                                              <EventCode>2 PERIODIC</EventCode>
                                              <CommandKey></CommandKey>
                                            </EventStruct>
                                          </Event>
                                          <MaxEnvelopes>1</MaxEnvelopes>
                                          <CurrentTime>2025-10-07T10:00:00Z</CurrentTime>
                                          <RetryCount>0</RetryCount>
                                          <ParameterList soap:arrayType="cwmp:ParameterValueStruct[1]">
                                            <ParameterValueStruct>
                                              <Name>Device.ManagementServer.URL</Name>
                                              <Value xsi:type="xsd:string"
                                                     xmlns:xsd="http://www.w3.org/2001/XMLSchema"
                                                     xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
                                                http://acs.example.com/acs
                                              </Value>
                                            </ParameterValueStruct>
                                          </ParameterList>
                                        </cwmp:Inform>
                                      </soap:Body>
                                    </soap:Envelope>
                                    "#;
    //ACS Endpoint
    let url = format!("http://127.0.0.1:{}", port);

    // Build http client
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "\"\"")
        .body(INFORM_MESSAGE)
        .send()
        .await
        .unwrap();

    // assert!();
}
