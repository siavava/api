/// Views model
///
/// This model is used to store the views of the routes
///
use serde::{Deserialize, Serialize};

use mongodb::{bson::doc, error::Result, Database};

/// Views struct
///
/// This struct is used to store the views of the routes
///
/// It tracks the route and the count of the views
#[derive(Serialize, Deserialize)]
pub struct PageViews {
  pub route: String,
  pub count: u64,
}

// impl debug for Views
impl std::fmt::Debug for PageViews {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Views {{ route: {}, count: {} }}",
      self.route, self.count
    )
  }
}

// pub async fn get_views(db: &Database, route: &str, increment: Option<bool>) -> Result<PageViews> {
//   let increment_status = increment.unwrap_or(false);

//   let collection = db.collection::<PageViews>("views");

//   let filter = doc! { "route": route };
//   // let update = doc! { "$inc": { "count": 1 } };
//   let update = doc! { "$inc": { "count": if increment_status { 1 } else { 0 } } };

//   let res = collection
//     .find_one_and_update(filter, update)
//     .upsert(true)
//     .return_document(mongodb::options::ReturnDocument::After)
//     .await;

//   println!("res: {:?}", res);

//   match res {
//     Ok(val) => Ok(val.unwrap()),
//     Err(e) => Err(e),
//   }
// }

// #[macro_export]
// macro_rules! views {
//   ($db:expr, $route:expr, $request_route:expr) => {
//     // if request_route matches route, increment count
//     // otherwise, do not increment count
//     // add to mongodb and increment count
//     $crate::models::get_views($db, $route, Some($route == $request_route))
//       .await
//       .unwrap()
//   };
// }
