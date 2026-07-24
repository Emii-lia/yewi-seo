use yew::prelude::*;
use yewi_seo::apply_twitter_card;

#[component(Comp)]
fn comp() -> Html {
  apply_twitter_card!(
    card = "summary_large_image",
    title = "Comp",
    description = "Comp description"
  );
  
  html! {
    <div>{ "Comp" }</div>
  }
}

fn main() {}