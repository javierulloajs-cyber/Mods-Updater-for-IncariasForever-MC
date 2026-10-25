use anyhow::{Context, Result};
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::{env, ffi::OsStr, path::PathBuf, thread::sleep, time::Duration};
use tokio::fs;
use std::result::Result::Ok;

use std::process::Command;


use crate::PROGRAM_VERSION;

const OWNER: &str = "javierulloajs-cyber";
const REPO: &str = "Mods-Updater-for-IncariasForever-MC";

// Definir entorno para asegurarse de descargar el ejecutable correcto
#[cfg(target_os = "windows")]
const ASSET_EXTENSION: &str = ".exe";

#[cfg(target_os = "linux")]
const ASSET_EXTENSION: &str = ".pdb";

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    html_url: String,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub async fn actualizar_programa(argumentos_opcionales: &Vec<String>) -> Result<()> {
    // Evita avanzar en caso de que el argumento este activo
    let arg_bloqued_version = argumentos_opcionales.get(1);
    if arg_bloqued_version.is_some() {
        if arg_bloqued_version.unwrap().eq("--b") { return Ok(()); }
        else { println!("[MOD UPDATER]: Se introdujo un argumento invalido. ¿Quisiste usar '--b'?") }
    }
        

    // Espera un segundo antes de empezar el sistema de actualización
    // Es asi por que en caso de que se haya ejecutado desde la terminal de una
    // versión anterior esa necesita tiempo para cerrarse, así que este tiempo
    // Está por precaución.
    sleep(Duration::from_secs(1));

    // Limpia el directorio de versiones anteriores
    eliminar_antecesores().await?;

    let mut cerrar_programa = false;
    let mut new_program_version_name: String = String::new();
    actualizar_programa_parte_1(&mut new_program_version_name, &mut cerrar_programa).await?;

    println!("¿Se cerrará el programa?: {cerrar_programa}");

    // Solo se pasa a esta parte si el programa ya fue marcado para cerrarse
    actualizar_programa_parte_2(&new_program_version_name, &mut cerrar_programa)?;

    if cerrar_programa  {
        println!("///////////////////////////////////////////");
        println!("[MOD UPDATER]: Actualización finalizada...");
        println!("///////////////////////////////////////////");
        std::process::exit(0);
    }

    Ok(())
}

// Elimina las versiones anteriores a la actual
async fn eliminar_antecesores() -> Result<()> {
    // Obtiene la ruta hasta donde está este ejecutable
    let mut exe_path = env::current_exe().expect("[MOD UPDATER]: El directorio es inaccesible.");

    // Guardar el nombre exacto de este ejecutable
    let executable_file_name = exe_path.file_name().unwrap().to_string_lossy().to_string();

    // Se deja la ruta, pero sin la terminación que incluye el ejecutable
    exe_path.pop();

    for elemento in exe_path.read_dir()?.flatten() {
        let file_name = elemento.file_name().to_string_lossy().to_string();

        // Se ignorará si el elemento actual es igual a este ejecutable
        if file_name == executable_file_name { continue; }

        // Ignorará también los directorio/carpetas
        if elemento.file_type()?.is_dir() { continue; }

        // Omite a los que no son ejecutables
        let ext = ASSET_EXTENSION.replace(".", "");
        if elemento.path().extension() != Some(OsStr::new(&ext)) { continue; }
        
        // Solo lo borrará SI NO es un directorio y SI NO es el ejecutable actual
        if file_name.contains("mod_updater") { fs::remove_file(elemento.path()).await?; }
    }

    Ok(())
}

