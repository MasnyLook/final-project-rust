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

use crate::html;

pub fn create_login_form(document: &Document, body: &Element) {
    let form_container = html!(document, "div", {
        "style" => "display: flex; justify-content: center; align-items: center; height: 100vh;"
    });

    let form = html!(document, "form", {
        "id" => "login-form",
        "style" => "display: flex; flex-direction: column; align-items: center; justify-content: center; border: 2px solid #ccc; padding: 20px; border-radius: 10px; box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);"
    });

    // Login
    let username_label = html!(document, "label", "Username:");
    form.append_child(&username_label).unwrap();

    let username_input = html!(document, "input", {
        "type" => "text",
        "id" => "username",
        "name" => "username",
        "required" => "true"
    }).dyn_into().unwrap();
    form.append_child(&username_input).unwrap();

    // Password
    let password_label = html!(document, "label", "Password:");
    form.append_child(&password_label).unwrap();

    let password_input = html!(document, "input", {
        "type" => "password",
        "id" => "password",
        "name" => "password",
        "required" => "true"
    }).dyn_into().unwrap();
    form.append_child(&password_input).unwrap();

    // Login button
    let login_button = html!(document, "button", {
        "type" => "submit"
    }, "Login").dyn_into().unwrap();
    form.append_child(&login_button).unwrap();

    let register_button = html!(document, "button", {
        "type" => "button",
        "id" => "register-button"
    }, "Register").dyn_into().unwrap();
    form.append_child(&register_button).unwrap();

    form_container.append_child(&form).unwrap();
    body.append_child(&form_container).unwrap();
}

#[derive(Deserialize)]
struct LoginResponse {
    is_authenticated: bool,
    token: String,
}

fn prepare_request_body(document: &Document) -> (String, String) {
    let form = document.get_element_by_id("login-form")
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

    let request_body = serde_json::json!({
        "name": username,
        "password": password
    }).to_string();

    (request_body, username)
}

pub fn setup_register_button(
    document: &Document,
    body: &Element,
    window: &web_sys::Window,
) {
    let document_clone = document.clone();
    let window_clone = window.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        // create request
        let client = Client::new();
        let (request_body, _) = prepare_request_body(&document_clone);
        web_sys::console::log_1(&request_body.clone().into());
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let window_clone = window_clone.clone();

        spawn_local(async move {
            let res = client.post("http://127.0.0.1:8006/create_account")
                .headers(headers)
                .body(request_body.to_string())
                .send()
                .await;
            match res {
                Ok(response) => {
                    let text = response.text().await;
                    match text {
                        Ok(body) => {
                            match serde_json::from_str::<models::CreateAccountResponse>(&body) {
                                Ok(create_account_response) => {
                                    if create_account_response.success {
                                        window_clone.location().set_href("/account.html").unwrap();
                                        window_clone.alert_with_message("Account created successfully.").unwrap();
                                    } else {
                                        window_clone.location().set_href("/account.html").unwrap();
                                        window_clone.alert_with_message("Account creation failed. Nickname already taken.").unwrap();
                                    }
                                }
                                Err(err) => {
                                    window_clone.alert_with_message("Server error.").unwrap();
                                    web_sys::console::log_1(&format!("Error parsing JSON: {:?}", err).into());
                                }
                            }
                        },
                        Err(err) => {
                            window_clone.alert_with_message("Server error.").unwrap();
                            web_sys::console::log_1(&format!("Error reading response body: {:?}", err).into());
                        }
                    }
                }
                Err(err) => {
                    window_clone.alert_with_message("Server error.").unwrap();
                    web_sys::console::log_1(&format!("Request error: {:?}", err).into());
                }
            }
        });
    }) as Box<dyn FnMut(_)>);
    let register_button = document.get_element_by_id("register-button").unwrap();
    register_button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).unwrap();
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
        // create request
        let storage = window_clone.local_storage().unwrap().unwrap();
        let client = Client::new();
        let (request_body, username) = prepare_request_body(&document_clone);
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let window_clone = window_clone.clone();

        spawn_local(async move {
            // send request
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
                                // handle response
                                web_sys::console::log_1(&body.clone().into());
                                match serde_json::from_str::<LoginResponse>(&body) {
                                    Ok(login_response) => {
                                        web_sys::console::log_1(&format!("isAuthenticated: {}", login_response.is_authenticated).into());
                                        web_sys::console::log_1(&format!("cookie: {}", login_response.token).into());
                                        if login_response.is_authenticated {
                                            storage.set_item("token", &login_response.token).unwrap();
                                            storage.set_item("name", &username).unwrap();
                                            window_clone.location().set_href("/account.html").unwrap();
                                        } else {
                                            window_clone.alert_with_message("Authentication failed. Please check your username and password.").unwrap();
                                        }                                    
                                    }
                                    Err(err) => {
                                        window_clone.alert_with_message("Server error.").unwrap();
                                        web_sys::console::log_1(&format!("Error parsing JSON: {:?}", err).into());
                                    }
                                }
                            }
                            Err(err) => {
                                window_clone.alert_with_message("Server error.").unwrap();
                                web_sys::console::log_1(&format!("Error reading response body: {:?}", err).into());
                            }
                        }
                    }
                    Err(err) => {
                        window_clone.alert_with_message("Server error.").unwrap();
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

    // loguout
    let logout_button = html!(document, "button", {
        "id" => "logoutButton",
        "class" => "btn btn-primary mt-3",
        "style" => "float: right; margin-right: 10px;"
    }, "Logout");
    body.append_child(&logout_button).unwrap();

    // welcome message
    let name = window().unwrap().local_storage().unwrap().unwrap().get_item("name").unwrap().unwrap();
    let div = html!(document, "div", {
        "style" => "font-size: 48px; margin-top: 20px; margin-left: 20px; color: lightblue; font-family: Arial, sans-serif;"
    }, &format!("Hello, {}!", name));
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

pub fn go_back_to_main_page(
    document: &Document,
    body: &Element
) {
    let main_page_link = html!(document, "a", {
        "href" => "/main.html",
        "style" => "position: fixed; bottom: 20px; left: 50%; transform: translateX(-50%);"
    }, "Go back to main page");
    body.append_child(&main_page_link).unwrap();
}
