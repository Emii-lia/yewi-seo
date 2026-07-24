use yew::prelude::*;
use yewi_seo::apply_open_graph;

#[component(Comp)]
fn comp() -> Html {
  apply_open_graph!( x = "value" );
  
  html! {
    <div>{ "Comp" }</div>
  }
}

fn main() {}