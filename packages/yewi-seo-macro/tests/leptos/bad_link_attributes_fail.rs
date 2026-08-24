use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo(
  link = ( canonical = "https://yewi.fiaro.app" )
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
  link = "https://yewi.fiaro.app"
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
  link( x = "https://yewi.fiaro.app" )
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
  link( canonical( default = "https://yewi.fiaro.app" ) )
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
  link( canonical = ( default = "https://yewi.fiaro.app" ) )
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
  link( icon( href = "/favicon.ico", rel = "icon" ) )
)]
#[component]
fn Comp_6() -> impl IntoView {
  view! {
    <div>
      {"Hello, World!"}
    </div>
  }
}


fn main() {}
