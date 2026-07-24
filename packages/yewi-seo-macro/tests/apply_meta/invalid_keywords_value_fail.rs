use yew::prelude::*;
use yewi_seo::apply_meta;

#[component(Comp)]
fn comp() -> Html {
  apply_meta!(
    keywords = ("comp", "test")
  );
  
  html! {
    <div>
      {"Comp"}
    </div>
  }
}

fn main() {}