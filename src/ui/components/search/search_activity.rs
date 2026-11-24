use crate::{
  api::search::search,
  ui::components::common::{
    nav::{Logo, SearchBox},
    Spinner,
  },
  utils::{magnet_link, pretty_date, pretty_num, SearchQuery, Torrent, ICON_SIZE},
};
use human_bytes::human_bytes;
use leptos::*;
use leptos_meta::Title;
use leptos_router::{use_query_map, A};
use phosphor_leptos::{Icon, IconWeight, ARROW_UP, CALENDAR_BLANK, DATABASE, MAGNET};

#[component]
pub fn SearchActivity() -> impl IntoView {
  let query = use_query_map();
  let q = move || query.with(|q| q.get("q").cloned()).unwrap_or_default();
  let after = move || query.with(|q| q.get("after").and_then(|after| after.parse::<usize>().ok()));

  let search_res = create_resource(
    move || (q(), after()),
    move |(q, after)| async move {
      let form = SearchQuery { q, after };
      search(&form).await.ok()
    },
  );

  let title = move || {
    if q().is_empty() {
      "Search".to_string()
    } else {
      q()
    }
  };

  view! {
    <Title text=title />
    <header class="results-header page-grid">
      <Logo />
      <SearchBox value=Signal::derive(q) />
    </header>
    <main class="results page-grid">
      <div class="results-list">
        <Suspense fallback=move || {
          view! {
            <div class="status">
              <Spinner />
            </div>
          }
        }>
          {move || {
            search_res
              .get()
              .map(|res| match res {
                None => {
                  view! { <div class="status error">"Couldn't reach the search server."</div> }
                    .into_view()
                }
                Some(res) if res.torrents.is_empty() => {
                  view! { <div class="status">"No torrents found. Try fewer or different words."</div> }
                    .into_view()
                }
                Some(res) => {
                  let next = res.next;
                  view! {
                    <TorrentListings torrents=res.torrents.into() />
                    {next
                      .map(|next| {
                        view! {
                          <div class="more">
                            <A class="btn" href=move || format!("/search?q={}&after={}", q(), next)>
                              "More results"
                            </A>
                          </div>
                        }
                      })}
                  }
                    .into_view()
                }
              })
          }}
        </Suspense>
      </div>
    </main>
  }
}

#[component]
fn TorrentListings(torrents: MaybeSignal<Vec<Torrent>>) -> impl IntoView {
  view! {
    <For each=move || torrents.get() key=|t| t.infohash.clone() let:child>
      <TorrentListing torrent=child />
    </For>
  }
}

#[component]
fn TorrentListing(torrent: Torrent) -> impl IntoView {
  let magnet = magnet_link(&torrent);
  let bytes = human_bytes(torrent.size_bytes as f64);
  let seeders = format!(
    "{} {}",
    pretty_num(torrent.seeders),
    if torrent.seeders == 1 { "seeder" } else { "seeders" }
  );
  let created = pretty_date(torrent.created_unix);
  let short_hash = torrent.infohash.chars().take(12).collect::<String>();

  view! {
    <article class="result">
      <div class="result-source">
        <span class="icon sm">
          <Icon icon=MAGNET weight=IconWeight::Regular size=ICON_SIZE />
        </span>
        <span>"magnet"</span>
        <code>{short_hash}</code>
      </div>
      <a href=magnet class="result-title">
        {torrent.name}
      </a>
      <div class="result-meta">
        <span class="meta-item seeds">
          <span class="icon sm">
            <Icon icon=ARROW_UP weight=IconWeight::Bold size=ICON_SIZE />
          </span>
          {seeders}
        </span>
        <span class="meta-item">
          <span class="icon sm">
            <Icon icon=DATABASE weight=IconWeight::Regular size=ICON_SIZE />
          </span>
          {bytes}
        </span>
        <span class="meta-item">
          <span class="icon sm">
            <Icon icon=CALENDAR_BLANK weight=IconWeight::Regular size=ICON_SIZE />
          </span>
          {created}
        </span>
      </div>
    </article>
  }
}
