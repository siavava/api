mod location;
mod views;

// function to inject routes
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  views::register(cfg);
  location::register(cfg);
}
