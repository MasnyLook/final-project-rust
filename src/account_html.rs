use web_sys::{Document, Element};
use web_sys::{window, HtmlElement, HtmlInputElement, Request, RequestInit, Headers, Response, RequestMode, HtmlFormElement};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use js_sys::Reflect;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use crate::models;
use crate::fetch_history;
use chrono::{DateTime, NaiveDateTime, Utc};

use reqwest::Client;
use reqwest::header;


pub fn create_login_form(document: &Document, body: &Element) {
    let form_container = document.create_element("div").unwrap();
    form_container.set_attribute("style", "display: flex; justify-content: center; align-items: center; height: 100vh;").unwrap();

    let form = document.create_element("form").unwrap();
    form.set_attribute("id", "login-form").unwrap();
    form.set_attribute("style", "display: flex; flex-direction: column; align-items: center; justify-content: center; border: 2px solid #ccc; padding: 20px; border-radius: 10px; box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);").unwrap();

    // Tworzenie pola dla nazwy użytkownika
    let username_label = document.create_element("label").unwrap();
    username_label.set_inner_html("Username:");
    form.append_child(&username_label).unwrap();

    let username_input: HtmlInputElement = document.create_element("input").unwrap().dyn_into().unwrap();
    username_input.set_attribute("type", "text").unwrap();
    username_input.set_attribute("id", "username").unwrap();
    username_input.set_attribute("name", "username").unwrap();
    username_input.set_attribute("required", "true").unwrap();
    form.append_child(&username_input).unwrap();

    // Tworzenie pola dla hasła
    let password_label = document.create_element("label").unwrap();
    password_label.set_inner_html("Password:");
    form.append_child(&password_label).unwrap();

    let password_input: HtmlInputElement = document.create_element("input").unwrap().dyn_into().unwrap();
    password_input.set_attribute("type", "password").unwrap();
    password_input.set_attribute("id", "password").unwrap();
    password_input.set_attribute("name", "password").unwrap();
    password_input.set_attribute("required", "true").unwrap();
    form.append_child(&password_input).unwrap();

    // Tworzenie przycisku logowania
    let login_button: HtmlElement = document.create_element("button").unwrap().dyn_into().unwrap();
    login_button.set_attribute("type", "submit").unwrap();
    login_button.set_inner_html("Login");
    form.append_child(&login_button).unwrap();

    // Dodanie formularza do kontenera
    form_container.append_child(&form).unwrap();

    // stworz nowy przycisk
    let restart_button = document.create_element("button").unwrap();
    restart_button.set_attribute("id", "restartButton").unwrap();
    restart_button.set_text_content(Some("Restart"));
    restart_button.set_attribute("class", "btn btn-primary mt-3").unwrap();
    body.append_child(&restart_button).unwrap();

    // Dodanie kontenera do podanego elementu
    body.append_child(&form_container).unwrap();
}

#[derive(Deserialize)]
struct LoginResponse {
    is_authenticated: bool,
    token: String,
}

