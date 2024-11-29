use crate::api::Api;
use web_sys::Url;
use crate::app::{ApiError, GameStatus, Player, PlayersCreate, SantaGameInfo};
use gloo::console::log;
use gloo::dialogs::alert;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use web_sys::{window, MouseEvent};
use yew::{function_component, html, prelude::*, Html};
use yew_i18n::use_translation;

#[function_component(Confetti)]
pub fn confetti() -> Html {
    html! {
        <div id="confettis">
            <div class="confetti"></div>
            <div class="confetti"></div>
            <div class="confetti"></div>
            <div class="confetti"></div>
            <div class="confetti"></div>
            <div class="confetti"></div>
            <div class="confetti"></div>
            <div class="confetti"></div>
            <div class="confetti"></div>
            <div class="confetti"></div>
            <div class="confetti"></div>
        </div>
    }
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PropsStartGame {
    pub santa_game_info: UseStateHandle<SantaGameInfo>,
    pub selected_language: String,
}

#[function_component(InitGame)]
pub fn init_game(props: &PropsStartGame) -> Html {
    let api = Api::new();
    let error_msg: UseStateHandle<Option<String>> = use_state(|| None);
    let sante_game_info = props.santa_game_info.clone();
    let participant_name: UseStateHandle<String> = use_state(|| "".to_string());
    let is_loading: UseStateHandle<bool> = use_state(|| false);
    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(&props.selected_language);

    let participant_name_on_change = {
        let participant_name = participant_name.clone();
        Callback::from(move |event: Event| {
            let input: HtmlInputElement =
                event.target().unwrap().unchecked_into::<HtmlInputElement>();
            let value = input.value();
            participant_name.set(value);
        })
    };

    let start_game = {
        let santa_game_info_clone = sante_game_info.clone();
        let api = api.clone();
        move |_| {
            let santa_game_info_clone = santa_game_info_clone.clone();
            let api = api.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let api = api.clone();
                let response = api.start_game().await.unwrap();
                if response.status() != 200 {
                    let api_response = response.json::<ApiError>().await.unwrap();
                    log!(format!("Error msg : {}", api_response.error));
                } else {
                    let santa_game = santa_game_info_clone.deref().clone();
                    santa_game_info_clone.set(SantaGameInfo {
                        status: GameStatus::InProgress,
                        ..santa_game
                    });
                }
            });
        }
    };

    let onsubmit = {
        let participant_name_clone = participant_name.clone();
        let is_loading_clone = is_loading.clone();
        let santa_game_info_clone = sante_game_info.clone();
        let error_msg_clone = error_msg.clone();
        let api = api.clone();

        Callback::from(move |event: SubmitEvent| {
            // validations
            let error_msg_clone = error_msg_clone.clone();
            error_msg_clone.set(None);
            event.prevent_default();
            if participant_name_clone.deref().trim().is_empty() {
                error_msg_clone.set(Some("Nome não pode ser vazio".to_string()));
                return;
            }
            // check if there is non alphabetic characters but allow spaces
            if participant_name_clone
                .deref()
                .chars()
                .any(|c| !c.is_alphabetic() && !c.is_whitespace())
            {
                error_msg_clone.set(Some(
                    "Nome não pode conter caracteres especiais".to_string(),
                ));
                return;
            }

            let participant_name_clone = participant_name_clone.clone();
            let is_loading_clone = is_loading_clone.clone();
            let santa_game_info_clone = santa_game_info_clone.clone();
            let api = api.clone();

            is_loading_clone.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let response = api.add_player(&participant_name_clone.deref()).await;
                let error_msg_clone = error_msg_clone.clone();

                match response {
                    Ok(response) => {
                        if response.status() != 200 {
                            let api_response = response.json::<ApiError>().await.unwrap();
                            log!(format!("Error msg : {}", api_response.error));
                            if api_response.error.contains("already exists") {
                                error_msg_clone.set(Some(
                                    "Participante já existe, tente outro nome".to_string(),
                                ));
                            }
                            return;
                        }
                        let response = response.json::<PlayersCreate>().await.unwrap();
                        log!(serde_json::to_string_pretty(&response).unwrap());
                        // update participants
                        let participant_name = participant_name_clone.deref().clone();
                        participant_name_clone.set("".to_string());
                        // update game info
                        let mut participant_list = santa_game_info_clone.deref().clone().players;
                        let player = Player {
                            name: participant_name.clone(),
                            has_picked: false,
                        };

                        participant_list.push(player);
                        santa_game_info_clone.set(SantaGameInfo {
                            players: participant_list,
                            ..santa_game_info_clone.deref().clone()
                        });
                    }
                    Err(err) => {
                        log!(format!("something bad happend: {}", err));
                        alert("Erro no servidor");
                    }
                }
            });

            is_loading_clone.set(false);
        })
    };

    let remove_player = Callback::from(move |name: String| {
        log!(format!("Removing player {}", name));
        let santa_game_info_clone = sante_game_info.clone();
        let api = api.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let santa_game_info_clone = santa_game_info_clone.clone();
            let response = api.remove_player(&name).await;

            match response {
                Ok(response) => {
                    if response.status() != 200 {
                        let api_response = response.json::<ApiError>().await.unwrap();
                        log!(format!("Error msg : {}", api_response.error));
                        return;
                    }
                    // update participants
                    let mut participant_list =
                        santa_game_info_clone.deref().clone().players.clone();
                    participant_list.retain(|player| player.name != name);
                    santa_game_info_clone.set(SantaGameInfo {
                        players: participant_list.clone(),
                        ..santa_game_info_clone.deref().clone()
                    });
                }
                Err(err) => log!(format!("something bad happend: {}", err)),
            }
        });
    });

    // for better visualization
    let sante_game_info_clone = props.santa_game_info.clone();
    let mut new_participants = HashMap::new();
    for (id, player) in sante_game_info_clone.deref().players.iter().enumerate() {
        let id = id + 1;
        new_participants.insert(id as i32, player.name.clone());
    }
    let mut keys = new_participants.keys().collect::<Vec<&i32>>();
    keys.sort();

    let input_class = if error_msg.deref().is_some() {
        "bg-red-50 border border-red-500 text-red-900 placeholder-red-700 text-sm rounded-lg focus:ring-red-500 dark:bg-gray-700 focus:border-red-500 block w-full p-2.5 dark:text-red-500 dark:placeholder-red-500 dark:border-red-500"
    } else {
        "block p-2.5 w-full z-20 text-sm text-gray-900 bg-gray-50 rounded-lg rounded-s-gray-100 rounded-s-2 border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:border-blue-500"
    };
    html! {
            if *is_loading {
                <div>{ &i18n.t("Loading...") }</div>
            } else {

            <div class="flex justify-center">
                <div class="w-full max-w-[48rem]">
                    <span class="text-sm text-gray-500 dark:text-gray-400"> { &i18n.t("Enter the names of the participants") }</span>
                        <form class="mt-4" onsubmit={onsubmit}>
                                <div class="flex">
                                <div class="relative w-full">
                                    <input onchange={participant_name_on_change} value={participant_name.deref().clone()} type="text" id="name" class={input_class} placeholder={i18n.t("John")} required={true}/>
                                        <button type="submit" class="absolute top-0 end-0 p-2.5 h-full text-sm font-medium text-white bg-blue-700 rounded-e-lg border border-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                                            <svg class="w-4 h-4" aria-hidden="true" fill="white" version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 45.402 45.402" data-darkreader-inline-fill="" style="--darkreader-inline-fill: white;"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <g> <path d="M41.267,18.557H26.832V4.134C26.832,1.851,24.99,0,22.707,0c-2.283,0-4.124,1.851-4.124,4.135v14.432H4.141 c-2.283,0-4.139,1.851-4.138,4.135c-0.001,1.141,0.46,2.187,1.207,2.934c0.748,0.749,1.78,1.222,2.92,1.222h14.453V41.27 c0,1.142,0.453,2.176,1.201,2.922c0.748,0.748,1.777,1.211,2.919,1.211c2.282,0,4.129-1.851,4.129-4.133V26.857h14.435 c2.283,0,4.134-1.867,4.133-4.15C45.399,20.425,43.548,18.557,41.267,18.557z"></path> </g> </g></svg>
                                        </button>
                                </div>
                            </div>

                            if let Some(error_msg) = error_msg.deref() {
                                    <p class="mt-2 text-sm text-red-600 dark:text-red-500"><span class="font-medium">{"Ops! "}</span>{error_msg}</p>
                            }
                        </form>
                    <div class="flow-root">
                        <ul role="list" class="divide-y divide-gray-200 dark:divide-gray-700">
                            {for keys.iter().map(|key| {
                                let participant_name = new_participants.get(key).unwrap().clone();
                                let participant_name_clone = participant_name.clone();
                                let remove_player = remove_player.clone();
                                return html! {
                                    <li class="py-3 sm:py-4 hover:bg-gray-50 dark:hover:bg-gray-700">
                                        <div class="flex items-center justify-between">
                                            <p class="text-sm font-medium text-gray-900 truncate dark:text-white ml-2">
                                                {participant_name.clone()}
                                            </p>
                                            <button onclick={move |_| remove_player.emit(participant_name_clone.clone())} type="button" class=" text-blue-700 border border-blue-700 hover:bg-blue-700 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-centeritems-center dark:border-blue-500 dark:text-blue-500 dark:hover:text-white dark:focus:ring-blue-800 dark:hover:bg-blue-500 mr-2">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none">
    <path d="M4 7H20M10 10V18M14 10V18M10 3H14C14.2652 3 14.5196 3.10536 14.7071 3.29289C14.8946 3.48043 15 3.73478 15 4V7H9V4C9 3.73478 9.10536 3.48043 9.29289 3.29289C9.48043 3.10536 9.73478 3 10 3ZM6 7H18V20C18 20.2652 17.8946 20.5196 17.7071 20.7071C17.5196 20.8946 17.2652 21 17 21H7C6.73478 21 6.48043 20.8946 6.29289 20.7071C6.10536 20.5196 6 20.2652 6 20V7Z" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
                                                <span class="sr-only">{&i18n.t("Remover participante")}</span>
                                            </button>
                                        </div>
                                    </li>
                                }
                                }
                            )}

                        </ul>
                    </div>

                    {if keys.len() > 2 {
                        html! {
                            <div class="flex justify-center mt-4">
                                <button class="px-6 py-3.5 text-base font-medium text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 rounded-lg text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={start_game}>{ &i18n.t("Start Game") }</button>
                            </div>
                        }
                    }else{
                        html!{}
                    }}
                </div>
            </div>
            }
        }
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PropsInProgressGame {
    pub participants: Vec<Player>,
    pub selected_language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
}


