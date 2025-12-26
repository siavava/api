mod location;
mod quotes;
mod views;

// function to inject routes
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  quotes::register(cfg);
  views::register(cfg);
  location::register(cfg);
}
