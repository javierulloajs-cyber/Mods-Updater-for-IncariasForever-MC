# Mods Updater - IncariasForever MC 

Herramienta ligera desarrollada en **Rust** para descargar y sincronizar automáticamente la lista de mods del servidor Minecraft *IncariasForever*.

---

## Características

- **Sincronización automática:** Lee el manifiesto de mods del repositorio y descarga las actualizaciones requeridas.
- **Limpieza de archivos obsoletos:** Borra mods antiguos o no listados para mantener la carpeta de mods limpia.
- **Rápido y liviano:** Construido en Rust para máxima velocidad y bajo consumo de recursos.

---

## Uso Rápido

### Opción 1: Ejecutable (Recomendado, para usuarios)
1. Descarga la última versión ejecutable desde la sección de **Releases**.
2. Lo puedes poner en cualquier parte de tu computadora
3. Ejecútalo antes de iniciar el juego (no siempre, pero util para mantenerte al tanto de los cambios de mods).

### Opción 2: Compilar desde el código fuente (Para desarrolladores)
Si prefieres compilar el proyecto tú mismo:

```bash
# Clonar el repositorio
git clone [https://github.com/javierulloajs-cyber/Mods-Updater-for-IncariasForever-MC.git](https://github.com/javierulloajs-cyber/Mods-Updater-for-IncariasForever-MC.git)

# Entrar al directorio
cd Mods-Updater-for-IncariasForever-MC

# Compilar y ejecutar
cargo run --release