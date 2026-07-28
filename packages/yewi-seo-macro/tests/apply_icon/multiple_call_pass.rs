use yew::prelude::*;
use yewi_seo::apply_icon;

#[component(Comp)]
fn comp() -> Html {
  apply_icon!(rel = "icon", href = "/favicon.ico", sizes = "32x32" );
  apply_icon!(rel = "icon", href = "/favicon.icon", sizes = "18x18" );

  html! {
    <div>{ "Comp" }</div>
  }
}

fn main() {}