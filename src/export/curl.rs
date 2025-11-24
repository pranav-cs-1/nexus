use crate::models::request::HttpRequest;

pub fn export_to_curl(request: &HttpRequest) -> String {
    let mut curl_cmd = format!("curl -X {} '{}'", request.method.as_str(), request.full_url());
    
    for (key, value) in &request.headers {
        curl_cmd.push_str(&format!(" \\\n  -H '{}: {}'", key, value));
    }
    
    if let Some(body) = &request.body {
        curl_cmd.push_str(&format!(" \\\n  -d '{}'", body.replace("'", "'\\''")));
    }
    
    curl_cmd
}

