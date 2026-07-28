use yew::{component, html, Html};
use yewi_seo::{apply_icon, apply_link, apply_meta, apply_open_graph, apply_twitter_card};

#[component(About)]
pub(crate) fn about() -> Html {
  apply_icon!(rel = "icon", href = "/favicon.ico");
  apply_twitter_card!(
    card = "summary_large_image",
    site = "@Emii_lia",
    creator = "@Emii_lia",
    title = "About - Yewi",
    description = "About page of Yewi",
    image = "https://fiaro.app/og-image.png",
  );
  apply_link!(
    canonical = "https://fiaro.app/about",
  );
  apply_meta!(
    title = "About - Yewi",
    description = "About page of Yewi",
    keywords("yew", "yewi", "about", "page"),
  );
  apply_open_graph!(
    title = "About - Yewi",
    description = "About page of Yewi",
    url = "https://yewi.fiaro.app/about",
    site_name = "Yewi",
    locale = "en_US",
    image = "https://yewi.fiaro.app/og-image.png",
  );

  html! {
    <div class="About">
      <h1>{"About"}</h1>
      <p>{"This is the about page."}</p>
    </div>
  }
}