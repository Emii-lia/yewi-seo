use leptos::prelude::*;
use yewi_seo::leptos::seo;

#[seo(
  meta(
    title = "Leptos | Yewi",
    description = "Yewi-seo example on Leptos",
    keywords = "leptos, yewi, yewi-seo, seo",
  ),
  open_graph(
    title = "Leptos | Yewi",
    description = "Yewi-seo example on Leptos",
    url = "https://yewi.fiaro.app",
    site_name = "Yewi",
    locale = "en_US",
    image = "https://yewi.fiaro.app/images/og.png",
  ),
  link(
    canonical = "https://yewi.fiaro.app",
  ),
  twitter(
    card = "summary_large_image",
    site = "@Emii_lia",
    creator = "@Emii_lia",
    title = "Leptos | Yewi",
    description = "Yewi-seo example on Leptos",
    image = "https://yewi.fiaro.app/og-image.png",
  ),
  icon(
    ( rel = "icon", href = "/favicon.ico" )
  )
)]
#[component]
fn App() -> impl IntoView {
	let (count, set_count) = signal(0);

	view! {
				<h1>Hello, world!</h1>
        <button
            on:click=move |_| set_count.set(3)
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}

fn main() {
    leptos::mount::mount_to_body(|| view! {
			<App/>
		})
}
