use std::{ffi::OsStr, fs::{self, File}, io::{BufRead, BufReader}, path::PathBuf};

use crate::{MODPACK_FOLDER_TAG, OFFICIAL_LIST_FILE, PROGRAM_VERSION};

// Busca la carpeta de mc (Indpependientemente de la plataforma)
pub fn obtener_ruta_minecraft_folder() -> PathBuf {
    #[cfg(target_os = "windows")]
    let appdata = dirs::data_dir()
        .expect("[J]: No se pudo obtener APPDATA.");

    #[cfg(target_os = "linux")]
    let appdata = dirs::home_dir()
        .expect("[J]: No se pudo obtener HOME_DIR.");

    appdata.join(".minecraft")
}

// Busca la carpeta de los modpacks
pub fn obtener_ruta_modpacks() -> PathBuf {
    obtener_ruta_minecraft_folder().join("modpacks")
}

// Se encarga de devolver el folder en el que trabajará
pub fn obtener_directorio_correcto(modpack_folder: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
        //imprimir_contenido_directorio(modpack_folder, "Carpeta")?;

        for item in modpack_folder.read_dir()? {
            let item = item?.path();

            //_imprimir_contenido_directorio(&item, "  Sub")?;

            // Si el modpack contiene el archivo Incarias_MC_Modpack.txt
            // devolverá el folder. Si no, no
            if esta_archivo_tag(&item) {
                return Ok(item)
            }
        }

    Err(format!("[MOD UPDATER]: Asegurate de que el modpack esté instalado. Ya que no se encontró el folder '{}' en ninguna carpeta.", MODPACK_FOLDER_TAG).into())
}

// Retorna true solo si existe el archivo Incarias_MC_Modpack.txt
pub fn esta_archivo_tag(folder: &PathBuf) -> bool {
    for archivo in folder.read_dir().expect("[J]: No se puede acceder") {
        let archivo = archivo.expect("[J]: No se puede ver el archivo").path();
        let filename = archivo.file_name().unwrap().to_string_lossy().to_string();

        let file_tag = format!("{}.txt", MODPACK_FOLDER_TAG);
        if filename.eq(&file_tag) {
            return true;
        }
    }

    false
}

// Funciones auxiliares
pub fn _obtener_local_lista_mods(modpack_folder: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mods_folder = modpack_folder.join("mods");

    let mut mods_list = Vec::new();

    for actual_mod in mods_folder.read_dir()? {
        let actual_mod = actual_mod?.path().file_name().unwrap().to_string_lossy().to_string();

        // Ignora el mod si no tiene el "INCARIAS MC" con extensión "jar" en su nombre
        if !actual_mod.contains("[INCARIAS MC]") || !actual_mod.contains(".jar") { continue; }

        mods_list.push(actual_mod);
    }

    Ok(mods_list)
}

pub fn eliminar_mods_absoletos(directorio: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Primero está en la carpeta raiz, posteriormente se mueve a mods
    // hay dos rutas, A: directorio (raiz), y B: ruta (dentro de mods)
    let ruta = directorio.join("mods");
    // Abre el archivo de forma segura
    let archivo = File::open(&ruta.join(OFFICIAL_LIST_FILE))
        .expect("[ModsUpdater]: El archivo {OFFICIAL_LIST_FILE} no existe en la carpeta (Algo salio mal al crearlo).");
    // Lo convierto en un Buffreader para tener acceso a metodos como .lines()
    let official_list: BufReader<File> = BufReader::new(archivo);
    
    // Guardo la lista de mods OFICIALES en una variable de string
    let mut mods_online_list: String = String::new();
    for linea in official_list.lines() {
        mods_online_list.push_str(&linea?);
    }

    for mod_actual in ruta.read_dir().expect("[J]: No se pudo acceder a la carpeta.") {
        let mod_actual = mod_actual.expect("[J]: No se puede extraer el mod.");

        let mod_name = &mod_actual.file_name().to_string_lossy().to_string();

        // Lo omitirá SI NO tiene la extensión jar
        if mod_actual.path().extension() != Some(OsStr::new("jar")) { continue; }

        // Mod está marcado: Sifnifica si contiene "INCARIAS MC" en el nombre
        // La segunda condición verifica si ese mod ya no está en la lista oficial
        // De forma corta, si fue parte del modpack (marcado INCARIAS MC) pero ya no está
        // En la lista oficial, será borrado
        if mod_esta_marcado(&mod_name) && !mods_online_list.contains(mod_name) {
            println!("[MOD UPDATER]: Se eliminó el mod {}.", mod_name);
            fs::remove_file(mod_actual.path())?;
        } else {
            println!("[MOD UPDATER]: Mod {} ignorado.", mod_name);
        }
    }

    Ok(())
}

fn mod_esta_marcado<T: AsRef<str>>(mod_name:  &T) -> bool {
    mod_name.as_ref().contains("[INCARIAS MC]") &&
    mod_name.as_ref().contains("jar")
}


// Funciones de debug
fn _imprimir_contenido_directorio(directorio: &PathBuf, pref: &str) -> Result<(), Box<dyn std::error::Error>> {
    for item in directorio.read_dir()? {
        let item = item?.path();
        let file_name = item.file_name();

        if item.is_dir() && item.file_name().unwrap().to_string_lossy().contains(MODPACK_FOLDER_TAG) {
            println!("{pref}: {:?} <-------", file_name);
            if fs::read_to_string(&item)?.contains(PROGRAM_VERSION) { println!(" ¡Es este!")}
            continue;
        }
        println!("{pref}: {:?}", file_name);
        if pref.contains("Sub") { continue; }
        let _= _imprimir_contenido_directorio(&item, "  Sub-carpeta")?;
    }


    Ok(())
}

