use yew::{function_component, html, Html};

#[function_component]
pub(crate) fn Home() -> Html {
  html! {
    <div class="Home">
      <div class="yewi-container">
        <header class="yewi-header">
          <h1 class="yewi-title">{"Yewi"}</h1>
          <p class="yewi-tagline">{"Component-driven UI kit for Yew"}</p>
        </header>

        <main class="yewi-main">
          <p class="yewi-description">
            {"A modern UI library with Tailwind + SCSS styling, inspired by React/Next.js patterns."}
          </p>

          <div class="yewi-grid">
            <div class="yewi-card">
              <h3>{"Get Started"}</h3>
              <code>{"yewi new my-app"}</code>
            </div>
            <div class="yewi-card">
              <h3>{"Add Component"}</h3>
              <code>{"yewi add button"}</code>
            </div>
            <div class="yewi-card">
              <h3>{"Documentation"}</h3>
              <a href="https://yewi.pages.dev" target="_blank" rel="noopener noreferrer">
                {"yewi.pages.dev"}
              </a>
            </div>
            <div class="yewi-card">
              <h3>{"Source"}</h3>
              <a href="https://github.com/Emii-lia/yewi-kit" target="_blank" rel="noopener noreferrer">
                {"github.com"}
              </a>
            </div>
          </div>
        </main>
      </div>
    </div>
  }
}