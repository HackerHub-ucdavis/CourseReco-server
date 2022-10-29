#[allow(dead_code)]
use xmltojson::to_json;

pub fn val2json(v: xmlrpc::Value) -> String {
    let mut buf = Vec::new();

    match v.write_as_xml(&mut buf) {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    };

    let str_xml = match std::str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let str_json = match to_json(str_xml) {
        Ok(v) => v,
        Err(_) => panic!("Invalid xml: {}", str_xml),
    };

    format!("{}", str_json)
}
