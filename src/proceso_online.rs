use std::{fs::{self, File, OpenOptions}, io::Write, path::{Path, PathBuf}};

use anyhow::Result;

use crate::{ItemInfo, OFFICIAL_LIST_FILE};

pub async fn proceso_de_descarga(modpack_directorio: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    
    let manifiesto = "https://raw.githubusercontent.com/javierulloajs-cyber/Mods-Updater-for-IncariasForever-MC/main/mods.json";

    let mods_folder = modpack_directorio.join("mods");
    if !mods_folder.exists() {
        tokio::fs::create_dir_all(&mods_folder).await?;
    }

    let shaderpacks_folder = modpack_directorio.join("shaderpacks");
    if !shaderpacks_folder.exists() {
        tokio::fs::create_dir_all(shaderpacks_folder).await?;
    }

    println!("Obteniendo lista de mods...");

    let cliente = reqwest::Client::builder()
        .user_agent("IncariasForever-ModUpdater/2.0.0 (https://github.com/javierulloajs-cyber)")
        .build()?;

    
    // Limpiar el archivo oficial de los mods para evitar guardar mods absoletos
    let official_list_path = &mods_folder.join(OFFICIAL_LIST_FILE);
    if official_list_path.exists() {
        fs::remove_file(&mods_folder.join(official_list_path)).expect("[MOD UPDATER]: No se pudo borrar el archivo de lista oficial (sirve para evitar guardar mods absletos).");
    }

    let mods: Vec<ItemInfo> = cliente
        .get(manifiesto)
        .send()
        .await
        .expect("[MOD UPDATE]: No se pudo extraer el Response del 'cliente'.")
        .json()
        .await
        .expect("[MOD UPDATER]: No se pudo extraer el json para un vector.");

    println!("Hay {} mods actualmente...", mods.len());

    for item_actual in mods {
        write_file_official_list(&mods_folder, &item_actual)?;

        match item_actual.folder.as_str() {
            "mods" => descargar_archivo_caso_mods(&cliente, &item_actual, &modpack_directorio).await?,
            "shaderpacks" => descargar_archivo_caso_shaderpacks(&cliente, &item_actual, &modpack_directorio).await?,
            "raiz" => descargar_archivo_caso_raiz(&cliente, &item_actual, &modpack_directorio).await?,
            _=> continue
        }
    }

    println!("Sincronización completada.");
    Ok(())
}



// Funciones semi-auxiliares
async fn descargar_archivo_caso_mods(cliente: &reqwest::Client, item_actual: &ItemInfo, modpack_directorio: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let destino = modpack_directorio.join("mods").join(&item_actual.filename);

    if destino.exists() {
        println!("[MOD UPDATER]: El mod '{}' ya está actualizado.", &item_actual.name);
    } else {
        println!("[MOD UPDATER]: El mod '{}' se está descargando...", &item_actual.name);
        descargar_archivo(&cliente, &item_actual.download_url, &destino).await?;
        println!("[MOD UPDATER]: El mod '{}' ha sido descargado correctamente.", &item_actual.name);
    }
    Ok(())
}

/* Error actual: [MOD UPDATER]: El shader 'Complementary Shaders Reimagined' se está descargando...
Error: Os { code: 3, kind: NotFound, message: "El sistema no puede encontrar la ruta especificada." }

Vamos, ¡YO PUEDO!
*/
async fn descargar_archivo_caso_shaderpacks(cliente: &reqwest::Client, item_actual: &ItemInfo, modpack_directorio: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let destino = modpack_directorio.join("shaderpacks").join(&item_actual.filename);

    if destino.exists() {
        println!("[MOD UPDATER]: El shader '{}' ya está actualizado.", &item_actual.name);
    } else {
        println!("[MOD UPDATER]: El shader '{}' se está descargando...", &item_actual.name);
        descargar_archivo(&cliente, &item_actual.download_url, &destino).await?;
        println!("[MOD UPDATER]: El shader '{}' ha sido descargado correctamente.", &item_actual.name);
    }
    Ok(())
}

async fn descargar_archivo_caso_raiz(cliente: &reqwest::Client, item_actual: &ItemInfo, modpack_directorio: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let destino = modpack_directorio.join(&item_actual.filename);

    if destino.exists() {
        println!("[MOD UPDATER]: El archivo '{}' ya está actualizado.", &item_actual.name);
    } else {
        println!("[MOD UPDATER]: El archivo '{}' se está descargando...", &item_actual.name);
        descargar_archivo(&cliente, &item_actual.download_url, &destino).await?;
        println!("[MOD UPDATER]: El archivo '{}' ha sido descargado correctamente.", &item_actual.name);
    }
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



fn write_file_official_list(directorio: &PathBuf, actual_mod: &ItemInfo) 
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