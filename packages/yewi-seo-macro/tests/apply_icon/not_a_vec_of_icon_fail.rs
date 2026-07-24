use yew::prelude::*;
use yewi_seo::apply_icon;

#[component(Comp)]
fn comp() -> Html {
  apply_icon!(
    rel = "icon",
    href = "https://yewi.fiaro.app/icons/logo.png",
  );

  html! {
    <div>{"Comp"}</div>
  }
}

fn main() {}