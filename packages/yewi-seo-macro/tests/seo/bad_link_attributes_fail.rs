use yew::prelude::*;
use yewi_seo::seo;

#[seo(
  link = ( canonical = "https://yewi.fiaro.app" )
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
  link = "https://yewi.fiaro.app"
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
  link( x = "https://yewi.fiaro.app" )
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
  link( canonical( default = "https://yewi.fiaro.app" ) )
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
  link( canonical = ( default = "https://yewi.fiaro.app" ) )
)]
#[component(Comp5)]
fn comp_5() -> Html {
  html! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  link( icon( href = "/favicon.ico", rel = "icon" ) )
)]
#[component(Comp6)]
fn comp_6() -> Html {
  html! {
    <div>
      {"Hello, World!"}
    </div>
  }
}


fn main() {}