pub fn setup_restart_button(
    document: &Document,
    body: &Element,
    window: &web_sys::Window,
) {
    let document_clone = document.clone();
    let window_clone = window.clone();
    let closure = Closure::wrap(Box::new(move || {
        let client = Client::new();
        let request_body = serde_json::json!({
            "name": "john",
            "password": "noble"
        });

        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let storage = window_clone.local_storage().unwrap().unwrap();
        spawn_local(async move {
            let res = client.post("http://127.0.0.1:8006/login")
                .headers(headers)
                .body(request_body.to_string())
                .send()
                .await;
                // .text()?;
                match res {
                    Ok(response) => {
                        let text = response.text().await;
                        match text {
                            Ok(body) => {
                                // Obsłuż odpowiedź
                                web_sys::console::log_1(&body.clone().into());
                                match serde_json::from_str::<LoginResponse>(&body) {
                                    Ok(login_response) => {
                                        web_sys::console::log_1(&format!("isAuthenticated: {}", login_response.is_authenticated).into());
                                        web_sys::console::log_1(&format!("cookie: {}", login_response.token).into());
                                        storage.set_item("token", &login_response.token).unwrap(); // CHECK
                                        storage.set_item("name", "jasio").unwrap();
                                    }
                                    Err(err) => {
                                        web_sys::console::log_1(&format!("Error parsing JSON: {:?}", err).into());
                                    }
                                }
                            }
                            Err(err) => {
                                // Obsłuż błąd
                                web_sys::console::log_1(&format!("Error reading response body: {:?}", err).into());
                            }
                        }
                    }
                    Err(err) => {
                        // Obsłuż błąd
                        web_sys::console::log_1(&format!("Request error: {:?}", err).into());
                    }
                }
        });
    }) as Box<dyn FnMut()>);

    let restart_button = document.get_element_by_id("restartButton").unwrap();
    restart_button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn ssetup_login_form(
    document: &Document,
    body: &Element,
    window: &web_sys::Window,
) {
    let document_clone = document.clone();
    let window_clone = window.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        event.prevent_default(); // prevent form submission
        let form = document_clone.get_element_by_id("login-form")
            .expect("login-form not found")
            .dyn_into::<HtmlFormElement>()
            .expect("login-form is not an HtmlFormElement");

        let username_input = form.query_selector("#username")
            .expect("username input not found")
            .expect("username input not found")
            .dyn_into::<HtmlInputElement>()
            .expect("username is not an HtmlInputElement");

        let password_input = form.query_selector("#password")
            .expect("password input not found")
            .expect("password input not found")
            .dyn_into::<HtmlInputElement>()
            .expect("password is not an HtmlInputElement");

        let username = username_input.value();
        let password = password_input.value();

        let storage = window_clone.local_storage().unwrap().unwrap();
        let client = Client::new();
        let request_body = serde_json::json!({
            "name": username,
            "password": password
        });
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let window_clone = window_clone.clone();

        spawn_local(async move {
            let res = client.post("http://127.0.0.1:8006/login")
                .headers(headers)
                .body(request_body.to_string())
                .send()
                .await;
                match res {
                    Ok(response) => {
                        let text = response.text().await;
                        match text {
                            Ok(body) => {
                                // Obsłuż odpowiedź
                                web_sys::console::log_1(&body.clone().into());
                                match serde_json::from_str::<LoginResponse>(&body) {
                                    Ok(login_response) => {
                                        web_sys::console::log_1(&format!("isAuthenticated: {}", login_response.is_authenticated).into());
                                        web_sys::console::log_1(&format!("cookie: {}", login_response.token).into());
                                        if login_response.is_authenticated {
                                            storage.set_item("token", &login_response.token).unwrap(); // CHECK
                                            storage.set_item("name", &username).unwrap();
                                            window_clone.location().set_href("/account.html").unwrap();
                                        } else {
                                            window_clone.alert_with_message("Authentication failed. Please check your username and password.").unwrap();
                                        }
                                        storage.set_item("auth", &login_response.is_authenticated.to_string()).unwrap();
                                    }
                                    Err(err) => {
                                        web_sys::console::log_1(&format!("Error parsing JSON: {:?}", err).into());
                                    }
                                }
                            }
                            Err(err) => {
                                web_sys::console::log_1(&format!("Error reading response body: {:?}", err).into());
                            }
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&format!("Request error: {:?}", err).into());
                    }
                }
        });



    }) as Box<dyn FnMut(_)>);
    let form = document.get_element_by_id("login-form").unwrap();
    let form: HtmlElement = form.dyn_into().unwrap();
    form.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn create_account_page(document: &Document, body: &Element) {
    fetch_history::fetch_user_board(document, body);

    let logout_button = document.create_element("button").unwrap();
    logout_button.set_attribute("id", "logoutButton").unwrap();
    logout_button.set_text_content(Some("Logout"));
    logout_button.set_attribute("class", "btn btn-primary mt-3").unwrap();
    logout_button.set_attribute("style", "float: right; margin-right: 10px;").unwrap();
    body.append_child(&logout_button).unwrap();

    let div = document.create_element("div").unwrap();
    let name = window().unwrap().local_storage().unwrap().unwrap().get_item("name").unwrap().unwrap();    
    div.set_inner_html(&format!("Hello, {}!", name));
    div.set_attribute("style", "font-size: 48px; margin-top: 20px; margin-left: 20px; color: lightblue; font-family: Arial, sans-serif;").unwrap();
    body.append_child(&div).unwrap();

    let closure = Closure::wrap(Box::new(move || {
        let window = window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        storage.remove_item("token").unwrap();
        storage.remove_item("name").unwrap();
        storage.remove_item("auth").unwrap();
        window.location().set_href("/account.html").unwrap();
    }) as Box<dyn FnMut()>);

    let logout_button = document.get_element_by_id("logoutButton").unwrap();
    logout_button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

// fn fetch_user_board(
//     document: &Document,
//     body: &Element,
// ) {
//     let window = web_sys::window().unwrap();
//     let message = prepare_request_body(&window);
//     let document_clone = document.clone();
//     let window_clone = window.clone();
//     let body_clone = body.clone();
//     spawn_local(async move {
//         let response = send_message_to_server(&window_clone, &message).await;
//         match response {
//             Ok(data) => {
//                 let results = parse_output_data(&data);
//                 match results {
//                     Ok(game_results) => {
//                         create_table_from_results(&document_clone, &body_clone, game_results);
//                     }
//                     Err(err) => {
//                         web_sys::console::log_1(&format!("Error parsing JSON: {:?}", err).into());
//                     }
//                 }
//             }
//             Err(err) => {
//                 web_sys::console::log_1(&format!("Request error: {:?}", err).into());
//             }
//         }
//     });
// }

// fn prepare_request_body (
//     window: &web_sys::Window,
// ) -> String {
//     let storage = window.local_storage().unwrap().unwrap();
//     let token = storage.get_item("token").unwrap().unwrap();
//     let name = storage.get_item("name").unwrap().unwrap();
//     let auth_token = models::AuthenticationToken {
//         user_name: name,
//         cookie: token,
//     };
//     let request_body = serde_json::to_string(&auth_token).unwrap();
//     web_sys::console::log_1(&JsValue::from_str(&request_body));
//     request_body
// }

// async fn send_message_to_server(window: &web_sys::Window, message: &str) -> Result<String, reqwest::Error>  {
//     let client = Client::new();
//     let mut headers = header::HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse().unwrap());
//     let request_body = message.to_string();

//     let response = client
//         .post("http://127.0.0.1:8006/user_board")
//         .headers(headers)
//         .body(request_body)
//         .send()
//         .await?;
            
//     let text = response.text().await?;
//     web_sys::console::log_1(&JsValue::from_str(&text));

//     Ok(text)
// }

// fn parse_output_data(data: &str) -> Result<Vec<models::GameResult>, serde_json::Error> {
//     let game_results: Vec<models::GameResult> = serde_json::from_str(data)?;
//     Ok(game_results)
// }

// pub fn create_table_from_results(document: &Document, body: &Element, results: Vec<models::GameResult>) {
//     let table = document.create_element("table").unwrap();
//     table.set_attribute("class", "table table-striped").unwrap();
//     table.set_attribute("style", "width: auto; margin: 0 auto; border: 1px solid grey;");

//     let caption = document.create_element("caption").unwrap();
//     caption.set_text_content(Some("History of Game Results"));
//     caption.set_attribute("style", "caption-side: top; font-size: 24px; font-weight: bold; margin-bottom: 10px;").unwrap();
//     table.append_child(&caption).unwrap();

//     let thead = document.create_element("thead").unwrap();
//     let header_row = document.create_element("tr").unwrap();
//     let headers = vec!["Time(sec)", "Moves", "Game Type", "Timestamp"];
//     for header in headers {
//         let th = document.create_element("th").unwrap();
//         th.set_text_content(Some(header));
//         th.set_attribute("style", "color: lightblue; text-align: center;").unwrap();
//         header_row.append_child(&th).unwrap();
//     }
//     thead.append_child(&header_row).unwrap();
//     table.append_child(&thead).unwrap();

//     let tbody = document.create_element("tbody").unwrap();
//     for result in results {
//         let row = document.create_element("tr").unwrap();
//         row.set_attribute("style", "color: white; text-align: center;").unwrap();

//         let score_time_cell = document.create_element("td").unwrap();
//         score_time_cell.set_text_content(Some(&result.score_time.to_string()));
//         row.append_child(&score_time_cell).unwrap();

//         let score_moves_cell = document.create_element("td").unwrap();
//         score_moves_cell.set_text_content(Some(&result.score_moves.to_string()));
//         row.append_child(&score_moves_cell).unwrap();

//         let game_type_cell = document.create_element("td").unwrap();
//         game_type_cell.set_text_content(Some(&result.game_type));
//         row.append_child(&game_type_cell).unwrap();

//         let timestamp_cell = document.create_element("td").unwrap();
//         let timestamp = format_timestamp(result.timestamp.secs_since_epoch, result.timestamp.nanos_since_epoch);
//         timestamp_cell.set_text_content(Some(&timestamp));
//         row.append_child(&timestamp_cell).unwrap();

//         tbody.append_child(&row).unwrap();
//     }
//     table.append_child(&tbody).unwrap();

//     body.append_child(&table).unwrap();
// }

// fn format_timestamp(secs_since_epoch: i64, nanos_since_epoch: i32) -> String {
//     let naive = NaiveDateTime::from_timestamp(secs_since_epoch, nanos_since_epoch as u32);
//     let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
//     datetime.format("%Y-%m-%d %H:%M:%S").to_string()
// }

pub fn go_back_to_main_page(
    document: &Document,
    body: &Element
) {
    let main_page_link = document.create_element("a").unwrap();
    main_page_link.set_attribute("href", "/main.html").unwrap();
    main_page_link.set_inner_html("Go back to main page");
    main_page_link.set_attribute("style", "position: fixed; bottom: 20px; left: 50%; transform: translateX(-50%);").unwrap();
    body.append_child(&main_page_link).unwrap();
}
