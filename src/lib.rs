use crate::ui::components::{
  home::home_activity::HomeActivity,
  search::search_activity::SearchActivity,
};
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod api;
mod errors;
mod ui;
mod utils;

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  let (is_routing, set_is_routing) = create_signal(false);

  provide_context(is_routing);

  view! {
    <Link rel="preconnect" href="https://fonts.googleapis.com" />
    <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
    <Stylesheet href="https://fonts.googleapis.com/css2?family=Nunito:wght@400;500;600;700;800&display=swap" />
    <Stylesheet id="avocadio" href="/assets/avocadio.css" />
    <Title formatter=|text| format!("{text} — Avocadio torrents") />
    <Link rel="icon" type_="image/svg+xml" href="/assets/icon.svg" />
    <Link rel="icon" type_="image/png" sizes="32x32" href="/assets/favicon-32.png" />
    <Meta name="description" content="Avocadio torrent search" />
    <Meta name="theme-color" content="#6f7f2e" />
    <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
    // Uncomment this for testing. Dont know another good way
    // <script src="https://cdn.jsdelivr.net/npm/eruda"></script>
    // <script>eruda.init();</script>

    // adding `set_is_routing` causes the router to wait for async data to load on new pages
    <Router set_is_routing>
      // shows a progress bar while async data are loading
      // <RoutingProgress is_routing max_time=std::time::Duration::from_millis(250)/>
      <div class="app">
        <Routes>
          <Route path="" view=HomeActivity />
          <Route path="search" view=SearchActivity />
        </Routes>
      </div>
    </Router>
  }
}

// Needs to be in lib.rs AFAIK because wasm-bindgen needs us to be compiling a lib. I may be wrong.
cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();
            leptos::mount_to_body(|| {
                view! { <App/> }
            });
        }
    }
}
