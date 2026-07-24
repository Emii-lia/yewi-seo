use yew::prelude::*;
use yewi_seo::seo;

#[seo(
  open_graph = ( title = "Hello Yewi" )
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
  open_graph = "Hello Yewi"
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
  open_graph( x = "Hello Yewi" )
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
  open_graph( title( default = "Default title" ) )
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
  open_graph( title = ( default = "Default title" ) )
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
