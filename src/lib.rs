use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let path = req.path();
    let mut key = &path[1..];

    let bucket = match env.bucket("R2") {
        Ok(b) => b,
        Err(e) => {
            console_error!("missing R2 Binding ({})", e);
            return Response::error("Internal Error", 500);
        }
    };

    // visit homepage entry as default if missing key
    let entrance_value;
    if key.eq("") {
        key = match env.var("ENTRANCE") {
            Ok(entrance) => {
                entrance_value = entrance.to_string();
                entrance_value.as_str()
            }
            Err(_) => "index.html"
        };
    }

    // access to R2 object
    let object_result = bucket.get(key).execute().await;
    let object_option = match object_result {
        Ok(option) => option,
        Err(e) => {
            console_error!("missing object with key {} ({})", key, e);
            return Response::error("Object Not Found", 404);
        }
    };
    let object = match object_option {
        Some(o) => o,
        None => {
            console_error!("error object with key {}", key);
            return Response::error("Object Not Found", 404);
        }
    };

    // prepare response headers
    let headers = build_response_headers(key, object.http_etag().as_str());
    object.write_http_metadata(headers.clone()).expect("error write metadata");

    // prepare response body
    let body_option = match object.body() {
        Some(option) => option,
        None => {
            console_error!("none object bytes with key {}", key);
            return Response::error("Object Not Found", 404);
        }
    };

    // return the response body
    match body_option.bytes().await {
        Ok(bs) => Ok(Response::from_bytes(bs)?.with_headers(headers)),
        Err(_) => Response::error("Object Not Found", 404)
    }
}

fn build_response_headers(key: &str, etag: &str) -> Headers {
    let mut headers = Headers::new();

    headers
        .set("Access-Control-Allow-Origin", "*")
        .expect("Unable to set Access-Control-Allow-Origin");
    headers
        .set("Access-Control-Allow-Methods", "GET,HEAD,POST,OPTIONS")
        .expect("Unable to set Access-Control-Allow-Methods");
    headers
        .set("Access-Control-Allow-Headers", "*")
        .expect("Unable to set Access-Control-Allow-Headers");
    headers
        .set("Access-Control-Max-Age", "86400")
        .expect("Unable to set Access-Control-Max-Age");

    headers
        .set("Vary", "Origin")
        .expect("Unable to set Vary");

    let guess = mime_guess::from_path(key);
    if let Some(raw_mime) = guess.first_raw() {
        headers
            .set("Content-Type", raw_mime)
            .expect("Unable to set Content-Type");
    }
    if key.eq("apple-app-site-association") {
        headers
            .set("Content-Type", "application/json")
            .expect("Unable to set Content-Type");
    }

    headers
        .set("etag", etag)
        .expect("Unable to set etag");

    headers
}
