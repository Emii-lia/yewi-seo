use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo]
#[component]
fn comp() -> impl IntoView {
  view! {
    <div>
      {"Comp"}
    </div>
  }
}

#[seo()]
#[component]
fn comp_2() -> impl IntoView {
  view! {
    <div>
      {"Comp2"}
    </div>
  }
}

fn main() {}
