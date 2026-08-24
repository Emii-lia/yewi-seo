use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo(
  icon = (
    (href = "/favicon.ico", rel = "icon")
  )
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
  icon( href = "/favicon.ico", rel = "icon" )
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
  icon(
    (x = "icon", href = "/favicon.ico")
  )
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
  icon( icon = ( href = "/favicon.ico", rel = "icon" ) )
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
  icon(
    ( rel = "icon", sizes = "32x32" )
  )
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
