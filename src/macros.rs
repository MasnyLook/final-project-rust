#[macro_export]
macro_rules! html {
    ($document:expr, $tag:expr, { $($attr:expr => $value:expr),* }, $text:expr) => {{
        let element = $document.create_element($tag).unwrap();
        $(
            element.set_attribute($attr, $value).unwrap();
        )*
        element.set_text_content(Some($text));
        element
    }};
    ($document:expr, $tag:expr, { $($attr:expr => $value:expr),* }) => {{
        let element = $document.create_element($tag).unwrap();
        $(
            element.set_attribute($attr, $value).unwrap();
        )*
        element
    }};
    ($document:expr, $tag:expr, $text:expr) => {{
        let element = $document.create_element($tag).unwrap();
        element.set_text_content(Some($text));
        element
    }};
}

#[macro_export]
macro_rules! set_element_style {
    ($document:expr, $id:expr, $style:expr) => {{
        $document.get_element_by_id($id).unwrap()
            .set_attribute("style", $style).unwrap();
    }};
}

#[macro_export]
macro_rules! set_element_text {
    ($document:expr, $id:expr, $text:expr) => {{
        $document.get_element_by_id($id).unwrap()
            .set_text_content(Some($text));
    }};
}