use std::env;
use futures::StreamExt;
use telegram_bot::types::UserId;
use telegram_bot::types::MessageKind;
use telegram_bot::*;
use telegram_bot::types::requests::SendMessage;
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::Tokio02AsyncWriteCompatExt;




#[cfg(not(all(windows, feature = "sql-browser-tokio")))]
#[tokio::main]
async fn main()  {

     
    let token = env::var("BOT_TELEGRAM").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let nuevo_mensaje = SendMessage::new(
        UserId::new(965839410).to_chat_ref(), 
        "Hola Mama soy yo"
    );

    api.send(nuevo_mensaje).await.unwrap();

    let conn = "server=tcp:localhost,1433;Database=GTDB;IntegratedSecurity=true;TrustServerCertificate=true;".to_owned();
    let config = Config::from_ado_string(&conn).unwrap();
    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    tcp.set_nodelay(true).unwrap();

    let mut client = Client::connect(config, tcp.compat_write()).await.unwrap();

    let stream = client.simple_query("SELECT Count(*) From ListaNegra").await.unwrap();
    let row = stream.into_row().await.unwrap();
    println!("{:?}", row);

    // let mut config = Config::new();
    // config.host("tcp:localhost");
    // config.port(1433);
    // config.authentication(AuthMethod::sql_server("Sa","CAPUFE"));
    // config.database("GTDB");
    // let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    // tcp.set_nodelay(true).unwrap();

    // let mut _client = Client::connect(config, tcp.compat_write()).await.unwrap();

    // let strem = _client.simple_query("Select Count(*) From ListaNegra").await.unwrap();
    // let row = strem.into_row().await.unwrap();

    // println!("{:?}", row);


    // Fetch new updates via long poll method
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update.unwrap();
        println!("{:?}",update);
        if let UpdateKind::Message(message) = update.kind {
            println!("{:?}", message);
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);
                // Answer message with "Hi".
                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                .await.unwrap();
            }
        }
    }
}

    
