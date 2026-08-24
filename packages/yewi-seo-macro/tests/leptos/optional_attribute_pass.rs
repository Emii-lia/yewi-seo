use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo(
  meta(
    title = "Comp",
    description = "Component-driven UI kit for Yew",
    keywords = "comp",
  )
)]
#[component]
fn comp() -> impl IntoView {
  ::leptos::view! {
    <div>
      {"Comp"}
    </div>
  }
}

fn main() {}
