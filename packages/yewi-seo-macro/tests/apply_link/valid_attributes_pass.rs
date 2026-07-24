use yew::prelude::*;
use yewi_seo::apply_link;

#[component(Comp)]
fn comp() -> Html {
  apply_link!(
    canonical = "https://yewi.fiaro.app",
    manifest = "https://yewi.fiaro.app/manifest.json",
    author = "https://fiaro.dev"
  );
  
  html! {
    <div>{ "Comp" }</div>
  }
}

fn main() {}