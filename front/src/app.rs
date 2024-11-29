use gloo::console::log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::{function_component, html, prelude::*, use_effect_with, Html};
use yew_i18n::use_translation;
use yew_i18n::I18nProvider;

use crate::api::Api;
use crate::components::{InProgressGame, InitGame};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    NotStarted,
    InProgress,
    Finished,
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Properties)]
pub struct Player {
    pub name: String,
    pub has_picked: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SantaGameInfo {
    pub status: GameStatus,
    pub players: Vec<Player>,
}

impl Default for SantaGameInfo {
    fn default() -> Self {
        SantaGameInfo {
            status: GameStatus::NotStarted,
            players: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayersCreate {
    pub names: Vec<String>,
}

#[function_component(Loading)]
pub fn loading() -> Html {
    let mut i18n = use_translation();
    let selected_language = "en";
    let _ = i18n.set_translation_language(selected_language);

    html! {
    <div class="text-center">
        <div role="status">
            <svg aria-hidden="true" class="inline w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
            </svg>
            <span class="sr-only">{&i18n.t("Loading...")}</span>
        </div>
    </div>
    }
}

#[function_component(Wrap)]
pub fn wrap() -> Html {
    let mut translations = HashMap::new();

    translations.insert(
        "pt_BR".to_string(),
        serde_json::json!({
            "Loading...": "Carregando...",
            "Secret Santa": "Amigo Secreto",
            "Amount of participants": "Total de participantes",
            "Restart Game": "Reiniciar Jogo",
            "Remove Player": "Remover Jogador",
            "Enter the names of the participants": "Digite os nomes dos participantes",
            "John": "Fulano",
            "Loading...": "Carregando...",
            "Start Game": "Iniciar Jogo",
            "You picked": "Você tirou ",
            "Select your name": "Selecione seu nome",
            "Who are you ?": "Quem é você ?",
            "Pick": "Sortear",
            "Share with other players": "Compartilhe com outros jogadores 👇 ",
            "Copy link": "Copiar link",
            "Game Finished": "Jogo Finalizado",
            "Copied to the clipboard": "Copiado para a área de transferência",
            "You already picked": "Você já tirou",
        }),
    );

    translations.insert(
        "en".to_string(),
        serde_json::json!({
            "Loading...": "Loading...",
            "Secret Santa": "Secret Santa",
            "Amount of participants": "Amount of participants",
            "Restart Game": "Restart Game",
            "Remove Player": "Remove Player",
            "Enter the names of the participants": "Enter the names of the participants",
            "John": "John",
            "Loading...": "Loading...",
            "Start Game": "Start Game",
            "You picked": "You picked ",
            "Select your name": "Select your name",
            "Who are you ?": "Who are you ?",
            "Pick": "Pick",
            "Share with other players": "Share with other players 👇 ",
            "Copy link": "Copy link",
            "Game Finished": "Game Finished",
            "Copied to the clipboard": "Copied to the clipboard",
            "You already picked": "You already picked",
        }),
    );
    html! {

        <I18nProvider
            supported_languages={vec!["en", "pt_BR"]}
            translations={translations}
        >
            <App />
        </I18nProvider>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let santa_game_info: UseStateHandle<SantaGameInfo> = use_state(|| SantaGameInfo::default());
    let is_loading = use_state(|| true);

    let api = Api::new();

    let counter = santa_game_info.deref().players.len();

    let is_loading_clone = is_loading.clone();
    let santa_game_info_clone = santa_game_info.clone();

    // load initial data
    let api_clone = api.clone();
    use_effect_with((), move |_| {
        let santa_game_info = santa_game_info_clone.clone();
        is_loading_clone.set(true);
        let api = api_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let santa_game_info_clone = santa_game_info.clone();
            let response = api.info().await.unwrap().json::<SantaGameInfo>().await;

            match response {
                Ok(response) => santa_game_info_clone.set(response),
                Err(err) => {
                    log!(format!("something bad happend: {}", err));
                }
            };
        });
        is_loading_clone.set(false);

        || {}
    });

    let reset_send = {
        let is_loading_clone = is_loading.clone();

        let santa_game_info_clone = santa_game_info.clone();

        move |_| {
            is_loading_clone.set(true);
            let api_clone = api.clone();

            let santa_game_info_clone = santa_game_info_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = api_clone.reset_game().await.unwrap();
                log!(response.status());
                santa_game_info_clone.set(SantaGameInfo::default());
            });
            is_loading_clone.set(false);
        }
    };

    let selected_language_ref = use_node_ref();
    let selected_language_handle = use_state(|| "en".to_string());
    let selected_language = selected_language_handle.clone();

    let on_select_change = {
        let selected_language_ref = selected_language_ref.clone();
        let selected_language_handle = selected_language_handle.clone();
        Callback::from(move |_| {
            if let Some(input) = selected_language_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                selected_language_handle.set(value);
            }
        })
    };

    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(&selected_language);
    html! {
        <div class="container mx-auto mt-10">

            <div class="language-selector">
                <select
                    ref={selected_language_ref}
                    onchange={on_select_change}
                    class="text-sm bg-transparent border-none focus:outline-none"
                >
                    <option value="en" selected=true hidden=true>{ "🌐 Language" }</option>
                    { for i18n.config.supported_languages.iter().map(|&lang| render_language_option(lang)) }
                </select>
            </div>

            if *is_loading {
                <Loading />
            } else {
                <div class="overflow-y-auto full max-h-screen p-4 text-center bg-white border border-gray-200 rounded-lg shadow sm:p-8 dark:bg-gray-800 dark:border-gray-700">
                    <div class="flex flex-col pb-10">
                        <h1 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">{&i18n.t("Secret Santa")}</h1>
                        <span class="text-sm text-gray-500 dark:text-gray-400"> { format!("{} {}", &i18n.t("Amount of participants"), counter) }</span>
                        <div class="flex mt-4 md:mt-6">
                        </div>

                        {match santa_game_info.deref().status {
                            GameStatus::NotStarted => {
                                html! {
                                    <InitGame santa_game_info={santa_game_info.clone()} selected_language={selected_language.deref().clone()} />
                                }
                            }
                            GameStatus::InProgress => {
                                html! {
                                    <InProgressGame
                                        participants={santa_game_info.players.clone()}
                                        selected_language={selected_language.deref().clone()}
                                    />
                                }
                            }
                            GameStatus::Finished => {
                                html! {
                                    <>
                                        <h3 class="text-3xl font-bold dark:text-white">{ &i18n.t("Game Finished")}</h3>
                                        <button class="mt-10 px-6 py-3.5 text-base font-medium text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 rounded-lg text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={reset_send} type="submit">{&i18n.t("Restart Game") }</button>
                                    </>
                                }
                            }
                        }}
                    </div>
                </div>
            }
        </div>
    }
}

fn render_language_option(lang: &'static str) -> Html {
    let flag_emoji = match lang {
        "en" => "🇺🇸",
        "pt_BR" => "🇧🇷",
        _ => "🌐",
    };

    html! {
        <option value={lang}>{ format!("{} {}", flag_emoji, lang) }</option>
    }
}
