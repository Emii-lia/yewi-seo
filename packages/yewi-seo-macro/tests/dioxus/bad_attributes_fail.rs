use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[seo(x)]
#[component]
fn Comp() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}

fn main() {}
