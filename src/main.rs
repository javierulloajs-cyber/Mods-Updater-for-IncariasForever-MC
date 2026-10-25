use std::{env, thread::sleep, time::Duration};

use serde::{Deserialize};

mod sistema_actualizacion;
mod proceso_online;
mod proceso_local;
mod crear_perfil_local;

use proceso_online::*;

pub const PROGRAM_FILE_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const OFFICIAL_LIST_FILE: &str = "official_list.txt";
pub const MODPACK_FOLDER_TAG: &str = "Incarias_MC_Modded";
// Agregaar los shaderspacks y mis opciones personalizadas

#[derive(Debug, Deserialize)]
pub struct ItemInfo {
    name: String,
    filename: String,
    download_url: String,
    folder: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argumentos_opcionales = std::env::args().collect::<Vec<String>>();

    println!("[MOD UPDATER]: Iniciando el programa en la version {PROGRAM_VERSION}...");

    // SIEMPRE DEBE EJECUTARSE ANTES QUE CUALQUIER OTRA COSA (sin contar recoleción de argumentos)
    sistema_actualizacion::actualizar_programa(&argumentos_opcionales).await?;

    // LOCAL
    let minecraft_folder_ruta = proceso_local::obtener_ruta_minecraft_folder();

    // Dirige a donde estan las rutas de el modpack
    let modpacks_ruta = proceso_local::obtener_ruta_modpacks();

    // Creación del perfil de versión local
    crear_perfil_local::agregar_modpack_launcher(&minecraft_folder_ruta)?;

    // Dentro hay un aviso en caso de que no pueda devolver el directorio del modpack
    let modpack_folder = proceso_local::obtener_directorio_correcto(&modpacks_ruta)?;

    // ONLINE
    proceso_de_descarga(&modpack_folder).await?;

    // LOCAL
    proceso_local::eliminar_mods_absoletos(&modpack_folder)?;

    println!("modpack_folder: {:?}", modpack_folder.file_name().unwrap());

    println!("[MOD UPDATER]: Finalizando el programa en la version {PROGRAM_VERSION}...");

    sleep(Duration::from_secs(3));
    Ok(())
}
