use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[seo(
  open_graph = ( title = "Hello Yewi" )
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
  open_graph = "Hello Yewi"
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
  open_graph( x = "Hello Yewi" )
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
  open_graph( title( default = "Default title" ) )
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
  open_graph( title = ( default = "Default title" ) )
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
