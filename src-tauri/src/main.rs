// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),

    windows_subsystem = "windows"
)]

// Menus dont have shortcuts, only submenus:
// https://github.com/tauri-apps/tauri/issues/4085

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn create_app_menu() -> Menu {
    let file_menu = Menu::new()
        .add_item(CustomMenuItem::new("new".to_string(), "New").accelerator("Ctrl+N"))
        .add_item(
            CustomMenuItem::new("open".to_string(), "Open").accelerator("CmdOrCtrl+O"),
        )
        .add_native_item(MenuItem::Separator)
        .add_item(
            CustomMenuItem::new("save".to_string(), "Save").accelerator("CmdOrCtrl+S"),
        );

    let view_menu = Menu::new()
        .add_item(CustomMenuItem::new("alpha".to_string(), "Alpha").accelerator("CmdOrCtrl+I"))
        .add_item(CustomMenuItem::new("beta".to_string(), "Beta").accelerator("CmdOrCtrl+T"));

    Menu::new()
        .add_submenu(Submenu::new("File", file_menu))
        .add_submenu(Submenu::new("View", view_menu))
        .add_submenu(Submenu::new("App", Menu::new().add_native_item(MenuItem::Quit)))
}

fn main() {
    tauri::Builder::default()
        .menu(create_app_menu())
        .on_menu_event(|event| match event.menu_item_id() {
            "new" => {
                println!("New menu item clicked!");
                event.window().emit("new-content", "").unwrap();
            }
            "open" => {
                println!("Open menu item clicked!");
                event.window().emit("open-file", "").unwrap();
            }
            "save" => {
                println!("Save menu item clicked!");
                event.window().emit("save-content", "").unwrap();
            }
            "alpha" => {
                println!("Alpha menu item clicked!");
                event.window().emit("switch-page", "alpha").unwrap();
            }
            "beta" => {
                println!("Beta menu item clicked!");
                event.window().emit("switch-page", "beta").unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//  1. 
    // #[macro_use]
    // extern crate lazy_static;

        // #[derive(Serialize, Deserialize)]
        // struct Poll {
        //     id: String,
        //     title: String,
        //     description: String,
        //     x: Vec<String>,
        //     y: Option<Vec<String>>,
        //     responses: Vec<PollResponse>,
        // }

        // #[derive(Serialize, Deserialize)]
        // struct PollResponse {
        //     name: String,
        //     selections: Vec<Selection>,
        // }

        // #[derive(Serialize, Deserialize)]
        // struct Selection {
        //     x: String,
        //     y: Option<String>,
        //     selection: String,
        // }

        // #[derive(Serialize, Deserialize)]
        // struct CreatePollRequest {
        //     title: String,
        //     description: String,
        //     x: Vec<String>,
        //     y: Option<Vec<String>>,
        // }

        // lazy_static! {
        //     static ref POLLS: Mutex<Vec<Poll>> = Mutex::new(Vec::new());
        // }

        // #[get("/poll")]
        // async fn get_polls() -> impl Responder {
        //     HttpResponse::Ok().json(&POLLS.lock().unwrap().deref())
        // }

        // #[post("/poll")]
        // async fn create_poll(req_body: String) -> impl Responder {
        //     let r: Result<CreatePollRequest, serde_json::Error> = serde_json::from_str(&req_body);
        //     match r {
        //         Ok(req) => {
        //             let id = Uuid::new_v4().to_string();
        //             let poll: Poll = Poll {
        //                 id: id.clone(),
        //                 title: req.title,
        //                 description: req.description,
        //                 x: req.x,
        //                 y: req.y,
        //                 responses: Vec::new(),
        //             };
        //             POLLS.lock().unwrap().push(poll);
        //             HttpResponse::Ok().body(id)
        //         }
        //         Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        //     }
        // }

        // #[get("/poll/{id}")]
        // async fn get_poll(path: web::Path<String>) -> impl Responder {
        //     let id = path.into_inner();
        //     let polls = POLLS.lock().unwrap();
        //     let poll: Option<&Poll> = polls.iter().find(|&p| p.id == id);

        //     match poll {
        //         Some(p) => HttpResponse::Ok().json(p),
        //         None => HttpResponse::NotFound().body(format!("No poll with id {id}")),
        //     }
        // }

        // #[post("/poll/respond/{id}")]
        // async fn respond_to_poll(path: web::Path<String>, req_body: String) -> impl Responder {
        //     let id = path.into_inner();
        //     let mut polls = POLLS.lock().unwrap();
        //     let poll_index: Option<usize> = polls.iter().position(|p| p.id == id);

        //     match serde_json::from_str(&req_body) {
        //         Ok(response) => match poll_index {
        //             Some(index) => {
        //                 polls[index].responses.push(response);
        //                 HttpResponse::Ok().finish()
        //             }
        //             None => HttpResponse::NotFound().body(format!("No poll with id {id}")),
        //         },
        //         Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        //     }
        // }



// 1. (SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, WindowBuilder, WindowUrl)-approach implemented

//     #![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

//     use tauri::{ Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, WindowBuilder, WindowUrl };

//     fn main() {
//         let system_tray_menu = SystemTrayMenu::new()
//             .add_item(CustomMenuItem::new("alpha".to_string(), "Show Alpha Window"))
//             .add_item(CustomMenuItem::new("beta".to_string(), "Show Beta Window"))
//             .add_native_item(SystemTrayMenuItem::Quit);

//         tauri::Builder::default()
//             .setup(|app| {
//                 let alpha_window = WindowBuilder::new(app, "alpha".to_string(), WindowUrl::default())
//                     .title("Alpha Window")
//                     .build()
//                     .unwrap();

//                 let beta_window = WindowBuilder::new(app, "beta".to_string(), WindowUrl::default())
//                     .title("Beta Window")
//                     .build()
//                     .unwrap();

//                 let system_tray = app.system_tray().build();
//                 system_tray.set(system_tray_menu).enable().unwrap();

//                 let alpha_window_clone = alpha_window.clone();
//                 let beta_window_clone = beta_window.clone();

//                 system_tray.on_event(move |event| match event {
//                     SystemTrayEvent::MenuItemClick { id, .. } => {
//                         if id == "alpha" {
//                             alpha_window_clone.show().unwrap();
//                             beta_window_clone.hide().unwrap();
//                         } else if id == "beta" {
//                             alpha_window_clone.hide().unwrap();
//                             beta_window_clone.show().unwrap();
//                         }
//                     }
//                     _ => {}
//                 });

//                 Ok(())
//             })
//             .run(tauri::generate_context!())
//             .expect("error while running tauri application");
//     }

//     struct CustomMenuItem {
//         id: String,
//         label: String,
//     }

//     impl CustomMenuItem {
//         fn new(id: String, label: String) -> Self {
//             Self { id, label }
//         }
//     }

//     impl<'a> From<CustomMenuItem> for SystemTrayMenuItem<'a> {
//         fn from(item: CustomMenuItem) -> Self {
//             SystemTrayMenuItem::CustomMenuItem(item.id, item.label)
//         }
//     }

// 2. Window-builder implemented

// 3. Global Shortcut Manager implemented
//     #![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
//     use tauri::{GlobalShortcutManager, Manager, WindowBuilder, WindowUrl};

//     fn main() {
//     tauri::Builder::default()
//         .setup(|app| {
//             let alpha_window = WindowBuilder::new(app, "alpha".to_string(), WindowUrl::default())
//                 .title("Alpha Window")
//                 .build()
//                 .unwrap();

//             let beta_window = WindowBuilder::new(app, "beta".to_string(), WindowUrl::default())
//                 .title("Beta Window")
//                 .build()
//                 .unwrap();

//             let mut shortcut_manager = app.global_shortcut_manager();
//             let shortcut_manager_for_a = shortcut_manager.register("CmdOrCtrl+A", move || {
//                 alpha_window.show().unwrap();
//                 beta_window.hide().unwrap();
//             });

//             if let Err(err) = shortcut_manager_for_a {
//                 eprintln!("Error registering shortcut A: {:?}", err);
//             }

//             let shortcut_manager_for_b = shortcut_manager.register("CmdOrCtrl+B", move || {
//                 alpha_window.hide().unwrap();
//                 beta_window.show().unwrap();
//             });

//             if let Err(err) = shortcut_manager_for_b {
//                 eprintln!("Error registering shortcut B: {:?}", err);
//             }

//             Ok(())
//         })
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
//     }

// 4. SystemTray Function used n works but opens 3 windows:

//     #![cfg_attr( all(not(debug_assertions), target_os = "windows"),windows_subsystem = "windows" )]
//     use tauri::{ CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowBuilder, WindowUrl, };

//     fn main() {
//         let system_tray_menu = SystemTrayMenu::new()
//             .add_item(CustomMenuItem::new("show_alpha".to_string(), "Show Alpha Window"))
//             .add_item(CustomMenuItem::new("show_beta".to_string(), "Show Beta Window"));

//         tauri::Builder::default()
//             .setup(|app| {
//                 let alpha_window = WindowBuilder::new(app, "alpha".to_string(), WindowUrl::default())
//                     .title("Alpha Window")
//                     .build()
//                     .unwrap();

//                 let beta_window = WindowBuilder::new(app, "beta".to_string(), WindowUrl::default())
//                     .title("Beta Window")
//                     .build()
//                     .unwrap();

//                 let system_tray = SystemTray::new().with_menu(system_tray_menu);

//                 let alpha_window_clone = alpha_window.clone();
//                 let beta_window_clone = beta_window.clone();

//                 system_tray.on_event(move |event| match event {
//                     SystemTrayEvent::MenuItemClick { id, .. } => {
//                         if id == "show_alpha" {
//                             alpha_window_clone.show().unwrap();
//                             beta_window_clone.hide().unwrap();
//                         } else if id == "show_beta" {
//                             alpha_window_clone.hide().unwrap();
//                             beta_window_clone.show().unwrap();
//                         }
//                     }
//                     _ => {}
//                 });

//                 Ok(())
//             })
//             .run(tauri::generate_context!())
//             .expect("error while running tauri application");
//     }

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn alphaa() {

//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }

