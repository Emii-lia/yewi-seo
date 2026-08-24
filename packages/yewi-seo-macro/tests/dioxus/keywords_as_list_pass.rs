use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[seo(
  meta(
    keywords("yew", "yewi", "test")
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
