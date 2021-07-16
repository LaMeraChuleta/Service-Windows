#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    //Bibliotecas
    use std::ffi::OsString;
    use windows_service::{
        service::{ ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
        service_manager::{ ServiceManager, ServiceManagerAccess},
    };
    //Permisos para el servicio
    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    //Conecta con la base de datos de servicios
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;
    let service_binary_path = ::std::env::current_exe()
        .unwrap()
        .with_file_name("ejecutable.exe");
    let service_info = ServiceInfo { 
        name: OsString::from("dir_service_chuleta"),
        display_name: OsString::from("Servcio-Chuleta"),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::OnDemand,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None, // run as System
        account_password: None,
    };
    let _service = service_manager.create_service(&service_info, ServiceAccess::empty())?;
    Ok(())
}
#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}