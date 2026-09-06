use yew::prelude::*;
use yew_router::prelude::*;

use crate::identity::Identity;
use crate::web::components::MeshBackground;
use crate::web::router::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let pubkey = use_state(|| -> Option<String> {
        Identity::load_local().map(|id| id.pubkey_hex())
    });

    let nav = use_navigator().unwrap();

    {
        let pubkey = pubkey.clone();
        let nav = nav.clone();
        use_effect_with((), move |_| {
            if (*pubkey).is_none() {
                nav.push(&Route::Generate);
            }
            || ()
        });
    }

    let pubkey_display = (*pubkey).clone().unwrap_or_default();
    let short_pk = if pubkey_display.len() > 16 {
        format!("{}...{}", &pubkey_display[..8], &pubkey_display[pubkey_display.len()-8..])
    } else {
        pubkey_display.clone()
    };

    html! {
        <>
            <MeshBackground />
            <div class="relative z-10 min-h-screen flex items-center justify-center px-6">
                <div class="backdrop-blur-xl bg-white/5 border border-white/[0.06] rounded-3xl shadow-[0_0_0_1px_rgba(255,255,255,0.03)_inset,0_32px_64px_-12px_rgba(0,0,0,0.5)] p-10 w-full max-w-md text-center">
                    <div class="fade-up flex justify-center mb-6">
                        <div class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-gradient-to-br from-green-500 to-green-600 shadow-[0_0_30px_rgba(34,197,94,0.4)]" style="animation: success-pop 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);">
                            <svg width="24" height="24" fill="none" stroke="white" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                <polyline points="20 6 9 17 4 12"></polyline>
                            </svg>
                        </div>
                    </div>
                    <h1 class="fade-up fade-up-delay-1 title-gradient text-3xl font-bold tracking-tight mb-2">
                        { "Welcome Back" }
                    </h1>
                    <p class="fade-up fade-up-delay-2 text-zinc-500 text-sm mb-8">
                        { "your identity is loaded and ready" }
                    </p>
                    <div class="fade-up fade-up-delay-3 mb-8">
                        <div class="inline-flex items-center gap-2 py-2.5 px-4 bg-white/[0.04] border border-white/[0.06] rounded-xl font-mono text-xs text-zinc-400 max-w-full overflow-hidden mx-auto">
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="opacity-50 shrink-0">
                                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
                                <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
                            </svg>
                            <span class="truncate">{ short_pk }</span>
                        </div>
                    </div>
                    <div class="fade-up fade-up-delay-4 flex justify-center">
                        <Link<Route> to={Route::App} classes="inline-flex items-center gap-2 py-3 px-7 bg-white/[0.06] border border-white/[0.1] rounded-xl text-zinc-200 text-sm font-medium no-underline transition-all duration-200 hover:bg-white/[0.1] hover:border-white/[0.15] hover:-translate-y-0.5">
                            { "enter murm" }
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="5" y1="12" x2="19" y2="12"></line>
                                <polyline points="12 5 19 12 12 19"></polyline>
                            </svg>
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </>
    }
}
