/// Views model
///
/// This model is used to store the views of the routes
///
/// # Example
///
/// ```
/// use wserver::models::Views;
///
/// let views = Views {
///  route: "/api".into(),
///  count: 1,
/// };
///
/// println!("views: {:?}", views);
/// ```
///
use serde::{Deserialize, Serialize};

use mongodb::{bson::doc, error::Result, Database};

/// Views struct
///
/// This struct is used to store the views of the routes
///
/// It tracks the route and the count of the views
#[derive(Serialize, Deserialize)]
pub struct Views {
  pub route: String,
  pub count: u64,
}

// impl debug for Views
impl std::fmt::Debug for Views {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Views {{ route: {}, count: {} }}",
      self.route, self.count
    )
  }
}

// ///
// /// Macro to create a new Views instance
// ///
// /// # Example
// ///
// /// ```
// /// use wserver::views;
// ///
// /// let views = views!["/api", 1];
// ///
// /// println!("views: {:?}", views);
// /// ```
// ///
// #[macro_export]
// macro_rules! views {
//   ($route:expr, $count:expr) => {
//     Views {
//       route: $route.into(),
//       count: $count,
//     }
//   };
// }

pub async fn increment_views(db: &Database, route: &str) -> Result<Views> {
  let collection = db.collection::<Views>("views");

  let filter = doc! { "route": route };
  let update = doc! { "$inc": { "count": 1 } };

  let res = collection
    .find_one_and_update(filter, update)
    .upsert(true)
    .return_document(mongodb::options::ReturnDocument::After)
    .await;

  println!("res: {:?}", res);

  match res {
    Ok(val) => Ok(val.unwrap()),
    Err(e) => Err(e),
  }
}

#[macro_export]
macro_rules! views {
  ($db:expr, $route:expr) => {
    // add to mongodb and increment count
    $crate::models::increment_views($db, $route).await.unwrap()
  };
}
