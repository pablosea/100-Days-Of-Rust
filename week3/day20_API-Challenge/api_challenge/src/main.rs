use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let suggestions = Arc::new(Mutex::new(get_suggestions()));
    for stream in listener.incoming() {
        let suggestions_clone = Arc::clone(&suggestions);
        thread::spawn(move || handle_connection(stream.unwrap(), suggestions_clone));
    }
}

#[derive(Clone)]
struct Suggestion {
    name: String,
    latitude: String,
    longitude: String,
    score: f64,
}

fn get_suggestions() -> Vec<Suggestion> {
    vec![
        Suggestion {
            name: "London, ON, Canada".to_string(),
            latitude: "42.98339".to_string(),
            longitude: "-81.23304".to_string(),
            score: 0.9,
        },
        Suggestion {
            name: "London, OH, USA".to_string(),
            latitude: "39.88645".to_string(),
            longitude: "-83.44825".to_string(),
            score: 0.5,
        },
    ]
}

fn handle_connection(mut stream: TcpStream, suggestions: Arc<Mutex<Vec<Suggestion>>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET /suggestions";
    if buffer.starts_with(get) {
        let request = String::from_utf8_lossy(&buffer[..]);
        let query_string = request.split_whitespace().nth(1).unwrap().split('?').nth(1).unwrap_or("");
        let query_params: HashMap<_, _> = query_string.split('&')
            .filter_map(|s| {
                let mut split = s.split('=');
                Some((split.next()?, split.next()?))
            })
            .collect();

        let q = query_params.get("q").unwrap_or(&"");
        let latitude = query_params.get("latitude").and_then(|s| s.parse::<f64>().ok());
        let longitude = query_params.get("longitude").and_then(|s| s.parse::<f64>().ok());

        let suggestions = suggestions.lock().unwrap();
        let mut filtered_suggestions: Vec<Suggestion> = suggestions.iter()
            .filter(|s| s.name.to_lowercase().contains(&q.to_lowercase()))
            .cloned()
            .collect();

        if let (Some(lat), Some(lon)) = (latitude, longitude) {
            for suggestion in &mut filtered_suggestions {
                let suggestion_lat = suggestion.latitude.parse::<f64>().unwrap();
                let suggestion_lon = suggestion.longitude.parse::<f64>().unwrap();
                let distance = ((lat - suggestion_lat).powi(2) + (lon - suggestion_lon).powi(2)).sqrt();
                suggestion.score /= distance + 1.0; // Adjust score based on distance
            }
        }

        filtered_suggestions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        let mut json_response = String::from("[");
        for (i, suggestion) in filtered_suggestions.iter().enumerate() {
            if i > 0 {
                json_response.push(',');
            }
            json_response.push_str(&format!(
                "{{\"name\":\"{}\",\"latitude\":\"{}\",\"longitude\":\"{}\",\"score\":{}}}",
                suggestion.name, suggestion.latitude, suggestion.longitude, suggestion.score
            ));
        }
        json_response.push(']');

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
            json_response
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write(status_line.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}


