use rdev::{listen, Event, EventType, Key};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    menu::{Menu, MenuItem},
    TrayIconBuilder,
};

fn main() {
    // 1. Locate Resources folder
    let mut resources_path = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("."));
    resources_path.pop();
    if resources_path.ends_with("MacOS") {
        resources_path.pop();
        resources_path.push("Resources");
    } else {
        // Fallback to current directory for cargo run
        resources_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    }

    // 2. Load all audio files into memory
    let mut clicks: Vec<Arc<Vec<u8>>> = Vec::new();
    for i in 1..=4 {
        let filename = format!("click{}.wav", i);
        let path = resources_path.join(&filename);
        let bytes = std::fs::read(&path).unwrap_or_else(|_| {
            println!("Warning: {} not found at {:?}", filename, path);
            vec![]
        });
        clicks.push(Arc::new(bytes));
    }

    let enter_bytes = Arc::new(std::fs::read(resources_path.join("enter.wav")).unwrap_or_else(|_| {
        println!("Warning: enter.wav not found");
        vec![]
    }));
    let cmd_bytes = Arc::new(std::fs::read(resources_path.join("cmd.wav")).unwrap_or_else(|_| {
        println!("Warning: cmd.wav not found");
        vec![]
    }));

    // 3. Dedicated audio thread with mpsc channel
    // The keyboard callback just sends bytes; the audio thread plays them.
    let (tx, rx) = mpsc::channel::<Vec<u8>>();

    thread::spawn(move || {
        // Audio output lives entirely in this thread → no Send/lifetime issues
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        println!("Audio stream initialized.");
        while let Ok(bytes) = rx.recv() {
            if bytes.is_empty() { continue; }
            println!("Received {} bytes to play.", bytes.len());
            let cursor = Cursor::new(bytes);
            match Decoder::new(cursor) {
                Ok(decoder) => {
                    match Sink::try_new(&stream_handle) {
                        Ok(sink) => {
                            sink.append(decoder);
                            sink.detach();
                            println!("Playing sound...");
                        }
                        Err(e) => println!("Sink error: {:?}", e),
                    }
                }
                Err(e) => println!("Decoder error: {:?}", e),
            }
        }
    });

    // 4. Keyboard listener thread — sends cloned bytes via channel
    let tx_kbd = tx.clone();
    thread::spawn(move || {
        println!("Keyboard listener started.");
        let callback = move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                println!("Key pressed: {:?}", key);
                let bytes: Vec<u8> = match key {
                    Key::Return => enter_bytes.as_ref().clone(),
                    Key::MetaLeft | Key::MetaRight => cmd_bytes.as_ref().clone(),
                    _ => {
                        if clicks.is_empty() { return; }
                        let idx = (rand::random::<u32>() as usize) % clicks.len();
                        clicks[idx].as_ref().clone()
                    }
                };
                let _ = tx_kbd.send(bytes);
            }
        };
        if let Err(e) = listen(callback) {
            println!("rdev error: {:?}", e);
        }
    });

    // 5. Tray icon + event loop (must be on main thread on macOS)
    let event_loop = EventLoopBuilder::new().build();
    let tray_menu = Menu::new();
    let quit_i = MenuItem::new("Quit", true, None);
    tray_menu.append_items(&[&quit_i]).unwrap();

    let mut _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Pepe Keyboard")
        .with_icon(create_dummy_icon())
        .build()
        .unwrap();

    let menu_channel = tray_icon::menu::MenuEvent::receiver();
    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_i.id() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}

fn create_dummy_icon() -> tray_icon::Icon {
    let width = 32;
    let height = 32;
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    for _ in 0..(width * height) {
        rgba.push(255); // R
        rgba.push(0);   // G
        rgba.push(0);   // B
        rgba.push(255); // A
    }
    tray_icon::Icon::from_rgba(rgba, width, height).unwrap()
}
