use lopdf::Document;

fn main() {
    let doc = Document::load("graph.pdf").unwrap();
    println!("{:?}", doc.get_pages());

    for page in doc.page_iter() {
        println!("on page: {:?}", page);
        let content = doc.get_and_decode_page_content(page).unwrap();
        println!("{:?}", content);
    }
}
