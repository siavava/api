use bytestring::ByteString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, Eq)]
pub struct PageViews {
  pub route: String,
  pub count: u64,
}

impl PartialEq for PageViews {
  fn eq(&self, other: &Self) -> bool {
    self.route == other.route
  }
}

impl PageViews {
  pub fn with(route: String) -> Self {
    Self {
      route,
      ..Self::default()
    }
  }
}

impl From<PageViews> for ByteString {
  fn from(val: PageViews) -> Self {
    serde_json::to_string(&val)
      .map(ByteString::from)
      .unwrap_or_default()
  }
}

impl From<ByteString> for PageViews {
  fn from(bytes: ByteString) -> Self {
    serde_json::from_str(bytes.as_ref()).unwrap_or_default()
  }
}
