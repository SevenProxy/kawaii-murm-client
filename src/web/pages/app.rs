use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};

use crate::event::profile::Profile;
use crate::identity::Identity;
use crate::web::router::Route;

const PROFILE_KEY: &str = "murm_profile";

fn load_profile() -> Profile {
    LocalStorage::get(PROFILE_KEY).unwrap_or_default()
}

fn save_profile(profile: &Profile) {
    let _ = LocalStorage::set(PROFILE_KEY, profile);
}

#[function_component(AppPage)]
pub fn app_page() -> Html {
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

    let active_channel = use_state(|| "general".to_string());

    // Profile settings state
    let settings_open = use_state(|| false);
    let profile = use_state(load_profile);
    let edit_name = use_state(|| String::new());
    let edit_about = use_state(|| String::new());
    let edit_picture = use_state(|| String::new());

    // Sync edit fields when settings open
    {
        let profile = profile.clone();
        let edit_name = edit_name.clone();
        let edit_about = edit_about.clone();
        let edit_picture = edit_picture.clone();
        let is_open = *settings_open;
        use_effect_with(is_open, move |is_open| {
            if *is_open {
                edit_name.set(profile.name.clone().unwrap_or_default());
                edit_about.set(profile.about.clone().unwrap_or_default());
                edit_picture.set(profile.picture.clone().unwrap_or_default());
            }
            || ()
        });
    }

    let toggle_settings = {
        let settings_open = settings_open.clone();
        Callback::from(move |_: MouseEvent| {
            settings_open.set(!*settings_open);
        })
    };

    let close_settings = {
        let settings_open = settings_open.clone();
        Callback::from(move |_: MouseEvent| {
            settings_open.set(false);
        })
    };

    let on_save_profile = {
        let profile = profile.clone();
        let edit_name = edit_name.clone();
        let edit_about = edit_about.clone();
        let edit_picture = edit_picture.clone();
        let settings_open = settings_open.clone();
        Callback::from(move |_: MouseEvent| {
            let mut p = Profile::new();
            let name = (*edit_name).trim().to_string();
            let about = (*edit_about).trim().to_string();
            let picture = (*edit_picture).trim().to_string();
            if !name.is_empty() {
                p = p.name(name);
            }
            if !about.is_empty() {
                p = p.about(about);
            }
            if !picture.is_empty() {
                p = p.picture(picture);
            }
            save_profile(&p);
            profile.set(p);
            settings_open.set(false);
        })
    };

    let display_name = profile.name.clone().unwrap_or_else(|| "you".to_string());

    let channels = vec![
        ("general", "chat"),
        ("announcements", "megaphone"),
        ("random", "hash"),
    ];

    html! {
        <div class="flex h-screen bg-[#09090b] text-zinc-200 overflow-hidden">
            // Server sidebar (narrow icon strip)
            <div class="flex flex-col items-center w-[72px] bg-[#0a0a0c] border-r border-white/[0.04] py-3 gap-3 shrink-0">
                // Home button
                <Link<Route> to={Route::Home}
                    classes="flex items-center justify-center w-12 h-12 rounded-2xl bg-yellow-500/20 border border-yellow-500/15 transition-all duration-200 hover:rounded-xl hover:bg-yellow-500/30">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
                        <path d="M2 17l10 5 10-5"></path>
                        <path d="M2 12l10 5 10-5"></path>
                    </svg>
                </Link<Route>>

                <div class="w-8 h-px bg-white/[0.06] my-1"></div>

                // Server icon placeholder
                <div class="flex items-center justify-center w-12 h-12 rounded-full bg-white/[0.06] border border-white/[0.06] transition-all duration-200 hover:rounded-xl hover:bg-white/[0.1] cursor-pointer">
                    <span class="text-sm font-semibold text-zinc-400">{ "m" }</span>
                </div>

                <div class="flex-1"></div>

                // User avatar at bottom - opens settings
                <div onclick={toggle_settings.clone()}
                    class="flex items-center justify-center w-10 h-10 rounded-full bg-yellow-500/20 border border-yellow-500/15 cursor-pointer transition-all duration-200 hover:rounded-xl">
                    if let Some(ref url) = profile.picture {
                        <img src={url.clone()} class="w-full h-full rounded-full object-cover" />
                    } else {
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                            <circle cx="12" cy="7" r="4"></circle>
                        </svg>
                    }
                </div>
            </div>

            // Channel sidebar
            <div class="flex flex-col w-60 bg-[#0c0c0e] border-r border-white/[0.04] shrink-0">
                // Server header
                <div class="flex items-center justify-between h-12 px-4 border-b border-white/[0.06] shrink-0">
                    <span class="text-sm font-semibold text-zinc-200 truncate">{ "murm" }</span>
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-zinc-500 cursor-pointer hover:text-zinc-300 transition-colors">
                        <polyline points="6 9 12 15 18 9"></polyline>
                    </svg>
                </div>

                // Channel list
                <div class="flex-1 overflow-y-auto py-3 px-2">
                    <div class="mb-4">
                        <div class="flex items-center gap-1 px-2 mb-1">
                            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="text-zinc-500">
                                <polyline points="6 9 12 15 18 9"></polyline>
                            </svg>
                            <span class="text-[11px] font-semibold uppercase tracking-wider text-zinc-500">{ "channels" }</span>
                        </div>

                        {
                            channels.iter().map(|(name, icon)| {
                                let is_active = *active_channel == *name;
                                let name_str = name.to_string();
                                let onclick = {
                                    let active_channel = active_channel.clone();
                                    let name_str = name_str.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        active_channel.set(name_str.clone());
                                    })
                                };

                                let active_classes = if is_active {
                                    "bg-white/[0.08] text-zinc-100"
                                } else {
                                    "text-zinc-500 hover:bg-white/[0.04] hover:text-zinc-300"
                                };

                                html! {
                                    <div
                                        key={name_str}
                                        onclick={onclick}
                                        class={classes!(
                                            "flex", "items-center", "gap-2", "px-2", "py-1.5",
                                            "rounded-lg", "cursor-pointer", "text-sm",
                                            "transition-all", "duration-150",
                                            active_classes
                                        )}
                                    >
                                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-60 shrink-0">
                                            if *icon == "chat" {
                                                <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
                                            } else if *icon == "megaphone" {
                                                <>
                                                    <path d="M3 11l18-5v12L3 13v-2z"></path>
                                                    <path d="M11.6 16.8a3 3 0 1 1-5.8-1.6"></path>
                                                </>
                                            } else {
                                                <line x1="4" y1="9" x2="20" y2="9"></line>
                                                <line x1="4" y1="15" x2="20" y2="15"></line>
                                                <line x1="10" y1="3" x2="8" y2="21"></line>
                                                <line x1="16" y1="3" x2="14" y2="21"></line>
                                            }
                                        </svg>
                                        <span class="truncate">{ name }</span>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                // User panel at bottom - opens settings
                <div onclick={toggle_settings}
                    class="flex items-center gap-2 p-2 bg-[#08080a] border-t border-white/[0.04] shrink-0 cursor-pointer hover:bg-white/[0.03] transition-colors">
                    <div class="flex items-center justify-center w-8 h-8 rounded-full bg-yellow-500/20 shrink-0 overflow-hidden">
                        if let Some(ref url) = profile.picture {
                            <img src={url.clone()} class="w-full h-full object-cover" />
                        } else {
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                                <circle cx="12" cy="7" r="4"></circle>
                            </svg>
                        }
                    </div>
                    <div class="flex-1 min-w-0">
                        <div class="text-xs font-medium text-zinc-300 truncate">{ &display_name }</div>
                        <div class="text-[10px] text-zinc-500 font-mono truncate">{ &short_pk[..12.min(short_pk.len())] }</div>
                    </div>
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-zinc-600 shrink-0">
                        <circle cx="12" cy="12" r="3"></circle>
                        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
                    </svg>
                </div>
            </div>

            // Main content area
            <div class="flex-1 flex flex-col min-w-0">
                // Channel header
                <div class="flex items-center h-12 px-4 border-b border-white/[0.06] shrink-0">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-zinc-500 mr-2 shrink-0">
                        <line x1="4" y1="9" x2="20" y2="9"></line>
                        <line x1="4" y1="15" x2="20" y2="15"></line>
                        <line x1="10" y1="3" x2="8" y2="21"></line>
                        <line x1="16" y1="3" x2="14" y2="21"></line>
                    </svg>
                    <span class="text-sm font-semibold text-zinc-200">{ &*active_channel }</span>
                    <div class="w-px h-5 bg-white/[0.06] mx-3"></div>
                    <span class="text-xs text-zinc-500 truncate">{ "welcome to the conversation" }</span>
                </div>

                // Messages area
                <div class="flex-1 overflow-y-auto p-4">
                    <div class="max-w-3xl mx-auto">
                        <div class="flex flex-col items-center justify-center h-full text-center py-20">
                            <div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-yellow-500/10 border border-yellow-500/10 mb-6">
                                <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="opacity-70">
                                    <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
                                </svg>
                            </div>
                            <h2 class="text-xl font-semibold text-zinc-200 mb-2">{ format!("welcome to #{}", &*active_channel) }</h2>
                            <p class="text-sm text-zinc-500 max-w-sm">
                                { "this is the start of the conversation. messages are end-to-end encrypted with your identity." }
                            </p>
                        </div>
                    </div>
                </div>

                // Message input
                <div class="px-4 pb-4 shrink-0">
                    <div class="max-w-3xl mx-auto">
                        <div class="flex items-center gap-3 bg-white/[0.04] border border-white/[0.06] rounded-xl px-4 py-3 transition-all duration-200 focus-within:border-yellow-500/20 focus-within:bg-white/[0.06]">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-zinc-500 shrink-0 cursor-pointer hover:text-zinc-300 transition-colors">
                                <circle cx="12" cy="12" r="10"></circle>
                                <line x1="12" y1="8" x2="12" y2="16"></line>
                                <line x1="8" y1="12" x2="16" y2="12"></line>
                            </svg>
                            <input
                                type="text"
                                placeholder={ format!("message #{}", &*active_channel) }
                                class="flex-1 bg-transparent text-sm text-zinc-200 placeholder-zinc-600 outline-none border-none"
                            />
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-zinc-500 shrink-0 cursor-pointer hover:text-zinc-300 transition-colors">
                                <path d="M22 2L11 13"></path>
                                <path d="M22 2l-7 20-4-9-9-4 20-7z"></path>
                            </svg>
                        </div>
                    </div>
                </div>
            </div>

            // Settings modal overlay
            if *settings_open {
                <div class="fixed inset-0 z-50 flex items-center justify-center">
                    // Backdrop
                    <div onclick={close_settings.clone()}
                        class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>

                    // Settings card
                    <div class="relative w-full max-w-lg mx-4 backdrop-blur-xl bg-[#111113] border border-white/[0.06] rounded-2xl shadow-2xl overflow-hidden"
                         style="animation: fade-up 0.25s cubic-bezier(0.16, 1, 0.3, 1) both;">

                        // Header
                        <div class="flex items-center justify-between px-6 py-4 border-b border-white/[0.06]">
                            <h2 class="text-base font-semibold text-zinc-100">{ "edit profile" }</h2>
                            <div onclick={close_settings.clone()}
                                class="flex items-center justify-center w-7 h-7 rounded-lg bg-white/[0.04] text-zinc-500 cursor-pointer hover:bg-white/[0.08] hover:text-zinc-300 transition-all">
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                    <line x1="18" y1="6" x2="6" y2="18"></line>
                                    <line x1="6" y1="6" x2="18" y2="18"></line>
                                </svg>
                            </div>
                        </div>

                        // Body
                        <div class="px-6 py-5 space-y-5">
                            // Avatar section
                            <div class="flex items-center gap-4">
                                <div class="shrink-0">
                                    if let Some(ref url) = profile.picture {
                                        <img src={url.clone()}
                                            class="w-16 h-16 rounded-2xl object-cover border border-white/[0.06]" />
                                    } else {
                                        <div class="w-16 h-16 rounded-2xl bg-yellow-500/15 border border-yellow-500/10 flex items-center justify-center">
                                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="opacity-70">
                                                <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                                                <circle cx="12" cy="7" r="4"></circle>
                                            </svg>
                                        </div>
                                    }
                                </div>
                                <div class="flex-1 min-w-0">
                                    <label class="block text-[11px] font-medium uppercase tracking-wider text-zinc-500 mb-1.5">{ "avatar url" }</label>
                                    <input
                                        type="text"
                                        value={(*edit_picture).clone()}
                                        oninput={
                                            let edit_picture = edit_picture.clone();
                                            Callback::from(move |e: InputEvent| {
                                                edit_picture.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                            })
                                        }
                                        placeholder="https://example.com/avatar.png"
                                        class="w-full bg-white/[0.04] border border-white/[0.06] rounded-lg px-3 py-2 text-sm text-zinc-200 placeholder-zinc-600 outline-none transition-all duration-200 focus:border-yellow-500/20 focus:bg-white/[0.06]"
                                    />
                                </div>
                            </div>

                            // Name field
                            <div>
                                <label class="block text-[11px] font-medium uppercase tracking-wider text-zinc-500 mb-1.5">{ "display name" }</label>
                                <input
                                    type="text"
                                    value={(*edit_name).clone()}
                                    oninput={
                                        let edit_name = edit_name.clone();
                                        Callback::from(move |e: InputEvent| {
                                            edit_name.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                        })
                                    }
                                    placeholder="what should we call you?"
                                    class="w-full bg-white/[0.04] border border-white/[0.06] rounded-lg px-3 py-2 text-sm text-zinc-200 placeholder-zinc-600 outline-none transition-all duration-200 focus:border-yellow-500/20 focus:bg-white/[0.06]"
                                />
                            </div>

                            // About field
                            <div>
                                <label class="block text-[11px] font-medium uppercase tracking-wider text-zinc-500 mb-1.5">{ "about" }</label>
                                <textarea
                                    value={(*edit_about).clone()}
                                    oninput={
                                        let edit_about = edit_about.clone();
                                        Callback::from(move |e: InputEvent| {
                                            edit_about.set(e.target_unchecked_into::<web_sys::HtmlTextAreaElement>().value());
                                        })
                                    }
                                    placeholder="tell us a bit about yourself"
                                    rows="3"
                                    class="w-full bg-white/[0.04] border border-white/[0.06] rounded-lg px-3 py-2 text-sm text-zinc-200 placeholder-zinc-600 outline-none resize-none transition-all duration-200 focus:border-yellow-500/20 focus:bg-white/[0.06]"
                                />
                            </div>

                            // Public key (read-only)
                            <div>
                                <label class="block text-[11px] font-medium uppercase tracking-wider text-zinc-500 mb-1.5">{ "public key" }</label>
                                <div class="flex items-center gap-2 w-full bg-white/[0.02] border border-white/[0.04] rounded-lg px-3 py-2">
                                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#ca8a04" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-50 shrink-0">
                                        <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
                                        <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
                                    </svg>
                                    <span class="text-xs text-zinc-500 font-mono truncate">{ &pubkey_display }</span>
                                </div>
                            </div>
                        </div>

                        // Footer
                        <div class="flex items-center justify-end gap-3 px-6 py-4 border-t border-white/[0.06] bg-white/[0.01]">
                            <button onclick={close_settings}
                                class="px-4 py-2 text-sm text-zinc-400 hover:text-zinc-200 transition-colors rounded-lg hover:bg-white/[0.04]">
                                { "cancel" }
                            </button>
                            <button onclick={on_save_profile}
                                class="btn-primary px-5 py-2 text-sm">
                                { "save" }
                            </button>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
