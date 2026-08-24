use dioxus::prelude::*;
use yewi_seo::dioxus::seo;

#[seo(
  twitter = ( title = "Hello Yewi" )
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
  twitter = "Hello Yewi"
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
  twitter( x = "Hello Yewi" )
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
  twitter( title( default = "Default title" ) )
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
  twitter( title = ( default = "Default title" ) )
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
  twitter( card = "invalid_card" )
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
