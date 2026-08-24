use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[seo(
  link = ( canonical = "https://yewi.fiaro.app" )
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
  link = "https://yewi.fiaro.app"
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
  link( x = "https://yewi.fiaro.app" )
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
  link( canonical( default = "https://yewi.fiaro.app" ) )
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
  link( canonical = ( default = "https://yewi.fiaro.app" ) )
)]
#[component]
fn Comp_5() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}

#[seo(
  link( icon( href = "/favicon.ico", rel = "icon" ) )
)]
#[component]
fn Comp_6() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}


fn main() {}
