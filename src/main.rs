use std::{fs::File, io::Write, path::Path};
use serde::{Deserialize};



#[derive(Debug, Deserialize)]
struct ModInfo {
    name: String,
    filename: String,
    download_url: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let manifiesto = "https://raw.githubusercontent.com/javierulloajs-cyber/Mods-Updater-for-IncariasForever-MC/main/mods.json";

    let mods_folder = Path::new("./mods");

    if !mods_folder.exists() {
        tokio::fs::create_dir_all(mods_folder).await?;
    }

    println!("Obteniendo lista de mods...");

    let cliente = reqwest::Client::builder()
        .user_agent("IncariasForever-ModsUpdater/2.0.0 (https://github.com/javierulloajs-cyber)")
        .build()?;

    let mods: Vec<ModInfo> = cliente.get(manifiesto).send().await?.json().await?;

    println!("Hay {} mods actualmente...", mods.len());

    for mod_actual in mods {
        let destino = mods_folder.join(&mod_actual.filename);

        if destino.exists() {
            println!("(OK): El mod {} ya está actualizado", &mod_actual.name);
        } else {
            println!("El mod {} se está descargando...", &mod_actual.name);
            descargar_archivo(&cliente, &mod_actual.download_url, &destino).await?;
            println!("El mod {} ha sido descargado correctamente.", &mod_actual.name);
        }
    }

    println!("Sincronización completada.");
    Ok(())
}

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