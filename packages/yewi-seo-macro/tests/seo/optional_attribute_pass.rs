use yew::prelude::*;
#[::yewi_seo::seo(
  meta(
    title = "Comp",
    description = "Component-driven UI kit for Yew",
    keywords = "comp",
  )
)]
#[component(Comp)]
fn comp() -> ::yew::Html {
  ::yew::html! {
    <div>
      {"Comp"}
    </div>
  }
}

fn main() {}