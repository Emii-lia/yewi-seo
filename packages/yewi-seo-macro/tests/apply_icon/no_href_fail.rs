use yew::prelude::*;
use yewi_seo::apply_icon;

#[component(Comp)]
fn comp() -> Html {
  apply_icon!(rel = "icon" );
  
  html! {
    <div>{"Comp"}</div>
  }
}

fn main() {}