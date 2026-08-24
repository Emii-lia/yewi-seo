use yew::{component, html, Html};
use yewi_seo::yew::seo;

#[seo(
  meta(
    title = "Yewi",
    description = "Component-driven UI kit for Yew",
    keywords("yew", "yewi", "yew-component", "tailwind", "scss", "component-driven", "ui", "kit"),
  ),
  open_graph(
    title = "Yewi",
    description = "Component-driven UI kit for Yew",
    url = "https://yewi.fiaro.app",
    site_name = "Yewi",
    locale = "en_US",
    image = "https://yewi.fiaro.app/og-image.png",
  ),
  link(
    canonical = "https://yewi.fiaro.app",
  ),
  twitter(
    card = "summary_large_image",
    site = "@Emii_lia",
    creator = "@Emii_lia",
    title = "Yewi",
    description = "Component-driven UI kit for Yew",
    image = "https://yewi.fiaro.app/og-image.png",
  ),
  icon(
    ( rel = "icon", href = "/favicon.ico" ),
  )
)]
#[component(Home)]
pub(crate) fn home() -> Html {
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
              <a href="https://yewi.fiaro.app" target="_blank" rel="noopener noreferrer">
                {"yewi.fiaro.app"}
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
