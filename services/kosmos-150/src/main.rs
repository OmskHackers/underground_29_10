use std::sync::Arc;

use kosmos150::pkg::db::{self};
use kosmos150::services::services::FlightService;
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

use kosmos150::menu::menu::Menu;
use kosmos150::menu::menu::MenuTransaction::{Input, Output, Exit};
use kosmos150::network::session::Session;
use kosmos150::utils::date::UssrDateTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let art = format!(r#"
                ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⠹⣿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⠇⠀⢻⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠲⣶⣶⣶⣶⣶⣶⣶⣾⡿⠀⠀⠈⣿⣶⣶⣶⣶⣶⣶⣶⡶⠖⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣾⠟⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⣦⡄⠀⠀⠀⠀⠀⢠⣶⠿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⡿⠀⠀⠀⣀⠀⠀⠘⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡤⠄⠀⠀⠀⠀⠀⠀⢠⣿⠃⢀⣴⡾⠿⣷⣄⡀⢹⣷⠀⠀⠀⠀⠀⠀⠀⢦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⣀⣀⣴⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⣾⣿⣾⠟⠋⠀⠀⠈⠙⢿⣶⣿⣇⠀⠀⠀⠀⠀⠀⠀⠙⢿⣶⣤⣄⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⣠⡾⢹⣿⠟⡅⠀⠀⠀⠀⠀⠀⠀⠀⣸⡿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠈⠛⢿⡄⠀⠀⠀⠀⠀⠀⠀⢨⡻⣿⡎⢿⣆⠀⠀⠀⠀
    ⠀⠀⢠⢾⣿⠇⢟⣥⡾⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠲⢤⣄⡀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⢻⣮⣿⠸⣿⡗⣄⠀⠀
    ⠀⢠⡿⢸⣿⣴⣿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠿⣷⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣦⣿⡇⣻⡆⠀
    ⠀⣾⣷⢸⡿⣋⣴⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣤⣄⣠⠀⠀⠀⠀⠈⢻⣿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢷⣌⢻⡇⣿⣿⠀
    ⠀⢿⣿⢘⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣶⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠹⣿⣿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠈⢿⣷⡄⣿⣿⢀
    ⣧⠸⣿⣼⡿⣣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣿⣿⣅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⣌⠻⣿⣿⠇⣼
    ⣿⡆⢹⡟⣰⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⠟⠉⠙⢿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⢿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⢹⣧⠹⡟⢠⣿
    ⢻⣿⡄⢰⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠙⢿⣷⣦⡀⠀⠀⠀⠀⠀⣼⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠈⣿⣧⢠⣿⡟
    ⡌⢿⣷⣾⡿⢠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣿⣦⡀⠀⠀⠀⣿⣿⣿⡟⠀⠀⠀⠀⠀⠀⠀⣎⢹⣿⣾⡿⢁
    ⣷⡌⠻⣿⡇⣼⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣀⠀⠀⠀⠀⠀⠀⠈⠻⣿⣿⣦⣀⣼⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⢰⣿⠀⣿⠟⣠⣾
    ⠘⣿⣦⡙⠃⣿⡇⣀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⡟⢿⣿⣶⣤⣄⣀⡀⠀⢀⣈⣿⣿⣿⣿⣿⣿⠏⠀⠀⠀⠀⠀⠀⢀⢸⣿⠀⢋⣼⣿⠃
    ⠀⢸⢿⣿⣦⣿⣇⠸⣆⠀⠀⠀⠀⢀⣤⣾⡿⠉⠁⠀⠈⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⣀⠀⠀⠀⠀⠀⣰⡇⢸⣿⣴⣿⠟⡥⠀
    ⠀⠀⢧⣍⠻⢿⣿⡀⣿⣇⠀⠀⢠⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠈⠉⠙⠛⠛⠛⠋⠉⠈⠻⣿⣿⡗⠀⠀⠀⣰⣿⠁⣾⡿⠛⣡⡾⠁⠀
    ⠀⠀⠀⠻⣷⣦⣍⡳⢹⣿⡗⣦⡀⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠀⠀⣠⢺⣿⡟⢈⣡⣴⣾⠟⠁⠀⠀
    ⠀⠀⠀⠀⠈⢻⣿⣿⣶⣿⣿⡌⢿⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⢃⣾⣿⣶⣿⠿⡟⠁⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠈⠳⣤⣍⣉⣛⣛⡊⠻⣿⣦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣿⠟⣡⣛⣉⣉⣭⣴⠞⠁⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠈⠙⢻⡿⠿⠿⠿⠿⠿⠛⣛⣩⣴⣶⡶⠞⣲⡶⢶⣶⡛⠿⣶⣶⣯⣉⡛⠛⠻⠿⠿⠿⠿⡟⠋⠁⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠶⣦⣤⣶⣶⣿⣿⡿⠛⣡⣴⠟⠉⠀⠀⠈⠻⢷⣄⡙⠻⢿⣿⣷⣶⣶⣶⠶⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠉⠀⠠⣾⠟⠁⠀⠀⠀⠀⠀⠀⠀⠙⣿⡦⠀⠀⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    
    ЕДИНАЯ ГОСУДАРСТВЕННАЯ СЕТЬ ВЫЧИСЛИТЕЛЬНЫХ ЦЕНТРОВ (ЕГСВЦ)
            СССР, Москва, {}
    
СЛУЖБА ПРЕДВАРИТЕЛЬНОГО ЗАКАЗА БИЛЕТОВ В КОСМИЧЕСКОЕ ПРОСТРАНСТВО 'КОСМОС-150'
    
"#, UssrDateTime::get_current_date());

    let listener = TcpListener::bind("0.0.0.0:2067").await?;
    db::init();
    println!("db initialized");
    let menu_arc = Arc::new(Menu::new());

    tokio::spawn(async move {
        loop {
            println!("generating flights...");
            if let Some(err) = FlightService::generate_flights() {
                eprintln!("Error generating flights; err = {:?}", err);
            }
            sleep(Duration::from_secs(5 * 60)).await;
        }
    });

    println!("server has been started, ready to accept connections");
    loop {
        let (socket, _) = listener.accept().await?;
        let art_clone = art.clone();

        let menu_arc_clone = menu_arc.clone();

        tokio::spawn(async move {
            let mut session = Session::new(socket);

            let greet_message = format!("{}{}", art_clone, menu_arc_clone.display_menu(&session));

            if let Some(err) = session.write(greet_message).await {
                eprintln!("failed to write to socket; err = {:?}", err);
                return;
            }

            loop {
                match session.read().await {
                    Ok(user_input) => {
                        let res: Result<usize, std::num::ParseIntError> = user_input.parse::<usize>();
                        match res {
                            Ok(selected)=> {
                                match menu_arc_clone.display_selected(&session, selected) {
                                    Ok(tr)=> {
                                        match tr {
                                            Output(output_fn)=> {
                                                let output_res = (output_fn)(&session);
                                                match output_res {
                                                    Ok(msg) => {
                                                        if let Some(err) = session.write(msg).await {
                                                            eprintln!("failed to write to socket; err = {:?}", err);
                                                            break;
                                                        }
                                                    }
                                                    Err(err) => {
                                                        if let Some(err) = session.write(format!("ПРОГРАММНЫЙ СБОЙ: {:?}", err)).await {
                                                            eprintln!("failed to write to socket; err = {:?}", err);
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                            Input(mut transaction) => {
                                                while !transaction.done() {
                                                    if let Some(err) = session.write(transaction.ask()).await {
                                                        eprintln!("failed to write to socket; err = {:?}", err);
                                                        break;
                                                    }
                                                    match session.read().await {
                                                        Ok(user_transaction_input) => {
                                                            transaction.enter(user_transaction_input);
                                                        }
                                                        Err(err) => {
                                                            eprintln!("failed to read from socket; err = {:?}", err);
                                                            break;
                                                        }
                                                    }
                                                }
                                                let tr_result = transaction.commit(&mut session);
                                                if let Some(err) = session.write(tr_result).await {
                                                    eprintln!("failed to write to socket; err = {:?}", err);
                                                    break;
                                                }
                                            }
                                            Exit => {
                                                if let Some(err) = session.write("Слава алгоритмам и науке СССР! До следующей виртуального встречи, товарищ!\n".to_string()).await {
                                                    eprintln!("failed to write to socket; err = {:?}", err);
                                                }
                                                break;
                                            }
                                        }
                                        
                                    }
                                    Err(err) => {
                                        if let Some(err) = session.write(format!("ПРОГРАММНЫЙ СБОЙ: {:?}", err)).await {
                                            eprintln!("failed to write to socket; err = {:?}", err);
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(_e)=> {
                                if let Some(err) = session.write(format!("СБОЙ ИЗ-ЗА НЕКОРРЕКТНОГО ВВОДА: НЕОБХОДИМО ВВЕСТИ ЦИФРУ")).await {
                                    eprintln!("failed to write to socket; err = {:?}", err);
                                    break;
                                }
                            }
                        }

                    }
                    Err(err) => {
                        eprintln!("failed to read from socket; err = {:?}", err);
                        break;
                    }
                }
                if let Some(err) = session.write(menu_arc_clone.display_menu(&session)).await {
                    eprintln!("failed to write to socket; err = {:?}", err);
                    break;
                }
            }
            if let Some(err) = session.close().await {
                eprintln!("failed to close connection; err = {:?}", err);
            }
            return;
        });
    }
}
