use yew::prelude::*;

#[::yewi_seo::seo]
#[component(Comp)]
fn comp() -> ::yew::Html {
  ::yew::html! {
    <div>
      {"Comp"}
    </div>
  }
}

#[::yewi_seo::seo()]
#[component(Comp2)]
fn comp_2() -> ::yew::Html {
  ::yew::html! {
    <div>
      {"Comp2"}
    </div>
  }
}

fn main() {}