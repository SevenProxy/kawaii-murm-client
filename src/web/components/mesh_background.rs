use yew::prelude::*;

#[function_component(MeshBackground)]
pub fn mesh_background() -> Html {
    html! {
        <div class="fixed inset-0 z-0 overflow-hidden">
            <div class="mesh-orb mesh-orb-1"></div>
            <div class="mesh-orb mesh-orb-2"></div>
            <div class="mesh-orb mesh-orb-3"></div>
        </div>
    }
}
