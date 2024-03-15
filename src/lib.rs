pub mod v4;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(tag = "version")]
pub enum Webhook {
    /// Webhook payload based on Prometheus docs
    #[serde(rename = "4")]
    Webhook4(v4::Webhook),
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_sample_v4_payload() {
        // Sample taken from https://www.puppeteers.net/blog/testing-alertmanager-webhooks-with-curl/
        let json_string = include_str!("../assets/sample-v4-payload.json");
        let json_value: Result<super::Webhook, _> = serde_json::from_str(json_string);
        assert!(json_value.is_ok());
    }
}
