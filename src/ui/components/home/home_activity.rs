use crate::ui::components::common::nav::{Logo, SearchBox};
use leptos::*;
use leptos_meta::Title;

/// Google-style home: the logo and the search box, nothing else.
#[component]
pub fn HomeActivity() -> impl IntoView {
  view! {
    <Title text="Search" />
    <main class="home">
      <div class="home-inner">
        <Logo big=true />
        <SearchBox big=true />
      </div>
    </main>
  }
}
