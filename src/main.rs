// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::str::FromStr;
use std::thread;
use serde::Serialize;
use id3::{Tag, TagLike};
use base64::prelude::*;
use urlencoding::decode;
use tiny_http::{Server, Response};

const MUSIC_DIR : &str = "PATH TO MP3 FOLDER";

fn main() {

    thread::spawn(|| {
        let server = Server::http("127.0.0.1:8000").unwrap();

        for request in server.incoming_requests() {

            let file_name: String = format!("{}/{}", MUSIC_DIR, decode(request.url()).unwrap());

            match std::fs::File::open(file_name){
                Ok(file) => {
                    let mut response: Response<fs::File> = Response::from_file(file);
                    response.add_header(tiny_http::Header::from_str("Access-Control-Allow-Origin:*").unwrap());
                    request.respond(response).unwrap();
                },
                Err(_e) => {}
            }
        
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ get_songs ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_songs() -> Vec<Song>{
    let mut songs : Vec<Song> = Vec::new();

    let dir = fs::read_dir(MUSIC_DIR).unwrap();

    for result in dir {

        let entry = result.unwrap();

        let file_name = entry.file_name().to_string_lossy().to_string();

        if entry.file_type().unwrap().is_file() && file_name.ends_with(".mp3"){

            let path = format!("{}/{}", MUSIC_DIR,file_name);

            let tag = Tag::read_from_path(&path).unwrap();

            let title = tag.title().unwrap().to_string();

            let artist = tag.artist().or_else(|| { Some("Unknown") }).unwrap().to_string();

            let mut pictures = tag.pictures();

            let mut image : String = String::new();

            if let Some(picture) = pictures.next(){
                image = BASE64_STANDARD.encode(&picture.data);
            }

            let song = Song {
                file_name,
                title,
                artist,
                image
            };

            songs.push(song);
        }
    }

    songs
}

#[derive(Serialize, Debug)]
struct Song{
    file_name : String,
    title : String,
    artist : String,
    image : String
}