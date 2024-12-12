use web_sys::{Document, Element};

pub fn create_main_page(document: &Document, body: &Element) {
    let main_page = document.create_element("div").unwrap();
    main_page.set_attribute("id", "mainpage").unwrap();
    main_page
        .set_attribute("class", "text-center mt-4")
        .unwrap();

    let header = document.create_element("h2").unwrap();
    header.set_inner_html("Welcome to the <em>Guess the Number</em> game!");
    header.set_attribute("class", "mb-4").unwrap();
    main_page.append_child(&header).unwrap();

    let games_list_description = document.create_element("p").unwrap();
    games_list_description.set_text_content(Some("This is the list of all games."));
    games_list_description
        .set_attribute("class", "mb-4 text-muted")
        .unwrap();
    main_page.append_child(&games_list_description).unwrap();

    let games_list = document.create_element("ul").unwrap();
    games_list.set_attribute("class", "list-unstyled").unwrap();

    let games = vec![("gcd", "index.html"), ("dividers", "dividers")];

    for (name, url) in games {
        let list_item = document.create_element("li").unwrap();
        list_item.set_attribute("class", "mb-2").unwrap();

        let link = document.create_element("a").unwrap();
        link.set_attribute("href", url).unwrap();
        link.set_text_content(Some(name));
        link.set_attribute("class", "text-primary").unwrap();

        list_item.append_child(&link).unwrap();
        games_list.append_child(&list_item).unwrap();
    }

    main_page.append_child(&games_list).unwrap();
    body.append_child(&main_page).unwrap();
}
