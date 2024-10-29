extern crate serde_json;

use std::net::IpAddr;

use elasticsearch::{
    auth::Credentials, 
    http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder}, 
    Elasticsearch, 
    SearchParts
};

use serde_json::Value;
use url::Url;
use utils::structs::{Flow, Source};

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setting up ES credentials and URL
    let credentials = Credentials::Basic("elastic".into(), "elastic".into());
    let u = Url::parse("http://localhost:9200")?;
    let conn_pool = SingleNodeConnectionPool::new(u);
    let transport = TransportBuilder::new(conn_pool).auth(credentials).build()?;
    
    // Create Client
    let client = Elasticsearch::new(transport);

    // Querying Suricata Index - paginated request to do, retrieving up to 10000 documents
    // add geo location
    let response = client
        .search(SearchParts::Index(&[".ds-logs-suricata.eve-default-2024.10.10-000002"]))
        .body(serde_json::json!({
            "_source": {
                "includes": [
                    "@timestamp",
                    "destination.address", 
                    "destination.as.number", 
                    "destination.as.organization.name",
                    "destination.domain",
                    "destination.bytes",
                    "destination.packets",
                    "destination.port",
                    "destination.ip",
                    "source.address", 
                    "source.as.number", 
                    "source.as.organization.name",
                    "source.domain",
                    "source.bytes",
                    "source.packets",
                    "source.port",
                    "source.ip",
                    "dns.answers",
                    "dns.op_code",
                    "dns.response_code",
                    "network.bytes",
                    "network.packets",
                    "network.protocol",
                    "network.transport",
                    "suricata.eve.tls.sni"
                ]
            },
            "query": {
                "match_all": {}
            },
            "size": 10000
        }))
        .send()
        .await?;

    // Get the JSON response
    let response_body = response.json::<Value>().await?;

    // Extract hits from the response body
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        // The flow info is stored in the _source field
        let flow_source = &hit["_source"];
        
        if let Some(flow_object) = flow_source.as_object(){
            for (field_name, field_value) in flow_object {
            println!("{}: {:?}\n", field_name, field_value);
            }
        }

        // Deserialize JSON into Flow struct
        if let Some(flow_object) = flow_source.as_object() {
            // Deserialize the flow object using the Flow struct
            let flow_example = create_flow(flow_object.clone());

        
            // Print the deserialized flow example
            /*println!("Destination OBJECT {:?}", flow_object.get("destination"));
            println!("{:?}", flow_example);*/
        }
    

    }


    Ok(())
}


fn create_flow(flow_object: serde_json::Map<String, Value>) -> Flow {
    println!("{:?}\n", flow_object);

    let source_parsed = utils::utils::create_source(flow_object.clone());
    let dest_parsed = utils::utils::create_destination(flow_object.clone());
    

    // Safely handle the timestamp
    let timestamp = flow_object.clone()
        .get("@timestamp")
        .and_then(|ts| ts.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "default_timestamp".to_string()); // Provide a default value

    let flow = Flow {
        timestamp,
        source: source_parsed,
        destination: dest_parsed,
        //dns_info, network_info, and tls_info here if needed
    };

    println!("{:?}", flow);
    flow
}



