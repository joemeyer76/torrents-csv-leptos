use crate::{ui::components::common::Spinner, utils::ICON_SIZE};
use leptos::*;
use leptos_router::*;
use phosphor_leptos::{Icon, IconWeight, MAGNIFYING_GLASS};

/// The Avocadio tile and wordmark, with "torrents" in pit brown. Links home.
#[component]
pub fn Logo(#[prop(optional)] big: bool) -> impl IntoView {
  let class = if big { "brand big" } else { "brand" };
  view! {
    <A href="/" class=class>
      <img src="/icon.svg" alt="" />
      <span>"Avocadio"</span>
      <span class="tag">"torrents"</span>
    </A>
  }
}

/// Pill search box. `big` is the home page version, with a button under it.
#[component]
pub fn SearchBox(
  #[prop(optional)] big: bool,
  #[prop(optional, into)] value: Option<Signal<String>>,
) -> impl IntoView {
  let is_routing = use_context::<ReadSignal<bool>>().expect("Missing is_routing");
  let value = value.unwrap_or_else(|| Signal::derive(String::new));
  let class = if big { "search-form big" } else { "search-form" };

  view! {
    <Form method="GET" action="/search" class=class>
      <div class="search-box">
        <span class="icon">
          <Icon icon=MAGNIFYING_GLASS weight=IconWeight::Bold size=ICON_SIZE />
        </span>
        <input
          type="search"
          name="q"
          placeholder="Search torrents"
          aria-label="Search torrents"
          autocomplete="off"
          autofocus=big
          value=move || value.get()
          prop:value=move || value.get()
        />
        <Show when=move || is_routing.get()>
          <Spinner />
        </Show>
      </div>
      <Show when=move || big>
        <div class="search-actions">
          <button type="submit" class="btn primary">
            "Search torrents"
          </button>
        </div>
      </Show>
    </Form>
  }
}
