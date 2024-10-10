use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Flow {
    pub timestamp: String, // example field
    // Add more fields as necessary
}
