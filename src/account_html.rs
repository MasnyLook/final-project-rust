use web_sys::{Document, Element};
use web_sys::{window, HtmlElement, HtmlInputElement, Request, RequestInit, Headers, Response, RequestMode, HtmlFormElement};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use js_sys::Reflect;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};

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
    let div = document.create_element("div").unwrap();
    let name = window().unwrap().local_storage().unwrap().unwrap().get_item("name").unwrap().unwrap();
    
    // Ustaw tekst wewnątrz elementu div
    div.set_inner_html(&format!("Witaj, {}", name));
    body.append_child(&div).unwrap();

    //button wyloguj
    let logout_button = document.create_element("button").unwrap();
    logout_button.set_attribute("id", "logoutButton").unwrap();
    logout_button.set_text_content(Some("Logout"));
    logout_button.set_attribute("class", "btn btn-primary mt-3").unwrap();
    body.append_child(&logout_button).unwrap();

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

// pub fn setup_login_form(
//     document: &Document,
//     body: &Element,
//     window: &web_sys::Window,
// ) {
//     println!("setup_login_form");
//     let document_clone = Rc::new(document.clone());
//     let window_clone = Rc::new(window.clone());
//     let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
//         event.prevent_default();
//         let form = document_clone.get_element_by_id("login-form").unwrap();
//         let username = form
//             .dyn_ref::<HtmlInputElement>()
//             .expect("login-form is not an HtmlInputElement")
//             .value();
//         let password = form
//             .dyn_ref::<HtmlInputElement>()
//             .expect("login-form is not an HtmlInputElement")
//             .value();

//         let storage = window_clone.local_storage().unwrap().unwrap();

//         println!("Username: {}, Password: {}", username, password);

//         let mut opts = RequestInit::new();
//         opts.set_method("POST");
//         // opts.body(Some(wasm_bindgen::JsValue::from_str("[1, 2, 3]")));
//         let body = wasm_bindgen::JsValue::from_str(&format!(r#"{{"name":"{}","password":"{}"}}"#, username, password));
//         opts.body(Some(&body));
//         opts.credentials(web_sys::RequestCredentials::Include);

//         let request = Request::new_with_str_and_init(
//             "http://127.0.0.1:8006/login",
//             &opts
//         ).unwrap();
//         request.headers().set("content-type", "application/json");

//         let window_clone = window_clone.clone();
//         wasm_bindgen_futures::spawn_local(async move {
//             let resp_value = JsFuture::from(window_clone.fetch_with_request(&request)).await.unwrap();
//             // Dalszy kod obsługi odpowiedzi
//         });


//         // let request = web_sys::Request::new_with_str_and_init(
//         //     "http://127.0.0.1:8006/login",
//         //     web_sys::RequestInit::new()
//         //     .method("POST")
//         //     .body(Some(&JsValue::from_str(&format!(r#"{{"name":"{}","password":"{}"}}"#, username, password))))
//         //     .headers(&{
//         //         let headers = web_sys::Headers::new().unwrap();
//         //         headers.set("Content-Type", "application/json").unwrap();
//         //         headers.into()
//         //     }),
//         // ).unwrap();

//         // let window_clone = window_clone.clone();
//         // wasm_bindgen_futures::spawn_local(async move {
//         //         let response = JsFuture::from(window_clone.fetch_with_request(&request)).await.unwrap();
//         //         let resp: Response = response.dyn_into().unwrap();
//         //         // let json = JsFuture::from(resp.json()?).await?;
//         //         println!("{:?}", resp);
            
//         //     // if response.ok() {
//         //     //     let json = response.json().await.unwrap();
//         //     //     println!("{:?}", json);
//         //     //     let token = js_sys::Reflect::get(&json, &JsValue::from_str("token")).unwrap().as_string().unwrap();
//         //     //     storage.set_item("token", &token).unwrap();
//         //     //     window_clone.location().set_href("/main.html").unwrap();
//         //     // } else {
//         //     //     web_sys::console::log_1(&JsValue::from_str("Login failed!"));
//         //     // }
//         // });

//     }) as Box<dyn FnMut(web_sys::Event)>);
//     let form = document.get_element_by_id("login-form").unwrap();
//     let form: HtmlElement = form.dyn_into().unwrap();
//     form.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref()).unwrap();
//     closure.forget();
// }

// #[wasm_bindgen]
// pub fn setup_login_form(
//     document: &Document,
//     body: &Element,
//     window: &web_sys::Window,
// ) {
//     let form = document.get_element_by_id("login-form")?;
//     let form: HtmlElement = form.dyn_into()?;

//     let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
//         event.prevent_default();
//         let username_input: HtmlInputElement = document.get_element_by_id("username").unwrap().dyn_into().unwrap();
//         let password_input: HtmlInputElement = document.get_element_by_id("password").unwrap().dyn_into().unwrap();

//         let username = username_input.value();
//         let password = password_input.value();

//         let window = window().unwrap();
//         let storage = window.local_storage().unwrap().unwrap();

//         // Wysyłanie danych logowania do serwera
//         let request = web_sys::Request::new_with_str_and_init(
//             "http://127.0.0.1:8006/login",
//             web_sys::RequestInit::new()
//                 .method("POST")
//                 .body(Some(&JsValue::from_str(&format!(r#"{{"name":"{}","password":"{}"}}"#, username, password))))
//                 .headers(&{
//                     let headers = web_sys::Headers::new().unwrap();
//                     headers.set("Content-Type", "application/json").unwrap();
//                     headers
//                 }),
//         ).unwrap();

//         let window_clone = window.clone();
//         let closure_clone = closure.clone();
//         wasm_bindgen_futures::spawn_local(async move {
//             let response = window_clone.fetch_with_request(&request).await.unwrap();
//             if response.ok() {
//                 let json = response.json().await.unwrap();
//                 let token = js_sys::Reflect::get(&json, &JsValue::from_str("token")).unwrap().as_string().unwrap();
//                 storage.set_item("token", &token).unwrap();
//                 window_clone.location().set_href("/main.html").unwrap();
//             } else {
//                 web_sys::console::log_1(&JsValue::from_str("Login failed!"));
//             }
//         });

//     }) as Box<dyn FnMut(_)>);

//     form.add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref()).unwrap();
//     closure.forget();
// }