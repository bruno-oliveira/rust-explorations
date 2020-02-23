#[get("/<id>")]
fn personalized_greet(id: String) -> String {
    let filename = format!("Hello {id}", id = id);
    return filename
}