use dioxus::prelude::*;
// Launch axum on the server
use axum::Router;
#[cfg(feature = "server")]
use tokio::runtime::Runtime;
// use tr

mod cwmp_msg;
mod soap_xml;
mod startup;
mod telemetry;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[cfg(feature = "server")]
async fn launch_server(component: fn() -> Element) {
    // Connect dioxus's logging infrastructure

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    dioxus::logger::initialize_default();
    let ip =
        dioxus::cli_config::server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = dioxus::cli_config::server_port().unwrap_or(8081);
    let server_addr = SocketAddr::new(ip, port);
    tracing::info!("{server_addr}");

    //Build a custom router

    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();
    let listener = tokio::net::TcpListener::bind(server_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
    // axum::serve(server_addr, router).await.unwrap();

    // let cwmp_listener = tokio::net::
}

fn main() {
    #[cfg(feature = "web")]
    dioxus::launch(App);

    // Launch axum on server;
    #[cfg(feature = "server")]
    {
        // use tokio::runtime::Runtime;

        if let Ok(rt) = Runtime::new() {
            rt.block_on(async move { launch_server(App).await });
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
