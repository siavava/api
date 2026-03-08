use bytestring::ByteString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct LocationData {
  pub city: String,
  pub state: String,
}

impl LocationData {
  pub fn with(city: String, state: String) -> Self {
    Self { city, state }
  }
}

impl From<LocationData> for ByteString {
  fn from(val: LocationData) -> Self {
    serde_json::to_string(&val)
      .map(ByteString::from)
      .unwrap_or_default()
  }
}

impl From<ByteString> for LocationData {
  fn from(bytes: ByteString) -> Self {
    serde_json::from_str(bytes.as_ref()).unwrap_or_default()
  }
}
