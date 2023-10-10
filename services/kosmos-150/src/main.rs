use tokio::net::TcpListener;

use kosmos150::menu::menu::Menu;
use kosmos150::menu::menu::MenuTransaction::{Input, Output, Exit};
use kosmos150::network::session::Session;
use kosmos150::utils::date::DateTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date_time = DateTime::new();
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
    
        ИНФОРМАЦИОННОЕ ВСЕСОЮЗНОЕ СЕТЕВОЕ ВЕЩАНИЕ
            СССР, Москва, {}
    
СЛУЖБА ПРЕДВАРИТЕЛЬНОГО ЗАКАЗА БИЛЕТОВ В КОСМИЧЕСКОЕ ПРОСТРАНСТВО 'КОСМОС-150'
    
"#, date_time.get_current_date());

    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        let art_clone = art.clone();

        tokio::spawn(async move {
            let mut session = Session::new(socket);

            let menu = Menu::new();
            let greet_message = format!("{}{}", art_clone, menu.display_menu());

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
                                match menu.display_selected(selected) {
                                    Ok(tr)=> {
                                        match tr {
                                            Output(msg)=> {
                                                if let Some(err) = session.write(msg).await {
                                                    eprintln!("failed to write to socket; err = {:?}", err);
                                                    return;
                                                }
                                            }
                                            Input(mut transaction) => {
                                                while !transaction.done() {
                                                    if let Some(err) = session.write(transaction.ask()).await {
                                                        eprintln!("failed to write to socket; err = {:?}", err);
                                                        return;
                                                    }
                                                    match session.read().await {
                                                        Ok(user_transaction_input) => {
                                                            transaction.enter(user_transaction_input);
                                                        }
                                                        Err(err) => {
                                                            eprintln!("failed to read from socket; err = {:?}", err);
                                                            return;
                                                        }
                                                    }
                                                }
                                                if let Some(err) = session.write(menu.commit(selected, transaction.get_user_inputs())).await {
                                                    eprintln!("failed to write to socket; err = {:?}", err);
                                                    return;
                                                }
                                            }
                                            Exit => {
                                                if let Some(err) = session.write("Слава алгоритмам и науке СССР! До следующей виртуального встречи, товарищ!".to_string()).await {
                                                    eprintln!("failed to write to socket; err = {:?}", err);
                                                }
                                                return;
                                            }
                                        }
                                        
                                    }
                                    Err(err) => {
                                        if let Some(err) = session.write(format!("ПРОГРАММНЫЙ СБОЙ: {:?}", err)).await {
                                            eprintln!("failed to write to socket; err = {:?}", err);
                                            return;
                                        }
                                    }
                                }
                            }
                            Err(_e)=> {
                                if let Some(err) = session.write(format!("СБОЙ ИЗ-ЗА НЕКОРРЕКТНОГО ВВОДА: НЕОБХОДИМО ВВЕСТИ ЦИФРУ")).await {
                                    eprintln!("failed to write to socket; err = {:?}", err);
                                    return;
                                }
                            }
                        }

                    }
                    Err(err) => {
                        eprintln!("failed to read from socket; err = {:?}", err);
                        return;
                    }
                }
                if let Some(err) = session.write(menu.display_menu()).await {
                    eprintln!("failed to write to socket; err = {:?}", err);
                    return;
                }
            }
        });
    }
}
