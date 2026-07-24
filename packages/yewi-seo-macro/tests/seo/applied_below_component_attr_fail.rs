use yew::prelude::*;
use yewi_seo::seo;

#[component(Comp)]
#[seo(
  meta( title = "Hello Yewi" )
)]
fn comp() -> Html {
  html! {
    <p>
      {"Hello, Yewi!"}
    </p>
  }
}

fn main() {}