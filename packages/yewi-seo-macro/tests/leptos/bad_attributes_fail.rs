use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo(x)]
#[component]
fn Comp() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

fn main() {}
