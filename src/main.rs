use tera::{Tera, Context};

fn main() {
    let tera = Tera::new("*.html").unwrap();
    let cx = Context::new();
    let rendered_a = tera.render("page_a.html", &cx);
    let rendered_b = tera.render("page_b.html", &cx);

    // A: Err(Error { kind: Msg("Failed to render \'page_a.html\': error while rendering macro `level1::inner`"), source: Some(Error { kind: Msg("Macro `self::deeper` not found in template `page_a.html`"), source: None }) })
    println!("A: {:?}", rendered_a);
    // B: Ok("\n\n\n\nhello from b\n\n\n\n\n")
    println!("B: {:?}", rendered_b);
}
