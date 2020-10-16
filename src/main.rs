#![windows_subsystem="windows"]

fn main() {
    web_view::builder()
        .title("Hello world!")
        .content(web_view::Content::Html(HTML))
        .size(320, 240)
        .user_data(())
        .invoke_handler(|_, _| Ok(()))
        .run()
        .unwrap();
}

const HTML: &str = r#"<!DOCTYPE html>
<html>
    <body>
        Hello world from Rust!
    </body>
</html>"#;
