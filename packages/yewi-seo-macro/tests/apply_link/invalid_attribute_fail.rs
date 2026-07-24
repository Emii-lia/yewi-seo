use yew::prelude::*;
use yewi_seo::apply_link;

#[component(Comp)]
fn comp() -> Html {
  apply_link!( x = "value" );
  
  html! {
    <div>{"Comp"}</div>
  }
}

fn main() {}