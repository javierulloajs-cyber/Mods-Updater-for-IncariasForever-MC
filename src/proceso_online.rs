use std::{fs::{self, File, OpenOptions}, io::Write, path::{Path, PathBuf}};

use crate::{ModInfo, OFFICIAL_LIST_FILE};

pub async fn proceso_de_descarga(directorio: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    
    let manifiesto = "https://raw.githubusercontent.com/javierulloajs-cyber/Mods-Updater-for-IncariasForever-MC/main/mods.json";

    let mods_folder = directorio.join("mods");

    if !mods_folder.exists() {
        tokio::fs::create_dir_all(&mods_folder).await?;
    }

    println!("Obteniendo lista de mods...");

    let cliente = reqwest::Client::builder()
        .user_agent("IncariasForever-ModsUpdater/2.0.0 (https://github.com/javierulloajs-cyber)")
        .build()?;

    let mods: Vec<ModInfo> = cliente.get(manifiesto).send().await?.json().await?;

    println!("Hay {} mods actualmente...", mods.len());

    for mod_actual in mods {
        let destino = mods_folder.join(&mod_actual.filename);

        write_file_official_list(&mods_folder, &mod_actual)?;

        if destino.exists() {
            println!("[MOD UPDATER]: El mod {} ya está actualizado.", &mod_actual.name);
        } else {
            println!("[MOD UPDATER]: El mod {} se está descargando...", &mod_actual.name);
            descargar_archivo(&cliente, &mod_actual.download_url, &destino).await?;
            println!("El mod {} ha sido descargado correctamente.", &mod_actual.name);
        }
    }

    println!("Sincronización completada.");
    Ok(())
}


// Funciones auxiliares

async fn descargar_archivo(
    cliente: &reqwest::Client,
    url: &str,
    path: &Path
) -> Result<(), Box<dyn std::error::Error>> {
    let response = cliente.get(url).send().await?;

    if !response.status().is_success() {
        return Err(format!(
            "Error al descargar {}: Código de estado HTTP {}",
            url,
            response.status()
        )
        .into());
    }
    
    let content = response.bytes().await?;

    let mut archivo = File::create(path)?;
    archivo.write_all(&content)?;

    Ok(())
}

fn write_file_official_list(directorio: &PathBuf, actual_mod: &ModInfo) 
    -> Result<(), Box<dyn std::error::Error>>
{
    let file_path = directorio.join(OFFICIAL_LIST_FILE);

    // 1. Leer el contenido actual del archivo si ya existe
    let contenido = if file_path.exists() {
        fs::read_to_string(&file_path)?
    } else {
        String::new()
    };

    // 2. Verificar si el filename ya está escrito
    if contenido.contains(&actual_mod.filename) {
        return Ok(());
    }

    // 3. Si no está, abrir el archivo en modo append y escribirlo
    let mut archivo = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)?;

    writeln!(archivo, "{}", actual_mod.filename)?;

    Ok(())
}