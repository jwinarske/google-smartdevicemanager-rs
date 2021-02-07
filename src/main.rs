use float_pretty_print::PrettyPrintFloat;
use google::home::enterprise::sdm::v1::*;
use prost::alloc::collections::BTreeMap;
use prost_types::value::Kind;
use prost_types::Value;
use smart_device_management_service_client::SmartDeviceManagementServiceClient;
use std::env;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};
use yup_oauth2::{AccessToken, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

pub mod google {
    pub mod api {
        include!("api/google.api.rs");
    }

    pub mod protobuf {
        include!("api/google.protobuf.rs");
    }

    pub mod home {
        pub mod enterprise {
            pub mod sdm {
                pub mod v1 {
                    include!("api/google.home.enterprise.sdm.v1.rs");
                }
            }
        }
    }
}

static ENDPOINT: &str = "https://smartdevicemanagement.googleapis.com";

fn dump_list(list: &Vec<Value>, prefix: &str) {
    for value in list {
        match value.kind.as_ref().unwrap() {
            Kind::NullValue(_) => {
                println!("{}\t\tNullValue", prefix);
            }
            Kind::NumberValue(num) => {
                println!("{}\t\t{}", prefix, PrettyPrintFloat(*num));
            }
            Kind::StringValue(str) => {
                println!("{}\t\t\"{}\"", prefix, str);
            }
            Kind::BoolValue(b) => {
                println!("{}\t\t{}", prefix, b);
            }
            Kind::StructValue(val) => {
                dump_map(&val.fields, prefix);
            }
            Kind::ListValue(list) => {
                dump_list(&list.values, prefix);
            }
        }
    }
}

fn dump_map(map: &BTreeMap<String, Value>, prefix: &str) {
    for (key, value) in map {
        println!("[{}]", key);
        match value.kind.as_ref().unwrap() {
            Kind::NullValue(_) => {
                println!("{}\tNullValue", prefix);
            }
            Kind::NumberValue(num) => {
                println!("{}\t{}", prefix, PrettyPrintFloat(*num));
            }
            Kind::StringValue(str) => {
                println!("{}\t\"{}\"", prefix, str);
            }
            Kind::BoolValue(b) => {
                println!("{}\t{}", prefix, b);
            }
            Kind::StructValue(val) => {
                dump_map(&val.fields, prefix);
            }
            Kind::ListValue(list) => {
                dump_list(&list.values, prefix);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Required Environmental variables
    let project_guid = env::var("PROJECT_GUID")
        .expect("export PROJECT_GUID as your Smart Device Manager project-id");

    let credentials = env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .expect("export GOOGLE_APPLICATION_CREDENTIALS pointing to your clientsecret.json");

    let token_storage = env::var("GOOGLE_APPLICATION_TOKEN_STORAGE")
        .expect("export GOOGLE_APPLICATION_TOKEN_STORAGE path");

    //
    // Service Configuration
    //
    let secret = yup_oauth2::read_application_secret(credentials)
        .await
        .expect("export GOOGLE_APPLICATION_CREDENTIALS pointing to your clientsecret.json");
    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(token_storage)
        .build()
        .await
        .unwrap();

    let scopes = &["https://www.googleapis.com/auth/sdm.service"];

    let access_token: AccessToken = auth.token(scopes).await?;
    let bearer_token = format!("Bearer {}", access_token.as_str());
    let token = MetadataValue::from_str(bearer_token.as_str())?;

    let certs = tokio::fs::read("certs/roots.pem").await?;

    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(certs.as_slice()))
        .domain_name("smartdevicemanagement.googleapis.com");

    let channel = Channel::from_static(ENDPOINT)
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut client = SmartDeviceManagementServiceClient::with_interceptor(
        channel,
        move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        },
    );

    let structures = client
        .list_structures(ListStructuresRequest {
            parent: format!("enterprises/{}", project_guid),
            page_size: 0,
            page_token: "".to_string(),
            filter: "".to_string(),
        })
        .await?
        .into_inner();

    for structure in structures.structures {
        println!("Structure Name: {}", structure.name);
        if structure.traits.is_some() {
            let map = &structure.traits.unwrap().fields;
            dump_map(&map, "");
        }
        let rooms = client
            .list_rooms(ListRoomsRequest {
                parent: structure.name,
                page_size: 0,
                page_token: "".to_string(),
            })
            .await?
            .into_inner();

        for room in rooms.rooms {
            println!("{}", room.name);
            if room.traits.is_some() {
                let map = &room.traits.unwrap().fields;
                dump_map(&map, "");
            }
        }
    }

    let devices = client
        .list_devices(ListDevicesRequest {
            parent: format!("enterprises/{}", project_guid),
            page_size: 0,
            page_token: "".to_string(),
            filter: "".to_string(),
        })
        .await?
        .into_inner();

    for device in devices.devices {
        println!("Device Name: {}", device.name);
        println!("Device Type: {}", device.r#type);
        if device.traits.is_some() {
            let map = device.traits.unwrap().fields;
            dump_map(&map, "");
        }
    }

    Ok(())
}
