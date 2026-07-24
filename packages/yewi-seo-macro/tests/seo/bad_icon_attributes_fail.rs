use yew::prelude::*;
use yewi_seo::seo;

#[seo(
  icon = (
    (href = "/favicon.ico", rel = "icon")
  )
)]
#[component(Comp)]
fn comp() -> Html {
  html! {
    <div>
      {"Hello, World!"}
    </div>
  }
}
#[seo(
  icon( href = "/favicon.ico", rel = "icon" )
)]
#[component(Comp2)]
fn comp_2() -> Html {
  html! {
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
#[component(Comp3)]
fn comp_3() -> Html {
  html! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  icon( icon = ( href = "/favicon.ico", rel = "icon" ) )
)]
#[component(Comp4)]
fn comp_4() -> Html {
  html! {
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
#[component(Comp5)]
fn comp_5() -> Html {
  html! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

fn main() {}
