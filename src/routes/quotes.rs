//! # Quotes Route
//!
//! REST and HTML endpoints for quote display and retrieval.
//!
//! # Endpoints
//!
//! | Method | Path           | Response  | Description                               |
//! |--------|----------------|-----------|-------------------------------------------|
//! | GET    | `/`            | HTML      | Self-contained page that cycles through quotes client-side. |
//! | GET    | `/one`         | JSON      | Minimal health-check stub.                |
//! | GET    | `/quotes/`     | JSON      | All quotes as a JSON array.               |
//! | GET    | `/quotes/test` | Text      | Plain-text health-check.                  |

use actix_web::{
  HttpResponse, Responder, get,
  web::{Html, scope},
};
use serde::{Deserialize, Serialize};
use tracing::info;

/// Registers quote endpoints.
///
/// # Arguments
///
/// * `cfg` — The Actix-Web service config to register routes on.
///
/// # Registered Routes
///
/// * `GET /` — the quotidian HTML page (served at the root).
/// * `GET /one` — health-check stub.
/// * `GET /quotes/` — JSON array of all quotes.
/// * `GET /quotes/test` — plain-text test route.
pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
  // cfg.service(raw_quotes);
  // cfg.service(quotidian);
  // cfg.service(test);

  cfg.service(quotidian).service(test_quotes).service(
    scope("/quotes")
      .service(get_quotes)
      // .service(quotidian)
      .service(test_quotes)
      .service(test),
  );
}

/// Wrapper for deserializing the static `quotes.json` file.
#[derive(Deserialize, Debug)]
struct QuoteData {
  /// List of all quotes loaded from the JSON file.
  quotes: Vec<Quote>,
}

/// A single quote with its text and attribution.
#[derive(Serialize, Deserialize, Debug)]
struct Quote {
  /// The quote body (may contain HTML markup).
  text: String,
  /// Who said or wrote the quote.
  author: String,
}

/// `GET /quotes/test` — plain-text health-check endpoint.
///
/// # Returns
///
/// `200 OK` with body `"Test route"` as plain text.
#[get("/test")]
async fn test() -> impl Responder {
  HttpResponse::Ok().body("Test route")
}

/// `GET /one` — minimal JSON health-check endpoint.
///
/// # Returns
///
/// `200 OK` with body `"Ok"` as JSON.
#[get("/one")]
async fn test_quotes() -> HttpResponse {
  HttpResponse::Ok().json("Ok")
}

/// `GET /quotes/` — returns all quotes from `quotes.json` as a JSON array.
///
/// # Returns
///
/// `200 OK` with a JSON array of quote objects.
///
/// # Example Response
///
/// ```json
/// [
///   { "text": "To be or not to be...", "author": "Shakespeare" },
///   { "text": "I think, therefore I am.", "author": "Descartes" }
/// ]
/// ```
#[get("/")]
async fn get_quotes() -> HttpResponse {
  info!("loading quotes");
  let quote_data: QuoteData = serde_json::from_str(include_str!("../static/quotes.json"))
    .expect("JSON was not well-formatted");

  info!("Quotes loaded successfully");

  HttpResponse::Ok().json(quote_data.quotes)
}

