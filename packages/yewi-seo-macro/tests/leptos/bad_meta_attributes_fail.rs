use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo(
  meta = ( title = "Hello, World!")
)]
#[component]
fn Comp_2() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  meta = "hello"
)]
#[component]
fn Comp_3() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  meta( x = "hello" )
)]
#[component]
fn Comp_4() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  meta( open_graph( title = "Open Graph title" ) )
)]
#[component]
fn Comp_4() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  meta( twitter( title = "Twitter title" ) )
)]
#[component]
fn Comp_5() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  meta( title( default = "Default title" ))
)]
#[component]
fn Comp_6() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  meta( title = ( default = "Default title" ))
)]
#[component]
fn Comp_7() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  meta( keywords = ( "key1", "key2", "key3" ))
)]
#[component]
fn Comp_7() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}


fn main() {}
