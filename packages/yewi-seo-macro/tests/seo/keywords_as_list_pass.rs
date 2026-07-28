use yew::prelude::*;
use yewi_seo::seo;

#[seo(
  meta(
    keywords("yew", "yewi", "test")
  )
)]
#[component(Comp)]
fn comp() -> Html {
  html! {
    <div>
      { "Comp" }
    </div>
  }
}

fn main() {}