fn get_url() -> String {
    if cfg!(debug_assertions) {
        "http://localhost:8080/".to_string()
    } else {
        let current_windown = window().expect("no global window exists");
        let location = current_windown.location();

        // Parse the full URL
        let full_url = location.href().unwrap_or_default();
        let parsed_url = Url::new(&full_url).expect("Failed to parse URL");

        // remove port from host
        let url_without_port = parsed_url.hostname();
        let url_parts: Vec<&str> = url_without_port.split(':').collect();
        let hostname = url_parts[0];

        // build url https//<host>/<port>
        format!("https://{}/{}/", hostname, parsed_url.port()).to_string()
    }
}

#[function_component(InProgressGame)]
pub fn in_progress(props: &PropsInProgressGame) -> Html {
    let partcipant_selected: UseStateHandle<String> = use_state(|| "".to_string());
    let sorted_participant: UseStateHandle<Option<Person>> = use_state(|| None);
    let mut i18n = use_translation();
    let _ = i18n.set_translation_language(&props.selected_language);
    let url = get_url();
    let api = Api::new();

    let onchange = {
        let partcipant_selected = partcipant_selected.clone();
        Callback::from(move |event: Event| {
            let input: HtmlInputElement =
                event.target().unwrap().unchecked_into::<HtmlInputElement>();
            let value = input.value();
            partcipant_selected.set(value.clone());
        })
    };

    let onclick = {
        let partcipant_selected = partcipant_selected.clone();
        let sorted_paticipant_clone = sorted_participant.clone();
        Callback::from(move |_| {
            let participant_selected = partcipant_selected.deref().clone();
            let sorted_paticipant_clone = sorted_paticipant_clone.clone();
            let api = api.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = api.pick_player(&participant_selected).await;

                let sorted_participant = sorted_paticipant_clone.clone();
                match response {
                    Ok(response) => {
                        if response.status() != 200 {
                            let api_response = response.json::<ApiError>().await.unwrap();
                            log!(format!("Error msg : {}", api_response.error));
                            return;
                        }
                        let response = response.json::<Person>().await.unwrap();
                        sorted_participant.set(Some(response));
                    }
                    Err(err) => log!(format!("something bad happend: {}", err)),
                }
            });
        })
    };

    let copy_to_clipboard = {
        let url = url.clone();
        let i18n = i18n.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(window) = window() {
                if let Some(navigator) = window.navigator().dyn_into::<web_sys::Navigator>().ok() {
                    let clipboard = navigator.clipboard();
                    let text = url.clone();
                    let i18n = i18n.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let promise = clipboard.write_text(&text);
                        match wasm_bindgen_futures::JsFuture::from(promise).await {
                            Ok(_) => {
                                alert( &i18n.t("Copied to the clipboard"));
                            }
                            Err(err) => {
                                alert(&format!(
                                    "Erro ao copiar para a área de transferência: {:?}",
                                    err
                                ));
                            }
                        }
                    });
                }
            }
        })
    };

    let participants_filtred = props
        .participants
        .iter()
        .filter(move |participant| !participant.has_picked)
        .map(|participant| participant.clone())
        .collect::<Vec<Player>>();

    html! {
        <div>
            { if let Some(sorted_participant) = sorted_participant.deref() {
                html! {
                    <>
                    <Confetti/>
                    <h1 class="animate__animated animate__rubberBand mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white">{ &i18n.t("You picked")}
                    <span class="text-blue-600 dark:text-blue-500">{sorted_participant.name.clone()}</span>
                    </h1>
                    </>
                }
            } else {
                html! {

                    <div class="">

                        <p class="animate__tada text-sm text-gray-500 dark:text-gray-400 mb-4"> { &i18n.t("Who are you ?") }</p>
                        <select onchange={onchange} class="block w-full px-4 py-3 text-base text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" >
                            <option selected={true}  value={""}>{&i18n.t("Select your name")}</option>
                        {
                            participants_filtred.iter().map(|participant| html! {
                                <option value={participant.name.clone()}>{participant.name.clone()}</option>
                            }).collect::<Html>()
                        }
                        </select>
                        { if !partcipant_selected.deref().is_empty() {
                            html! {
                                <button class="mt-10 px-6 py-3.5 text-base font-medium text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 rounded-lg text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={onclick}>{ &i18n.t("Pick")}</button>
                            }
                        }else{
                            html!{}
                        }}

                        <h2 class="animate__animated animate__rubberBand mb-4 mt-20 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl dark:text-white">{&i18n.t("Share with other players")}</h2>
                        <div class="flex justify-center">
                        <div class="grid grid-cols-8 gap-2 w-full max-w-[23rem]">
                            <label for="link" class="sr-only">{"Label"} </label>
                            <input
                                id="link"
                                type="text"
                                class="col-span-6 bg-gray-50 border border-gray-300 text-gray-500 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-gray-400 dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                value={url.clone()}
                                readonly={true}
                                disabled={true}
                            />

                            <button
                                class="col-span-2 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 items-center inline-flex justify-center"
                                onclick={copy_to_clipboard}
                            >

                                <span id="default-message">{&i18n.t("Copy link")}</span>
                                <span id="success-message" class="hidden inline-flex items-center">
                                    <svg class="w-3 h-3 text-white me-1.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 16 12">
                                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 5.917 5.724 10.5 15 1.5"/>
                                    </svg>
                                    {"Copied!"}
                                </span>
                            </button>
                        </div>
                        </div>
                    </div>
                }
            }}

        </div>
    }
}
