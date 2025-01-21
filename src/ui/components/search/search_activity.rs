use crate::{
  api::search::search,
  ui::components::common::Spinner,
  utils::{magnet_link, pretty_date, pretty_num, SearchQuery, Torrent, ICON_SIZE},
};
use human_bytes::human_bytes;
use leptos::*;
use leptos_meta::Title;
use leptos_router::{use_query_map, A};
use phosphor_leptos::{Icon, IconWeight, CALENDAR_BLANK, DATABASE, DOWNLOAD_SIMPLE};

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

  let err_msg = " Error loading.";

  let title = move || {
    if q().is_empty() {
      "Search".to_string()
    } else {
      q()
    }
  };

  view! {
    <Title text=title />
    <main class="container mx-auto mt-6 px-4">
      <Suspense fallback=move || {
        view! { <Spinner /> }
      }>
        {move || {
          search_res
            .get()
            .map(|res| match res {
              None => {
                view! { <div>{err_msg}</div> }
              }
              Some(res) => {
                if !res.torrents.is_empty() {
                  view! {
                    <div>
                      <TorrentListings torrents=res.torrents.into() />
                      {if let Some(next) = res.next {
                        view! {
                          <div class="join grid grid-cols-1">
                            <A
                              class="join-item btn btn-outline"
                              href=move || { format!("?q={}&after={}", q(), next) }
                            >
                              "Next"
                            </A>
                          </div>
                        }
                      } else {
                        view! { <div></div> }
                      }}
                    </div>
                  }
                } else {
                  view! { <div class="text-lg">"No results."</div> }
                }
              }
            })
        }}

      </Suspense>
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
  let seeders = pretty_num(torrent.seeders);
  let created = pretty_date(torrent.created_unix);

  let row_c = "inline-flex items-center";
  let icon_c = "w-4 h-4 mr-2";

  view! {
    <div class="card card-compact shadow-xl bg-base-100 mb-3">
      <div class="card-body">
        <a href=magnet class="link card-title break-all">
          {torrent.name}
        </a>
        <div class="flex justify-between text-lg">
          <div class=row_c>
            <span class=icon_c>
              <Icon icon=DOWNLOAD_SIMPLE weight=IconWeight::Regular size=ICON_SIZE />
            </span>
            <span class="text-primary">{seeders}</span>
          </div>
          <div class=row_c>
            <span class=icon_c>
              <Icon icon=DATABASE weight=IconWeight::Regular size=ICON_SIZE />
            </span>
            <span>{bytes}</span>
          </div>
          <div class=row_c>
            <span class=icon_c>
              <Icon icon=CALENDAR_BLANK weight=IconWeight::Regular size=ICON_SIZE />
            </span>
            <span>{created}</span>
          </div>
        </div>
      </div>
    </div>
  }
}
