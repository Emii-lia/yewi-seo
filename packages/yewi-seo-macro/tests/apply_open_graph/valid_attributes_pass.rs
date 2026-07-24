use yew::prelude::*;
use yewi_seo::apply_open_graph;

#[component(Comp)]
fn comp() -> Html {
  apply_open_graph!(
    title = "Comp",
    description = "Comp description",
    url = "https://yewi.fiaro.app"
  );
  
  html! { 
    <div>{ "Comp" }</div>
  }
}

fn main() {}