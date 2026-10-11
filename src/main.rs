use serde::{Deserialize};

mod proceso_online;
mod proceso_local;

use proceso_online::*;

pub const PROGRAM_VERSION: &str = "dev 0.0.1";

pub const OFFICIAL_LIST_FILE: &str = "official_list.txt";


#[derive(Debug, Deserialize)]
pub struct ModInfo {
    name: String,
    filename: String,
    download_url: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //proceso_de_descarga().await?;

    // LOCAL
    let modpacks_ruta = proceso_local::obtener_ruta_modpacks();

    // Dentro hay un aviso en caso de que no pueda devolver el directorio del modpack
    let modpack_folder = proceso_local::obtener_directorio_correcto(&modpacks_ruta)?;

    for item in proceso_local::obtener_local_lista_mods(&modpack_folder)? {
        println!("Mod: {}", item);
    }

    // ONLINE
    proceso_de_descarga(&modpack_folder).await?;

    proceso_local::eliminar_mods_absoletos(&modpack_folder)?;

    println!("modpack_folder: {:?}", modpack_folder.file_name().unwrap());
    Ok(())
}
