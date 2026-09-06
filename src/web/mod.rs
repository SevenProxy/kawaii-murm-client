#[cfg(target_arch = "wasm32")]
pub mod components;

#[cfg(target_arch = "wasm32")]
pub mod pages;

#[cfg(target_arch = "wasm32")]
mod router;

#[cfg(target_arch = "wasm32")]
use router::App;

#[cfg(not(target_arch = "wasm32"))]
use yew::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="min-h-screen bg-gray-900 text-white flex items-center justify-center">
            <div class="text-center">
                <h1 class="text-4xl font-bold mb-4">{ "murm" }</h1>
                <p class="text-gray-400">{ "Hello, Yew + TailwindCSS!" }</p>
            </div>
        </div>
    }
}

pub fn server() {
    yew::Renderer::<App>::new().render();
}
