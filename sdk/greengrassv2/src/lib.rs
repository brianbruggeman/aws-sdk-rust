#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
//! <p>AWS IoT Greengrass brings local compute, messaging, data management, sync, and ML inference capabilities
//! to edge devices. This enables devices to collect and analyze data closer to the source of
//! information, react autonomously to local events, and communicate securely with each other on
//! local networks. Local devices can also communicate securely with AWS IoT Core and export IoT data
//! to the AWS Cloud. AWS IoT Greengrass developers can use AWS Lambda functions and components to create and
//! deploy applications to fleets of edge devices for local operation.</p>
//! <p>AWS IoT Greengrass Version 2 provides a new major version of the AWS IoT Greengrass Core software, new APIs, and a new console.
//! Use this API reference to learn how to use the AWS IoT Greengrass V2 API operations to manage components,
//! manage deployments, and core devices.</p>
//! <p>For more information, see <a href="https://docs.aws.amazon.com/greengrass/v2/developerguide/what-is-iot-greengrass.html">What is AWS IoT Greengrass?</a> in
//! the <i>AWS IoT Greengrass V2 Developer Guide</i>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
mod http_serde;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("greengrassv2", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;