use crate::{
    constants::{LANG_CODES, LANG_NAMES, LOCALES},
    model::app_state::ApplicationData,
    ui::icon::{flags, Lang},
    utils::click::{close_elements, use_outside_click},
    ButtonLang, Route,
};
use dioxus::prelude::*;
// use dioxus_logger::tracing::Value;
use fluent_templates::Loader;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

#[component]
pub fn LangDropDown() -> Element {
    let mut data = use_context::<ApplicationData>();
    let lang_id = &LanguageIdentifier::from_str(&(data.lang_code)() as &str).unwrap();
    let rtl = use_memo(move || {
        if (data.lang_code)() == "ar" {
            true
        } else {
            false
        }
    });

    use_effect(use_reactive!(|| {
        use_outside_click(move |_| {
            (data.show_lang_menu).toggle();
        });
    }));

    rsx! {
        div { class: "relative ml-3",
            div {
                button {
                    class: "relative flex rounded-full bg-white text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                    r#type: "button",
                    id: "user-menu-button",
                    aria_expanded: "false",
                    aria_haspopup: "true",
                    onclick: move |ev| {
                        ev.stop_propagation();
                        (data.show_lang_menu).set(false);
                    },
                    // "Up high!"
                    span { class: "absolute -inset-1.5" }
                    span { class: "sr-only", "Open user menu" }

                    Lang {}
                }
            }
            div {
                id: "popup",
                class: "absolute z-10 mt-2 w-40 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none",
                class: if !(data.show_lang_menu)() { "hidden" },
                class: if !rtl() { "right-0" } else { "left-0" },
                role: "menu",
                aria_orientation: "vertical",
                aria_labelledby: "user-menu-button",
                ul { class: "flex flex-col",
                    for ((code , name) , flag) in LANG_CODES.iter().zip(LANG_NAMES.iter()).zip(flags().iter()) {
                        match *code {
                            "en" => rsx!{
                                Link { class: "grid grid-cols-3 gap-4 text-sm text-gray-700 hover:bg-slate-300 cursor-pointer hover:ring-1 items-center px-2 py-1",
                                    onclick: move |_| {
                                        (data.lang_code).set(code.to_string());
                                        let eval = ButtonLang();
                                        eval.send((*code).into()).unwrap();
                                        (data.show_lang_menu).toggle();
                                    },
                                    to: Route::Home {},
                                        div{ class: "col-span-1 ",  { flag } },
                                        div { class: "col-span-2 text-base", {LOCALES.lookup(lang_id, name)} }
                                },
                            },
                            _ => rsx!{
                                Link { class: "grid grid-cols-3 gap-4 text-sm text-gray-700 hover:bg-slate-300 cursor-pointer hover:ring-1 items-center px-2 py-1",
                                    onclick: move |_| {
                                        (data.lang_code).set(code.to_string());
                                        let eval = ButtonLang();
                                        eval.send((*code).into()).unwrap();
                                        (data.show_lang_menu).toggle();
                                    },
                                    to: Route::HomeLang {
                                        lang: code.to_string(),
                                    },
                                    div{ class: "col-span-1 ",  { flag } },
                                    div { class: "col-span-2 text-base", {LOCALES.lookup(lang_id, name)} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Other solution
// #[component]
// fn Home() -> Element {
//     rsx! {
//         h1 { class: "p-4", "Flags:" }
//         for (flag , lang) in FLAGS.iter().zip(LANG_NAMES.iter()) {
//             div { class: "",
//                 div { class: "px-4 py-1 flex flex-row space-x-2",
//                     div { {flag.render()} }
//                     div { "{lang}" }
//                 }
//             }
//         }
//     }
// }

// #[derive(Clone, Copy)]
// pub enum Flag {
//     De,
//     Sm,
// }

// impl Flag {
//     pub fn render(&self) -> Element {
//         match self {
//             Flag::De => rsx! {
//                 svg {
//                     class: "h-6",
//                     id: "flag-icons-de",
//                     view_box: "0 0 640 480",
//                     path { fill: "#fc0", d: "M0 320h640v160H0z" }
//                     path { fill: "#000001", d: "M0 0h640v160H0z" }
//                     path { fill: "red", d: "M0 160h640v160H0z" }
//                 }
//             },
//             Flag::Sm => rsx! {
//                 svg {
//                     class: "h-6",
//                     id: "flag-icons-sm",
//                     view_box: "0 0 640 480",
//                     path { fill: "#fcf", d: "M0 0h640v480H0z" }
//                     path { fill: "#002d8f", d: "M0 240h640v240H0z" }
//                     path { fill: "yellow", d: "M0 160h640v160H0z" }
//                 }
//             },
//         }
//     }
// }

// pub const FLAGS: [Flag; 2] = [Flag::De, Flag::Sm];
// pub const LANG_NAMES: [&str; 2] = ["English", "German"];