// Maneja la parte online
async fn actualizar_programa_parte_1(new_program_version_name: &mut String, cerrar_programa: &mut bool) -> Result<()> {
    println!("==================================");
    println!("   ACTUALIZADOR DEL PROGRAMA");
    println!("==================================");

    let client = reqwest::Client::builder()
        .user_agent("Incarias-Mod-Updater")
        .build()?;

    // 1. Consultar la última versión estable publicada.
    let api_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest", OWNER, REPO
    );

    println!("\n[1/3] Consultando última versión...");

    let response = client
        .get(&api_url)
        .header(USER_AGENT, "Incarias-Mod-Updater")
        .send()
        .await?
        .error_for_status()
        .context("No se pudo consultar el release de GitHub")?;

    let release: Release = response.json().await?;

    println!("Versión encontrada: {}", release.tag_name);
    println!("Página: {}", release.html_url);

    if release.tag_name == PROGRAM_VERSION {
        println!("[MOD UPDATER]: No hace falta actualizar el programa :) (se prosigue con el analisis para los mods).");
        return Ok(())
    }
    *cerrar_programa = true; // Por default es false, asi que no hace falta programar casos para que pase a ser false

    // 2. Buscar un archivo compatible con la extensión configurada.
    let asset = release
        .assets
        .iter()
        .find(|asset| {
            asset.name.to_lowercase().ends_with(ASSET_EXTENSION)
        })
        .context("El release no contiene un archivo compatible")?;

    println!("\n[2/3] Archivo seleccionado: {}", asset.name);

    // 3. Descargar el archivo.
    println!("\n[3/3] Descargando...");

    let response = client
        .get(&asset.browser_download_url)
        .send()
        .await?
        .error_for_status()
        .context("Falló la descarga del archivo")?;

    let bytes = response.bytes().await?;

    let output_dir = PathBuf::new();

    let output_path = output_dir.join(&asset.name);

    let output_display = output_path.display().to_string();
    // Guardamos el nombre para usarlo desde otras funciones
    new_program_version_name.push_str(&output_display);

    fs::write(&output_path, &bytes).await?;

    println!("\n¡Descarga completada!");
    println!("Versión: {}", release.tag_name);
    println!("Archivo: {}", output_display);

    Ok(())
}

// Maneja la parte local, es donde se ejecutará el nuevo
fn actualizar_programa_parte_2(new_program_version_name: &String, cerrar_programa: &bool) -> Result<()> {
    // Solo se ejecutará la parte dos si la orden de *Cerrar programa* ya fue marcada como True
    // Para evitar errores, como que se ejecute dos veces el programa
    if !*cerrar_programa { return Ok(()) }

    let mut exe_path = env::current_exe().expect("[MOD UPDATER]: El directorio es inaccesible.");
    exe_path.pop();

    // Definir el nombre del ejecutable dependiendo del OS
    /*
    No hace falta un sistema como este por que la parte 1 se encarga de
    descargar un programa compatible con el OS donde está.

    let exe_name = if cfg!(target_os = "windows") {
        "mod_updater_0.0.1.exe"
    } else {
        "mod_updater_0.0.1"
    };
    */

    let objetivo_path = exe_path.join(new_program_version_name);

    println!("[MOD UPDATER]: Intentando ejecutar: {:?}", objetivo_path);
    
    #[cfg(target_os = "windows")]
    let _status = Command::new("cmd")
        .args(["/C", "start", "", "/D"])
        .arg(&exe_path)
        .arg(&objetivo_path)
        .status()
        .expect("[MOD UPDATER]: Falló al intentar ejecutar el ejecutable.");

    #[cfg(target_os = "linux")]
    let _status = {
    let terminales = ["x-terminal-emulator", "xterm"];
    let mut resultado = None;

    for terminal in terminales {
        let intento = Command::new(terminal)
            .arg("-e")
            .arg(&objetivo_path)
            .spawn();

        if let Ok(proceso) = intento {
            resultado = Some(proceso);
            break;
        }
    }


    resultado.expect(
        "[MOD UPDATER]: No se encontró una terminal compatible. \
         Instala x-terminal-emulator o xterm."
    )
    };

    println!("[MOD UPDATER]: El programa solicitó abrir una nueva terminal.");

    Ok(())
}
