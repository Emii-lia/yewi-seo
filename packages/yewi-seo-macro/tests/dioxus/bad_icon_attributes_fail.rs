use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[seo(
  icon = (
    (href = "/favicon.ico", rel = "icon")
  )
)]
#[component]
fn Comp() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}
#[seo(
  icon( href = "/favicon.ico", rel = "icon" )
)]
#[component]
fn Comp_2() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}

#[seo(
  icon(
    (x = "icon", href = "/favicon.ico")
  )
)]
#[component]
fn Comp_3() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}

#[seo(
  icon( icon = ( href = "/favicon.ico", rel = "icon" ) )
)]
#[component]
fn Comp_4() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}

#[seo(
  icon(
    ( rel = "icon", sizes = "32x32" )
  )
)]
#[component]
fn Comp_5() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}

fn main() {}
