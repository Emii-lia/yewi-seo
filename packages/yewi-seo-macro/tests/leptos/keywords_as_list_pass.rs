use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo(
  meta(
    keywords("yew", "yewi", "test")
  )
)]
#[component]
fn comp() -> impl IntoView {
  view! {
    <div>
      { "Comp" }
    </div>
  }
}

fn main() {}
