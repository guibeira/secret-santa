use std::ops::Deref;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::{function_component, html, prelude::*, use_effect_with, Html};

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
    html! {
    <div class="text-center">
        <div role="status">
            <svg aria-hidden="true" class="inline w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
            </svg>
            <span class="sr-only">{"Carregando..."}</span>
        </div>
    </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let santa_game_info: UseStateHandle<SantaGameInfo> = use_state(|| SantaGameInfo::default());
    let is_loading = use_state(|| true);
    let api = Api::default();

    let counter = santa_game_info.deref().players.iter().count();

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
                    return;
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

    html! {
            <div class="container mx-auto mt-10">

            if *is_loading {
                <Loading />
            }else{


                <div class="overflow-y-auto full max-h-screen p-4 text-center bg-white border border-gray-200 rounded-lg shadow sm:p-8 dark:bg-gray-800 dark:border-gray-700">
                    <div class="flex flex-col  pb-10">
                        <h1 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">{"Amigo Secreto"}</h1>
                        <span class="text-sm text-gray-500 dark:text-gray-400"> { format!("Total participantes  {}",counter) }</span>
                        <div class="flex mt-4 md:mt-6">
                        </div>


                    {match santa_game_info.deref().status {
                        GameStatus::NotStarted => {

                            html! {
                                <InitGame santa_game_info={santa_game_info.clone()} />
                            }
                        }
                        GameStatus::InProgress => {
                            html! {
                                <InProgressGame participants={santa_game_info.players.clone()} />
                            }
                        }
                        GameStatus::Finished => {
                            html! {
                                <>
                                    <h3 class="text-3xl font-bold dark:text-white">{"Jogo Finalizado"}</h3>
                                    <button class="mt-10 px-6 py-3.5 text-base font-medium text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 rounded-lg text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={reset_send} type="submit">{ "Reiniciar jogo" }</button>
                                </>
                            }
                        }
                    }
                    }
                    </div>
                </div>
            }
        </div>

    }
}
