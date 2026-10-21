use std::collections::HashMap;
use std::fs::{self, File, OpenOptions, exists};
use std::io::{Read, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::MODPACK_FOLDER_TAG;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LauncherProfiles {
    profiles: HashMap<String, Profile>,
    selected_profile: String,
    settings: Option<HashMap<String, Value>>,
    version: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Profile {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    game_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_version_id: Option<String>,
    #[serde(rename = "type")]
    profile_type: String,
    created: String,
    last_used: String,
}

pub fn agregar_modpack_launcher(minecraft_folder: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Obtener la ruta del archivo launcher_profiles.json en AppData o Home (automatizado por el script del proceso local)
    let mut path = PathBuf::from(minecraft_folder);
    path.push("launcher_profiles.json");

    // 2. Abrir y leer el archivo como cadena de texto
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 2.1 obtener las rutas necesarias
    // Ruta de todos los modpacks
    let modpacks_ruta = minecraft_folder.join("modpacks");

    // Ruta de este modpack
    let modpack_ruta = modpacks_ruta.join(MODPACK_FOLDER_TAG);

    let modpack_ruta_string = modpack_ruta.to_string_lossy().to_string();

    // 3. Deserializar el contenido JSON a la estructura de Rust
    let mut profiles_data: LauncherProfiles = serde_json::from_str(&contents)?;

    let ya_existe = profiles_data.profiles.values().any(|profile| {
        profile.name == MODPACK_FOLDER_TAG || profile.game_dir.as_deref() == Some(&modpack_ruta_string)
    });

    if ya_existe {
        println!("[MOD UPDATER]: El modpack '{}' ya está registrado en el launcher. No se hicieron cambios.", MODPACK_FOLDER_TAG);
        return Ok(());
    }

    crear_archivo_tag(&modpack_ruta)?;

    // 4. Generar un identificador único para el nuevo perfil y la fecha actual en formato ISO
    let profile_id = uuid::Uuid::new_v4().to_string().replace("-", "");
    let fecha_actual = chrono::Utc::now().to_rfc3339();

    // 5. Definir la nueva instancia
    let nuevo_modpack = Profile {
        name: MODPACK_FOLDER_TAG.to_string(),
        game_dir: Some(modpack_ruta_string),
        last_version_id: Some("fabric-loader-0.19.5-1.20.1".to_string()),
        profile_type: "custom".to_string(),
        created: fecha_actual.clone(),
        last_used: fecha_actual,
    };

    // 6. Insertar el nuevo perfil dentro del diccionario de perfiles
    profiles_data.profiles.insert(profile_id, nuevo_modpack);

    // 7. Serializar de nuevo a formato JSON con sangría (pretty) y sobrescribir el archivo
    let updated_contents = serde_json::to_string_pretty(&profiles_data)?;
    let mut write_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)?;
    
    write_file.write_all(updated_contents.as_bytes())?;

    println!("[MOD UPDATER]: Modpack '{}' agregado y guardado exitosamente en el launcher.", MODPACK_FOLDER_TAG);
    Ok(())
}


fn crear_archivo_tag(modpack_folder: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Asegura que las carpetas antecesoras existan
    fs::create_dir_all(modpack_folder)?;

    // Cambiar join por push
    let mut path = PathBuf::from(modpack_folder);

    let filename = format!("{}.txt", MODPACK_FOLDER_TAG);
    path.push(filename);

    // Evita que se intenté crear si ya existe
    if exists(&path)? { return Ok(()) }

    File::create(path).expect("[MOD UPDATER]: No se pudo crear el archivo aquí.");
    //fs::write(path, "Incarias MC").expect("[MOD UPDATER]: No se pudo crear aquí.");

    Ok(())
}