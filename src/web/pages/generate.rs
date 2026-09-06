use yew::prelude::*;
use yew_router::prelude::*;

use crate::identity::Identity;
use crate::web::components::MeshBackground;
use crate::web::router::Route;

#[function_component(GeneratePage)]
pub fn generate_page() -> Html {
    let identity_state = use_state(|| -> Option<Identity> { None });
    let error = use_state(|| -> Option<String> { None });
    let loading = use_state(|| false);
    let nav = use_navigator().unwrap();

    {
        let identity_state = identity_state.clone();
        use_effect_with((), move |_| {
            if let Some(identity) = Identity::load_local() {
                identity_state.set(Some(identity));
            }
            || ()
        });
    }

    {
        let identity_state = identity_state.clone();
        let nav = nav.clone();
        use_effect_with((*identity_state).clone(), move |id| {
            if id.is_some() {
                nav.push(&Route::Home);
            }
            || ()
        });
    }

    let on_generate = {
        let identity_state = identity_state.clone();
        let error = error.clone();
        let loading = loading.clone();
        Callback::from(move |_: MouseEvent| {
            let identity_state = identity_state.clone();
            let error = error.clone();
            let loading = loading.clone();
            loading.set(true);
            error.set(None);

            match Identity::save_local() {
                Ok(identity) => {
                    identity_state.set(Some(identity));
                }
                Err(e) => {
                    error.set(Some(format!("{e}")));
                    loading.set(false);
                }
            }
        })
    };

    if (*identity_state).is_some() {
        return html! {};
    }

    html! {
        <>
            <MeshBackground />
            <div class="relative z-10 min-h-screen flex items-center justify-center px-6">
                <div class="backdrop-blur-xl bg-white/5 border border-white/[0.06] rounded-3xl shadow-[0_0_0_1px_rgba(255,255,255,0.03)_inset,0_32px_64px_-12px_rgba(0,0,0,0.5)] w-full max-w-md">
                    <div class="p-10">
                        <div class="fade-up text-center mb-10">
                            <div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl mb-6 bg-yellow-500/20 border border-yellow-500/15">
                                <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
                                    <path d="M2 17l10 5 10-5"></path>
                                    <path d="M2 12l10 5 10-5"></path>
                                </svg>
                            </div>
                            <h1 class="title-gradient text-3xl font-bold tracking-tight mb-3">
                                { "create your identity" }
                            </h1>
                            <p class="text-zinc-500 text-sm leading-relaxed max-w-xs mx-auto">
                                { "generate a cryptographic keypair to start using murm. your private key stays in this browser." }
                            </p>
                        </div>

                        if let Some(ref e) = *error {
                            <div class="fade-up flex items-center gap-3 p-3 px-4 bg-red-500/10 border border-red-500/20 rounded-xl text-red-300 text-[13px] mb-6">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="shrink-0 opacity-70">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <line x1="15" y1="9" x2="9" y2="15"></line>
                                    <line x1="9" y1="9" x2="15" y2="15"></line>
                                </svg>
                                <span>{ e.as_str() }</span>
                            </div>
                        }

                        <div class="fade-up fade-up-delay-2">
                            <button
                                onclick={on_generate}
                                disabled={*loading}
                                class="btn-primary w-full text-base"
                            >
                                if *loading {
                                    <div class="spinner"></div>
                                    <span>{ "generating..." }</span>
                                } else {
                                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
                                        <path d="M2 17l10 5 10-5"></path>
                                        <path d="M2 12l10 5 10-5"></path>
                                    </svg>
                                    <span>{ "generate keypair" }</span>
                                }
                            </button>
                        </div>
                    </div>

                    <div class="mx-4 mb-4 px-5 py-4 bg-white/[0.02] border border-white/[0.04] rounded-2xl">
                        <div class="flex items-start gap-3">
                            <div class="shrink-0 mt-0.5">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-70">
                                    <circle cx="12" cy="12" r="10"></circle>
                                    <line x1="12" y1="16" x2="12" y2="12"></line>
                                    <line x1="12" y1="8" x2="12.01" y2="8"></line>
                                </svg>
                            </div>
                            <p class="text-xs leading-relaxed text-zinc-500">
                                { "your private key is stored in " }
                                <code class="px-1.5 py-0.5 rounded text-xs bg-white/[0.06] text-zinc-400">{ "localStorage" }</code>
                                { " and never leaves this device. clearing browser data will remove it." }
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}
