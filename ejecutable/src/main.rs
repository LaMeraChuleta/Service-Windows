#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    service::run()
}
#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
// mod dataBase {
//     use sqlx::mssql::MssqlConnectOptions;    
//     fn crear_conexion(){
//         let nuevaConexion = MssqlConnectOptions::new()
//             .host(host)
//     }
// }
#[cfg(windows)]
mod service {
    //Blibliotecas importadas
    use std::{ffi::OsString, sync::mpsc, time::Duration};
    use windows_service::{
        define_windows_service,
        Result,
        service::{
            ServiceControl, 
            ServiceControlAccept, 
            ServiceExitCode, 
            ServiceState, 
            ServiceStatus,
            ServiceType,
        },
        service_control_handler::{
            self, 
            ServiceControlHandlerResult
        },
        service_dispatcher,
    };  
    pub enum Status {
        Pause,
        Stop,             
    }
    //Constantes   
    const SERVICE_NAME: &str = "dir_service_chuleta";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;        
    pub fn run() -> Result<()> { 
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }
    define_windows_service!(ffi_service_main, my_service_main);
    pub fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(_e) = run_service() {
        }
    }    
    pub fn run_service() -> Result<()> {
        use std::fs::File;
        use std::io::prelude::*;
        // Create a channel to be able to poll a stop event from the service worker loop.
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        //Se realiza la suscripcion para los evento de windows 
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            let mut archivo_eventos = File::create("C:\\log.txt").unwrap();    
            match control_event {                
                //Notifica a un servicio para que informe del estado actual del servicio
                // gerente de control. Siempre devuelve NoError incluso si no está implementado.
                ServiceControl::Interrogate => {
                    archivo_eventos.write_all(b"servcio status Interrogate :?").unwrap();
                    ServiceControlHandlerResult::NoError
                },
                ServiceControl::Pause => {
                    archivo_eventos.write_all(b"servcio status pausa:/").unwrap();
                    shutdown_tx.send(Status::Pause).unwrap();
                    ServiceControlHandlerResult::NoError
                },
                ServiceControl::Continue => {
                    archivo_eventos.write_all(b"servcio status continuar :)").unwrap();
                    ServiceControlHandlerResult::NoError
                },
                // Handle stop
                ServiceControl::Stop => {
                    archivo_eventos.write_all(b"servcio status detenido :(").unwrap();
                    shutdown_tx.send(Status::Stop).unwrap();
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
            controls_accepted: ServiceControlAccept::all(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;
        let mut archivo = File::create("C:\\log.txt").unwrap();                                  
        loop {             
            archivo.write_all(b"hola mundo\n").unwrap();
            // Poll shutdown event.
            match shutdown_rx.recv_timeout(Duration::from_secs(10)) {
                // Break the loop either upon stop or channel disconnect
                Ok(Status::Stop) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Ok(Status::Pause) => {
                    status_handle.set_service_status(ServiceStatus {
                        service_type: SERVICE_TYPE,
                        current_state: ServiceState::Paused,
                        controls_accepted: ServiceControlAccept::empty(),
                        exit_code: ServiceExitCode::Win32(0),
                        checkpoint: 0,
                        wait_hint: Duration::default(),
                        process_id: None,
                    })?;
                    ()
                },
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



