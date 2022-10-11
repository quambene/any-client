mod codec;
mod config;
mod convert_descriptors;

use crate::client::grpc_client::{
    codec::DynamicCodec, config::Config, convert_descriptors::convert_file_descriptor_proto,
};
use anyhow::{anyhow, Context};
use http::{uri::PathAndQuery, Uri};
use log::{error, info};
use prost_reflect::{DescriptorPool, DynamicMessage};
use serde_json::Deserializer;
use std::{path::Path, str::FromStr};
use tokio_stream::StreamExt;
use tonic::{client::Grpc, transport::Channel, IntoRequest};

pub async fn use_grpc_client(config_file: String) -> Result<(), anyhow::Error> {
    info!("Using grpc client");

    let config: Config = serde_json::from_str(&config_file).context("Can't deserialize json")?;
    let url = config.api.url;
    let proto_path = Path::new(&config.proto.path);
    let proto_file = Path::new(&config.proto.file);

    info!("Proto path: {}", proto_path.display());
    info!("Proto file: {}", proto_file.display());

    let mut file_descriptor_protos = protobuf_parse::Parser::new()
        .pure()
        .includes(&[proto_path])
        .input(proto_file)
        .parse_and_typecheck()
        .context(format!("Can't parse proto file '{}'", proto_file.display()))?
        .file_descriptors;

    let file_descriptor_proto = file_descriptor_protos
        .pop()
        .ok_or_else(|| anyhow!("Can't get proto"))?;

    let file_descriptor_proto = convert_file_descriptor_proto(file_descriptor_proto);

    let mut pool = DescriptorPool::new();
    pool.add_file_descriptor_proto(file_descriptor_proto)
        .context("Can't add file descriptor")?;

    let message_descriptor = pool
        .get_message_by_name(format!("{}.{}", config.proto.package, config.proto.message).as_str())
        .context("Can't get message descriptor")?;

    let service_descriptor = pool
        .services()
        .find(|service| service.name() == config.proto.service)
        .context("Can't get service descriptor")?;

    let method_descriptor = service_descriptor
        .methods()
        .find(|method| method.name() == config.proto.method)
        .context("Can't get method descriptor")?;

    let message = config
        .proto
        .request
        .context("Can't get message")?
        .to_string();
    let mut deserializer = Deserializer::from_str(message.as_str());
    let dynamic_message = DynamicMessage::deserialize(message_descriptor, &mut deserializer)
        .context("Can't deserialize dynamic message")?;
    deserializer.end().context("Can't end deserializer")?;

    let uri: Uri = url.parse().context("Can't parse url")?;

    let builder = Channel::builder(uri);
    let channel = builder.connect().await.context("Can't create channel")?;

    let mut client = Grpc::new(channel);

    client.ready().await.context("Client not ready")?;

    let path = PathAndQuery::from_str(&format!(
        "/{}/{}",
        method_descriptor.parent_service().full_name(),
        method_descriptor.name()
    ))
    .context("Can't parse endpoint")?;

    let codec = DynamicCodec::new(method_descriptor);

    let request = dynamic_message.into_request();

    let response = client
        .server_streaming(request, path, codec)
        .await
        .context("Can't send grpc request")?;

    let mut stream = response.into_inner();

    while let Some(item) = stream.next().await {
        match item {
            Ok(msg) => println!("{}", msg),
            Err(err) => error!("{}", err),
        }
    }

    Ok(())
}
