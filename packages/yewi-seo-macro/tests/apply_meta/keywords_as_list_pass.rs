use yew::prelude::*;
use yewi_seo::apply_meta;

#[component(Comp)]
fn comp() -> Html {
  apply_meta!(
    keywords("yew", "yewi", "info")
  );
  html! {
    <div>
      {"Comp"}
    </div>
  }
}

fn main() {}