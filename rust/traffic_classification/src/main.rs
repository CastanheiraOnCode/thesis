
extern crate serde_json;
use std::ptr::null;
use elasticsearch::{
    auth::Credentials, 
    http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder}, 
    params::Refresh, 
    Elasticsearch, 
    RenderSearchTemplate, 
    SearchParts
};
use serde::{Serialize, Deserialize};
use url::Url;
use serde_json::Value;

mod structs;
use structs::flow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Setting up ES credentials and URL
    let credentials = Credentials::Basic("elastic".into(), "elastic".into());
    let u = Url::parse("http://localhost:9200")?;
    let conn_pool = SingleNodeConnectionPool::new(u);
    let transport = TransportBuilder::new(conn_pool).auth(credentials).build()?;
    
    // Create Client
    let client = Elasticsearch::new(transport);

    // Querying Suricata Index - will have to use pagination, for now 10000 as the number of packets work
    let response = client
        .search(SearchParts::Index(&[".ds-logs-suricata.eve-default-2024.10.10-000002"]))
        .body(serde_json::json!({
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
    // Iterate over each hit
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        // Each `hit` is a JSON object, you can access its fields
        // The flow info is stored in the _source field
        let flow = &hit["_source"];
        

        
        if let Some(flow_object) = flow.as_object(){
            for (field_name, field_value) in flow_object {
            println!("{}: {:?}", field_name, field_value);
            }
        }

        //Let's populate the flow structure
        let flow_timestamp = &flow["@timestamp"];
        let flow_example = flow::Flow{
            timestamp: flow_timestamp.to_string(),
        };
        println!("{:?}\n", flow);
        print!("\n");
        println!("{:?}\n", flow_example.timestamp);
    }

    Ok(())
}

