use yew::prelude::*;
use yewi_seo::seo;

#[seo(x)]
#[component(Comp)]
fn comp() -> Html {
  html! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

fn main() {}