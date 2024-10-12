use std::net::IpAddr;
use serde_json::Value;
use super::structs::{Destination, Source};



pub fn create_source(flow_object: serde_json::Map<String, Value>) -> Source{

    let source_flow = flow_object.get("source").and_then(|v| v.as_object());

    let source_obj = if let Some(source) = source_flow {
        Source {
            address: source
                .get("address")
                .and_then(|f| f.as_str())
                .map(|s| s.to_string()),

            as_number: source
                .get("as")
                .and_then(|a| a.as_object())
                .and_then(|as_obj| as_obj.get("number"))
                .and_then(|n| n.as_i64()),
                
            as_organization_name: source
                                  .get("as")
                                  .and_then(|a| a.as_object())
                                  .and_then(|as_obj| as_obj.get("organization"))
                                  .and_then(|o| o.as_object())
                                  .and_then(|g| g.get("name"))
                                  .and_then(|f |f.as_str())
                                  .map(|n| n.to_string()),

            bytes: source
                   .get("bytes")
                   .and_then(|b| b.as_i64()),

            packets: source
                   .get("packets")
                   .and_then(|p| p.as_i64()),

            port: source
                  .get("port")
                  .and_then(|p| p.as_i64()),
            
            ip: source
                .get("ip")
                .and_then(|i| i.as_str())
                .and_then(|ip_str| ip_str.parse::<IpAddr>().ok()),
        }
    } else {
        Source {
            address: None,
            as_number: None,
            as_organization_name: None,
            bytes: None,
            packets: None,
            port: None,
            ip: None,
        }
    };

    source_obj

}

pub fn create_destination(flow_object: serde_json::Map<String, Value>) -> Destination{

    let dest_flow = flow_object.get("destination").and_then(|v| v.as_object());

    let dest_obj = if let Some(dest) = dest_flow {
        Destination {
            address: dest
                .get("address")
                .and_then(|f| f.as_str())
                .map(|s| s.to_string()),

            as_number: dest
                .get("as")
                .and_then(|a| a.as_object())
                .and_then(|as_obj| as_obj.get("number"))
                .and_then(|n| n.as_i64()),
                
            as_organization_name: dest
                                  .get("as")
                                  .and_then(|a| a.as_object())
                                  .and_then(|as_obj| as_obj.get("organization"))
                                  .and_then(|o| o.as_object())
                                  .and_then(|g| g.get("name"))
                                  .and_then(|f |f.as_str())
                                  .map(|n| n.to_string()),
            
            domain: dest
                    .get("domain")
                    .and_then(|d | d.as_str())
                    .map(|s| s.to_string()),


            bytes: dest
                   .get("bytes")
                   .and_then(|b| b.as_i64()),

            packets: dest
                   .get("packets")
                   .and_then(|p| p.as_i64()),

            port: dest
                  .get("port")
                  .and_then(|p| p.as_i64()),
            
            ip: dest
                .get("ip")
                .and_then(|i| i.as_str())
                .and_then(|ip_str| ip_str.parse::<IpAddr>().ok()),
        }
    } else {
        Destination {
            address: None,
            as_number: None,
            as_organization_name: None,
            domain: None, 
            bytes: None,
            packets: None,
            port: None,
            ip: None,
        }
    };

    dest_obj

}