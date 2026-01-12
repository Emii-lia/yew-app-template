use yew::{html, Html};
use yew_router::Routable;
use crate::app::{Home};
use crate::app::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
  #[at("/")]
  Home,
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn switch(route: AppRoute) -> Html {
  match route {
    AppRoute::Home => html! {<Home/>},
    AppRoute::NotFound => html! {<NotFound/>},
  }
}