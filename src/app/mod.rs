mod routes;
mod page;
mod not_found;

pub(crate) use routes::*;
pub(crate) use page::*;

use yew::{function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <div class="app">
        <Switch<AppRoute> render={switch}/>
      </div>
    </BrowserRouter>
  }
}