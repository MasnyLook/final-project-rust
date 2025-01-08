use web_sys::{Document, Element};
use crate::fetch_history;

use crate::html;

pub fn create_main_page(document: &Document, body: &Element) {
    let main_page = html!(document, "div", {
        "id" => "mainpage",
        "class" => "text-center mt-4"
    });

    let header = document.create_element("h2").unwrap();
    header.set_inner_html("Welcome to the <em>Guess the Number</em> game!");
    header.set_attribute("class", "mb-4").unwrap();
    main_page.append_child(&header).unwrap();

    let games_list_description = html!(document, "p", {
        "class" => "mb-4 text-muted"
    }, "This is the list of all games.");
    main_page.append_child(&games_list_description).unwrap();

    let games_list = html!(document, "ul", {
        "class" => "list-unstyled"
    });

    let games = vec![("gcd", "index.html"), ("dividers", "dividers")];

    for (name, url) in games {
        let list_item = html!(document, "li", {
            "class" => "mb-2"
        });

        let link = html!(document, "a", {
            "href" => url,
            "class" => "text-primary"
        }, name);

        list_item.append_child(&link).unwrap();
        games_list.append_child(&list_item).unwrap();
    }

    main_page.append_child(&games_list).unwrap();

    let footer = html!(document, "div", {
        "style" => "position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); text-align: center; padding: 10px;"
    });
    fetch_history::fetch_leaderboard(document, &footer);

    body.append_child(&footer).unwrap();
    body.append_child(&main_page).unwrap();
}
