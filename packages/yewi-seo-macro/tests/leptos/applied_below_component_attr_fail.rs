use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[component]
#[seo(
  meta( title = "Hello Yewi" )
)]
fn Comp() -> impl IntoView {
  view! {
    <p>
      {"Hello, Yewi!"}
    </p>
  }
}

fn main() {}
