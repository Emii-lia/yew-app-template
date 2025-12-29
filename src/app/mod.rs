mod routes;
mod page;
mod not_found;

pub(crate) use routes::*;
pub(crate) use page::*;

use yew::{function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Switch};
use crate::components::{Modal, ModalProvider};

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <ModalProvider>
        <div class="app">
          <Modal/>
          <Switch<AppRoute> render={switch}/>
        </div>
      </ModalProvider>
    </BrowserRouter>
  }
}