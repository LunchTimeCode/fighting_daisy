use rocket::{Route, get, http::ContentType, routes};

#[get("/<asset>")]
pub fn assets(asset: &str) -> (ContentType, Vec<u8>) {
    let file = fighting_daisy::assets::read_any_file(asset);
    let bytes = file.contents();

    let file_type = asset.split('.').last().unwrap();

    let ct: ContentType = match file_type {
        "js" => ContentType::JavaScript,
        "css" => ContentType::CSS,
        "html" => ContentType::HTML,
        "png" => ContentType::PNG,
        "svg" => ContentType::SVG,
        "json" => ContentType::JSON,
        "xml" => ContentType::XML,
        "msgpack" => ContentType::MsgPack,
        "txt" => ContentType::Plain,
        "ico" => ContentType::Icon,
        _ => {
            return (
                ContentType::Plain,
                "Unexpected type requested".as_bytes().to_vec(),
            );
        }
    };
    (ct, bytes.to_vec())
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/_assets", routes![assets])
}