/// `GET /` — serves a self-contained HTML page that cycles through all
/// quotes client-side.
///
/// # Behavior
///
/// The full quote list is embedded as inline JSON so no additional API
/// calls are needed. Quotes advance every 3 seconds via client-side
/// JavaScript.
///
/// # Returns
///
/// `200 OK` with an HTML page containing inline JavaScript that cycles
/// through all quotes.
#[get("/")]
async fn quotidian() -> Html {
  // Html::new(include_str!("../static/quotes.html"))
  let raw_data = include_str!("../static/quotes.json");
  let json: serde_json::Value =
    serde_json::from_str(raw_data).expect("JSON was not well-formatted");

  let quotes_data = json["quotes"].as_array().expect("Quotes not found in JSON");

  if quotes_data.is_empty() {
    return Html::new("No quotes available");
  }

  // Serialize all quotes so we can embed them into the returned HTML and let client-side JS cycle through them.
  let quotes_json = serde_json::to_string(quotes_data).expect("Failed to serialize quotes");

  // Each request returns a web page which will itself cycle through ALL the quotes (client-side).
  Html::new(format!(
    r#"
        <body
          style="
            margin: 0; padding: 0; width: 100svw; height: 100svh;
            background-color: #111110; color: #d9d8e1e6;
            display: flex;
            justify-content: center;
            align-items: center;
            font-family: system-ui, sans-serif;
          ">

          <div style="
            display: flex;
            flex-direction: column;
            gap: 2rem;
            align-items: flex-end;
            justify-content: center;
            width: min(90%, 44ch);
            min-height: 300px;
            padding: 2rem;
            line-height: 1.5;
            position: relative;
          ">
            <div id="quote-area" style="width: 100%; text-align: left;" />
            </div>
            <div id="author-area" style="width: 100%; text-align: right; display: block; display: none;">
            </div>
            <!--
            <div style="width:100%; text-align:center; margin-top:1rem; font-size:0.85rem; color: #9b99a6; position: absolute; bottom: 0;">
              find me <a href="https://github.com/your_username">here</a>.
            </div>
            -->
            <div class="container">
              <p class="text">
                <a href="https://amittai.space" class="link">
                  <span class="background"></span>
                  created by&nbsp;<span class="author">amittai</span>
                </a>
              </p>
            </div>
          </div>

          <script>
            const quotes = {quotes_json};
            const QUOTE_INTERVAL_MS = 3000;
            const quoteArea = document.getElementById('quote-area');
            const authorArea = document.getElementById('author-area');

            function renderQuote(q) {{
              quoteArea.innerHTML = q.text || '';
              authorArea.textContent = q.author ? ('— ' + q.author) : '';
            }}

            // Cycle through all quotes sequentially. Each request returns HTML that will
            // iterate over every quote in order, pausing QUOTE_INTERVAL_MS between transitions.
            function startCycle() {{
              let i = 0;
              if (!quotes || quotes.length === 0) {{
                quoteArea.textContent = 'No quotes available';
                return;
              }}

              // render first immediately
              renderQuote(quotes[0]);

              // Start a timer that advances one quote every QUOTE_INTERVAL_MS
              setInterval(() => {{
                i = (i + 1) % quotes.length;
                renderQuote(quotes[i]);
              }}, QUOTE_INTERVAL_MS);
            }}

            // When the page loads, start the cycle
            if (document.readyState === 'loading') {{
              document.addEventListener('DOMContentLoaded', startCycle);
            }} else {{
              startCycle();
            }}
          </script>
          <style>
          h1:has(~ .container > .text > .link:hover) {{
            animation: focus 0.9s forwards;
          }}

          h1:not(:has(~ .container > .text > .link:hover)) {{
            animation: blur 0.9s forwards;
          }}

          /* button */
          .container {{
            display: flex;
            flex: 1;
            align-items: flex-end;
            justify-content: center;
            padding-bottom: 16px;
            position: fixed;
            bottom: 0;
            left: 50%;
            transform: translateX(-50%);
          }}

          /* Responsive design for larger screens */
          @media (min-width: 1024px) {{
            .container {{
              justify-content: flex-start;
              padding-bottom: 24px;
            }}
          }}

          .text {{
            display: flex;
            align-items: baseline;
            gap: 8px; /* equivalent to gap-x-2 */
            font-size: 0.8125rem;
            color: #6b7280; /* equivalent to text-gray-500 */
          }}

          .link {{
            display: flex;
            align-items: center;
            border-radius: 0.5rem;
            padding: 2px 8px;
            font-size: 13px;
            font-weight: 500;
            color: rgba(255, 255, 255, 0.3);
            position: relative;
            text-decoration: none;
            transition: color 0.2s;
            position: relative;
          }}

          .link:hover {{
            color: #0ea5e9;
          }}

          .background {{
            position: absolute;
            inset: 0;
            z-index: -10;
            transform: scale(0.75);
            border-radius: 0.5rem;
            background: rgba(255, 255, 255, 0.05);
            opacity: 0;
            transition: all 0.2s;
            left: -10%;
            width: 120%;
          }}

          .link:hover .background {{
            transform: scale(1);
            opacity: 1;
          }}

          .author {{
            color: white;
          }}
          </style>
        </body>
        "#
  ))
}
