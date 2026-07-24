use yew::prelude::*;
use yewi_seo::apply_twitter_card;

#[component(Comp)]
fn comp() -> Html {
  apply_twitter_card!();

  html! {
    <div>{"Comp"}</div>
  }
}

fn main() {}