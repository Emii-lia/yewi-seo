use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[seo(
  meta = ( title = "Hello, World!")
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
  meta = "hello"
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
  meta( x = "hello" )
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
  meta( open_graph( title = "Open Graph title" ) )
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
  meta( twitter( title = "Twitter title" ) )
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
  meta( title( default = "Default title" ))
)]
#[component]
fn Comp_6() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}

#[seo(
  meta( title = ( default = "Default title" ))
)]
#[component]
fn Comp_7() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}

#[seo(
  meta( keywords = ( "key1", "key2", "key3" ))
)]
#[component]
fn Comp_7() -> Element {
  rsx! {
    div {
      "Hello, World!"
		}
  }
}


fn main() {}
