use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo(
  twitter = ( title = "Hello Yewi" )
)]
#[component]
fn comp() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  twitter = "Hello Yewi"
)]
#[component]
fn comp_2() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  twitter( x = "Hello Yewi" )
)]
#[component]
fn comp_3() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  twitter( title( default = "Default title" ) )
)]
#[component]
fn comp_4() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  twitter( title = ( default = "Default title" ) )
)]
#[component]
fn comp_5() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  twitter( card = "invalid_card" )
)]
#[component]
fn comp_5() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}


fn main() {}
