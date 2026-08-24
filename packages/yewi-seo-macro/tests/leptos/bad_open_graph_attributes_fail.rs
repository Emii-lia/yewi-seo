use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo(
  open_graph = ( title = "Hello Yewi" )
)]
#[component]
fn Comp() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  open_graph = "Hello Yewi"
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
  open_graph( x = "Hello Yewi" )
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
  open_graph( title( default = "Default title" ) )
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
  open_graph( title = ( default = "Default title" ) )
)]
#[component]
fn Comp_5() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}


fn main() {}
