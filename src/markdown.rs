/// This code is heavily inspired by the Yew example [https://github.com/yewstack/yew/blob/master/examples/futures/src/markdown.rs]
///
/// Right now we're calling JS to colorize the code, there's nothing stopping us
/// From bringing this logic to rust if we'd like to, all it takes is the implementation of it.
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use yew::{
    html,
    services::ConsoleService,
    virtual_dom::{VNode, VTag, VText},
    Html,
};

pub fn generate_html(markdown: &str) -> Html {
    let mut res_tree = vec![];
    let mut tree = vec![];

    // Instead of repeating this code or exposing it as a mmethod taking the 'tree'
    // this is a simple way of generating the code we need in its stead.
    macro_rules! add_child {
        ($child:expr) => {{
            let l = tree.len();
            assert_ne!(l, 0);
            tree[l - 1].add_child($child);
        }};
    }

    for event in Parser::new_ext(&markdown, Options::empty()) {
        match event {
            Event::Start(t) => tree.push(to_vtag(t)),
            Event::End(t) => {
                let len = tree.len();
                assert!(len >= 1);
                let mut top = tree.pop().unwrap();
                if let Tag::CodeBlock(_) = t {
                    let mut pre = VTag::new("div");
                    // pre.add_attribute("class", &"container");
                    pre.add_child(top.into());
                    top = pre;
                }
                if len == 1 {
                    res_tree.push(top);
                } else {
                    tree[len - 2].add_child(top.into());
                }
            }
            Event::Code(c) => {
                ConsoleService::log(&format!("Text: {}", &c.into_string()));
            }
            Event::Text(text) => {
                // We've added the 'Code' tag in the to_tag method
                let text = text.into_string();

                if tree[tree.len() - 1].tag() == "pre" {
                    let colorized = colorize(&text);
                    let span = yew::utils::document().create_element("code").unwrap();

                    span.set_inner_html(&colorized);
                    add_child!(VNode::VRef(span.into()).into());
                } else {
                    add_child!(VText::new(text).into());
                }
            }
            Event::SoftBreak => {
                add_child!(VText::new("\n".into()).into());
            }
            Event::HardBreak => {
                add_child!(VTag::new("br").into());
            }
            Event::Rule => {
                add_child!(VTag::new("hr").into());
            }
            _ => println!("Skipping event"),
        }
    }

    if res_tree.len() == 1 {
        VNode::VTag(Box::new(res_tree.pop().unwrap()))
    } else {
        html! {
            <div> { for res_tree.into_iter() } </div>
        }
    }
}

// Generate a VTag element for later rendering to the actual dom
// Use VTag over string to skip string -> Html later
fn to_vtag(tag: Tag) -> VTag {
    use Tag::*;
    match tag {
        Paragraph => {
            let mut v = VTag::new("p");
            v.add_attribute("class", &"title");
            v
        }
        Heading(s) => {
            let mut v = VTag::new("p");
            v.add_attribute("class", &"");
            v
        }
        BlockQuote => {
            let mut el = VTag::new("blockquote");
            el.add_attribute("class", &"blockquote");
            el
        }
        CodeBlock(code_type) => {
            let mut el = VTag::new("pre");
            if let CodeBlockKind::Fenced(lang) = code_type {
                match lang.as_ref() {
                    "rust" => el.add_attribute("class", &"language-rust"),
                    _ => {}
                };
            }
            el
        }
        List(None) => VTag::new("ul"),
        List(Some(1)) => VTag::new("ol"),
        List(Some(ref start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start);
            el
        }
        Item => VTag::new("li"),
        FootnoteDefinition(ref _footnote_id) => VTag::new("span"),
        Table(_) => {
            let mut el = VTag::new("table");
            el.add_attribute("class", &"table");
            el
        }
        TableHead => VTag::new("th"),
        TableRow => VTag::new("tr"),
        TableCell => VTag::new("td"),
        Emphasis => {
            let mut el = VTag::new("span");
            el.add_attribute("class", &"font-italic");
            el
        }
        Strong => {
            let mut el = VTag::new("span");
            el.add_attribute("class", &"font-weight-bold");
            el
        }
        Link(_link_type, ref href, ref title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href);
            if !title.is_empty() {
                el.add_attribute("title", &title);
            }
            el
        }
        Image(_link_type, ref src, ref title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src);
            if !title.is_empty() {
                el.add_attribute("title", &title);
            }
            el
        }
        Strikethrough => {
            let mut el = VTag::new("span");
            el.add_attribute("class", &"text-decoration-strikethrough");
            el
        }
    }
}

fn colorize(code: &str) -> String {
    let m = highlight(code);
    if let Some(res) = m.as_string() {
        res
    } else {
        "Hello non-js world!".into()
    }
}

#[wasm_bindgen]
extern "C" {
    fn highlight(html: &str) -> JsValue;
}
