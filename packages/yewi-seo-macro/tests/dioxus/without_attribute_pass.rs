use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[seo]
#[component]
fn Comp() -> Element {
  rsx! {
    div {
			"Comp"
		}
  }
}

#[seo()]
#[component]
fn Comp_2() -> Element {
  rsx! {
    div {
			"Comp2"
		}
  }
}

fn main() {}
