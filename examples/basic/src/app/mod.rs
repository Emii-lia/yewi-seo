mod routes;
mod page;
mod not_found;
pub mod about;

pub(crate) use routes::*;
pub(crate) use page::*;

use yew::{component, html, Html};
use yew_router::prelude::{BrowserRouter, Switch};

#[component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <div class="app">
        <Switch<AppRoute> render={switch}/>
      </div>
    </BrowserRouter>
  }
}