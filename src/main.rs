use std::{thread::sleep, time::Duration};

use serde::{Deserialize};

mod sistema_actualizacion;
mod proceso_online;
mod proceso_local;

use proceso_online::*;

pub const PROGRAM_FILE_NAME: &str = env!("CARGO_PKG_NAME");

pub const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const OFFICIAL_LIST_FILE: &str = "official_list.txt";

pub const MODPACK_FOLDER_TAG: &str = "Incarias_MC_Modpack.txt";


#[derive(Debug, Deserialize)]
pub struct ModInfo {
    name: String,
    filename: String,
    download_url: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[MOD UPDATER]: Iniciando el programa en la version {PROGRAM_VERSION}...");

    sistema_actualizacion::actualizar_programa().await?;

    // LOCAL
    let modpacks_ruta = proceso_local::obtener_ruta_modpacks();

    // Dentro hay un aviso en caso de que no pueda devolver el directorio del modpack
    let modpack_folder = proceso_local::obtener_directorio_correcto(&modpacks_ruta)?;

    //for item in proceso_local::obtener_local_lista_mods(&modpack_folder)? {
        //println!("Mod: {}", item);
    //}

    // ONLINE
    proceso_de_descarga(&modpack_folder).await?;


    // LOCAL
    proceso_local::eliminar_mods_absoletos(&modpack_folder)?;

    println!("modpack_folder: {:?}", modpack_folder.file_name().unwrap());

    sleep(Duration::from_secs(10));
    Ok(())
}
