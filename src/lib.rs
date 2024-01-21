
pub fn success() {
    print!("{}", serde_json::json!({
        "code": 200,
        "message": "success"
    }));
}

pub fn error(data: String) {
    print!("{}", serde_json::json!({
        "code": 400,
        "message": data
    }));
}

pub fn result_string(data: Result<String, Box<dyn std::error::Error>>) {
    match data {
        Ok(data) => {
            print!("{}", serde_json::json!({
                "code": 200,
                "message": "success",
                "data": data
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": data.to_string()
            }));
        }
    }
}

pub fn result_value(data: Result<serde_json::Value, Box<dyn std::error::Error>>) {
    match data {
        Ok(data) => {
            print!("{}", serde_json::json!({
                "code": 200,
                "message": "success",
                "data": data
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": data.to_string(),
            }));
        }
    }
}


pub fn result(data: Result<(), Box<dyn std::error::Error>>) {
    match data {
        Ok(()) => {
            print!("{}", serde_json::json!({
                "code": 200,
                "message": "success",
            }));
        },
        Err(data) => {
            print!("{}", serde_json::json!({
                "code": 400,
                "message": data.to_string()
            }));
        }
    }
}