use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[seo(
  meta(
    title = "Comp",
    description = "Component-driven UI kit for Yew",
    keywords = "comp",
  )
)]
#[component]
fn comp() -> Element {
  rsx! {
    div {
			"Comp"
		}
  }
}

fn main() {}
