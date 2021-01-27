#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    ping_service::run()
}
#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
#[cfg(windows)]
mod  ping_service {
    //Blibliotecas importadas
    use std::{
        ffi::OsString,        
        sync::mpsc,
        time::Duration,
    };
    use windows_service::{
        define_windows_service,
        Result,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        },
        service_control_handler::{self, ServiceControlHandlerResult},
        service_dispatcher,
    };
    use telegram_bot::types::UserId;
    use telegram_bot::types::MessageKind;
    use telegram_bot::*;
    use telegram_bot::types::requests::SendMessage;
    use futures::StreamExt;
    use std::env;
    use tokio;
    const SERVICE_NAME: &str = "ping_service";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;    
    pub fn run() -> Result<()> {      
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }
    define_windows_service!(ffi_service_main, my_service_main);
    pub fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(_e) = run_service() {
        }
    }
    async fn app() {
        let token = env::var("BOT_TELEGRAM").expect("TELEGRAM_BOT_TOKEN not set");
        let api = Api::new(token);
        let nuevo_mensaje = SendMessage::new(
            UserId::new(965839410).to_chat_ref(), 
            "Hola Mama soy yo"
        );
        api.send(nuevo_mensaje).await.unwrap();
        // let mut stream = api.stream();
        // while let Some(update) = stream.next().await {
        //     If the received update contains a new message...
        //     let update = update.unwrap();
        //     println!("{:?}",update);
        //     if let UpdateKind::Message(message) = update.kind {
        //         println!("{:?}", message);
        //         if let MessageKind::Text { ref data, .. } = message.kind {
        //             Print received text message to stdout.
        //             println!("<{}>: {}", &message.from.first_name, data);
        //             Answer message with "Hi".
        //             api.send(message.text_reply(format!(
        //                 "Hi, {}! You just wrote '{}'",
        //                 &message.from.first_name, data
        //             )))
        //             .await.unwrap();
        //         }
        //     }
        // }
    }
    pub fn run_service() -> Result<()> {
        use std::fs::File;
        use std::io::prelude::*;
        // Create a channel to be able to poll a stop event from the service worker loop.
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        //Se realiza la suscripcion para los evento de windows 
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                // Notifies a service to report its current status information to the service
                // control manager. Always return NoError even if not implemented.
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
                // Handle stop
                ServiceControl::Stop => {
                    shutdown_tx.send(()).unwrap();
                    ServiceControlHandlerResult::NoError
                }
                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };
        //Se crea el status_handle
        let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;
        // Tell the system that service is running
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;
        let mut archivo = File::create("C:\\log.txt").unwrap();          
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let future = app();
        rt.block_on(future);
        loop {             
            archivo.write_all(b"hola mundo\n").unwrap();
            // Poll shutdown event.
            match shutdown_rx.recv_timeout(Duration::from_secs(10)) {
                // Break the loop either upon stop or channel disconnect
                Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                // Continue work if no events were received within the timeout
                Err(mpsc::RecvTimeoutError::Timeout) => (),
            };
        }        
        // Tell the system that service has stopped.
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;
        
        Ok(())
    }
}



