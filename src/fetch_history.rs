use web_sys::{Document, Element};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use crate::models;
use chrono::{DateTime, NaiveDateTime, Utc};
use reqwest::Client;
use reqwest::header;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::html;

pub fn fetch_user_board(
    document: &Document,
    body: &Element,
) {
    let window = web_sys::window().unwrap();
    let message = prepare_request_body(&window);
    let document_clone = document.clone();
    let window_clone = window.clone();
    let body_clone = body.clone();
    spawn_local(async move {
        let response = send_message_to_server(&window_clone, &message).await;
        match response {
            Ok(data) => {
                let results = parse_output_data(&data);
                match results {
                    Ok(game_results) => {
                        create_table_from_results(&document_clone, &body_clone, game_results, false);
                    }
                    Err(err) => {
                        window_clone.alert_with_message("Error fetching user board").unwrap();
                        web_sys::console::log_1(&format!("Error parsing JSON: {:?}", err).into());
                    }
                }
            }
            Err(err) => {
                window_clone.alert_with_message("Error fetching user board").unwrap();
                web_sys::console::log_1(&format!("Request error: {:?}", err).into());
            }
        }
    });
}

pub fn fetch_leaderboard(
    document: &Document,
    body: &Element,
) {
    let window = web_sys::window().unwrap();
    let document_clone = document.clone();
    let window_clone = window.clone();
    let body_clone = body.clone();
    spawn_local(async move {
        let response = send_getmessage_to_server(&window_clone).await;
        match response {
            Ok(data) => {
                let results = parse_output_data(&data);
                match results {
                    Ok(game_results) => {
                        create_table_from_results(&document_clone, &body_clone, game_results, true);
                    }
                    Err(err) => {
                        window_clone.alert_with_message("Error fetching leaderboard").unwrap();
                        web_sys::console::log_1(&format!("Error parsing JSON: {:?}", err).into());
                    }
                }
            }
            Err(err) => {
                window_clone.alert_with_message("Error fetching leaderboard").unwrap();
                web_sys::console::log_1(&format!("Request error: {:?}", err).into());
            }
        }
    });
}

fn prepare_request_body (
    window: &web_sys::Window,
) -> String {
    let storage = window.local_storage().unwrap().unwrap();
    let token = storage.get_item("token").unwrap().unwrap();
    let name = storage.get_item("name").unwrap().unwrap();
    let auth_token = models::AuthenticationToken {
        user_name: name,
        cookie: token,
    };
    let request_body = serde_json::to_string(&auth_token).unwrap();
    web_sys::console::log_1(&JsValue::from_str(&request_body));
    request_body
}


async fn send_message_to_server(window: &web_sys::Window, message: &str) -> Result<String, reqwest::Error>  {
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let request_body = message.to_string();

    let response = client
        .post("http://127.0.0.1:8006/user_board")
        .headers(headers)
        .body(request_body)
        .send()
        .await?;
            
    let text = response.text().await?;
    web_sys::console::log_1(&JsValue::from_str(&text));

    Ok(text)
}

async fn send_getmessage_to_server(window: &web_sys::Window) -> Result<String, reqwest::Error>  {
    let client = Client::new();

    let response = client
        .get("http://127.0.0.1:8006/leaderboard")
        .send()
        .await?;
            
    let text = response.text().await?;
    web_sys::console::log_1(&JsValue::from_str(&text));

    Ok(text)
}

fn parse_output_data(data: &str) -> Result<Vec<models::GameResult>, serde_json::Error> {
    let game_results: Vec<models::GameResult> = serde_json::from_str(data)?;
    Ok(game_results)
}

pub fn create_table_from_results(document: &Document, body: &Element, results: Vec<models::GameResult>, is_username: bool) {
    let table = html!(document, "table", {
        "class" => "table table-striped",
        "style" => "width: auto; margin: 0 auto; border: 1px solid grey;"
    });

    // title 
    let caption = html!(document, "caption", {
        "style" => "caption-side: top; font-size: 24px; font-weight: bold; margin-bottom: 10px;"
    }, if is_username { "Leaderboard" } else { "History of Game Results" });
    table.append_child(&caption).unwrap();

    let thead = html!(document, "thead", {});
    let header_row = html!(document, "tr", {});

    let mut headers = vec!["Time(sec)", "Moves", "Game Type", "Timestamp"];
    if is_username {
        headers.insert(0, "Nick");
    }

    for header in headers {
        let th = html!(document, "th", {
            "style" => "color: lightblue; text-align: center;"
        }, header);
        header_row.append_child(&th).unwrap();
    }
    thead.append_child(&header_row).unwrap();
    table.append_child(&thead).unwrap();

    let tbody = html!(document, "tbody", {});
    for result in results {
        let row = html!(document, "tr", {
            "style" => "color: white; text-align: center;"
        });

        // add columns
        if is_username {
            let username_cell = html!(document, "td", {}, &result.user_name);
            row.append_child(&username_cell).unwrap();
        }

        let score_time_cell = html!(document, "td", {}, &result.score_time.to_string());
        row.append_child(&score_time_cell).unwrap();

        let score_moves_cell = html!(document, "td", {}, &result.score_moves.to_string());
        row.append_child(&score_moves_cell).unwrap();

        let game_type_cell = html!(document, "td", {}, &result.game_type);
        row.append_child(&game_type_cell).unwrap();

        let timestamp_cell = html!(document, "td", {}, &format_timestamp(result.timestamp.secs_since_epoch, result.timestamp.nanos_since_epoch));
        row.append_child(&timestamp_cell).unwrap();

        tbody.append_child(&row).unwrap();
    }
    table.append_child(&tbody).unwrap();

    body.append_child(&table).unwrap();
}

fn format_timestamp(secs_since_epoch: i64, nanos_since_epoch: i32) -> String {
    let naive = NaiveDateTime::from_timestamp(secs_since_epoch, nanos_since_epoch as u32);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
