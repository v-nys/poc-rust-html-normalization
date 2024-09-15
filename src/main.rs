use ego_tree;
use regex::Regex;
use scraper::{ElementRef, Html, Node};

fn normalize_whitespace(text: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(text, " ").to_string()
}

fn recurse(node: ego_tree::NodeRef<Node>, new_html: &mut String) {
    match node.value() {
        Node::Doctype(doctype) => {
            new_html.push_str(&format!("<!doctype {}>", doctype.name()));
        }
        Node::Comment(_) => {}
        Node::Element(elem) => {
            let elem_ref = ElementRef::wrap(node).expect("Specifically works in this case.");
            let tag = elem_ref.value().name();
            match tag {
                "pre" | "code" | "textarea" | "svg" => new_html.push_str(&elem_ref.html()),
                _ => {
                    new_html.push_str(&format!("<{}", tag));
                    // attributes include classes!
                    for (attr_name, attr_value) in elem.attrs() {
                        new_html.push_str(&format!(" {attr_name}=\"{attr_value}\""));
                    }
                    new_html.push_str(">");
                    for node in elem_ref.children() {
                        recurse(node, new_html);
                    }
                    new_html.push_str(&format!("</{}>", tag));
                }
            }
        }
        Node::Text(text) => {
            new_html.push_str(&normalize_whitespace(&text.to_string()));
        }
        Node::Document => {}
        Node::Fragment | Node::ProcessingInstruction(_) => {
            unimplemented!("these nodes are not supported");
        }
    }
}

fn main() {
    let html = r#"<!doctype html><html><head></head><body><p>Dit is een zin.
        

        Dit is nog een zin.</p>
        <pre>Deze tekst

        zou moeten       blijven</pre>
        <p>Dit is de derde <a class="a-link link-cls" href="www.ap.be">zin</a>.       En dit de vierde.</p>"#;
    let document = Html::parse_document(html);
    let mut new_html = String::new();
    for node in document.tree.root().children() {
        recurse(node, &mut new_html);
    }
    print!("{}", new_html);
}
