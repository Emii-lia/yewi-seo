use yew::prelude::*;
use yewi_seo::seo;

#[seo(
  meta = ( title = "Hello, World!")
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
  meta = "hello"
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
  meta( x = "hello" )
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
  meta( open_graph( title = "Open Graph title" ) )
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
  meta( twitter( title = "Twitter title" ) )
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
  meta( title( default = "Default title" ))
)]
#[component(Comp6)]
fn comp_6() -> Html {
  html! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  meta( title = ( default = "Default title" ))
)]
#[component(Comp7)]
fn comp_7() -> Html {
  html! {
    <div>
      {"Hello, World!"}
    </div>
  }
}

#[seo(
  meta( keywords = ( "key1", "key2", "key3" ))
)]
#[component(Comp7)]
fn comp_7() -> Html {
  html! {
    <div>
      {"Hello, World!"}
    </div>
  }
}


fn main() {}