use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[component]
#[seo(
  meta( title = "Hello Yewi" )
)]
fn Comp() -> Element {
  rsx! {
    p {
      "Hello, Yewi!"
		}
  }
}

fn main() {}
