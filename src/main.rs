// Declarar módulos
// CONCEPTO RUST: Los archivos son módulos, hay que declararlos explícitamente
mod task;
mod storage;

use std::env;
use std::io::{self, Write};

use task::{Task, TaskStatus};
use storage::Storage;

/// Muestra el menú de ayuda
fn show_help() {
    println!("\n📋 GESTOR DE TAREAS EN RUST");
    println!("==========================\n");
    println!("Uso: task-manager <comando> [argumentos]\n");
    println!("Comandos disponibles:");
    println!("  add <título> [descripción]  - Agregar nueva tarea");
    println!("  list                        - Listar todas las tareas");
    println!("  start <id>                  - Marcar tarea como en progreso");
    println!("  complete <id>               - Marcar tarea como completada");
    println!("  delete <id>                 - Eliminar tarea");
    println!("  update <id> <nuevo_título>  - Actualizar título de tarea");
    println!("  help                        - Mostrar esta ayuda\n");
}

/// Lista todas las tareas
///
/// CONCEPTO RUST:
/// - Función que devuelve Result para manejo de errores
/// - El símbolo '?' propaga errores hacia arriba
fn list_tasks(storage: &Storage) -> Result<(), String> {
    let tasks = storage.load_tasks()?;

    if tasks.is_empty() {
        println!("\n📭 No hay tareas registradas.\n");
        return Ok(());
    }

    println!("\n📋 LISTA DE TAREAS");
    println!("==================\n");

    // CONCEPTO: iter() crea un iterador sobre referencias
    for task in tasks.iter() {
        println!("ID: {} | {}", task.id, task.status_str());
        println!("Título: {}", task.title);

        // CONCEPTO: if let para pattern matching opcional
        if let Some(desc) = &task.description {
            println!("Descripción: {}", desc);
        }

        println!("Creada: {}", task.created_at.format("%Y-%m-%d %H:%M"));
        println!("Actualizada: {}", task.updated_at.format("%Y-%m-%d %H:%M"));
        println!("---");
    }

    println!();
    Ok(())
}

/// Agrega una nueva tarea
fn add_task(storage: &Storage, args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Falta el título de la tarea".to_string());
    }

    let title = args[0].clone();

    // CONCEPTO: Usamos Option para valores opcionales
    let description = if args.len() > 1 {
        Some(args[1..].join(" ")) // Une el resto de argumentos
    } else {
        None
    };

    let id = storage.get_next_id()?;
    let task = Task::new(id, title, description);

    storage.add_task(task)?;
    println!("\n✅ Tarea #{} creada exitosamente!\n", id);

    Ok(())
}

/// Inicia una tarea (marca como en progreso)
fn start_task(storage: &Storage, task_id: u32) -> Result<(), String> {
    let mut tasks = storage.load_tasks()?;

    // CONCEPTO: Buscar tarea mutable y modificarla
    match tasks.iter_mut().find(|t| t.id == task_id) {
        Some(task) => {
            task.start();
            storage.save_tasks(&tasks)?;
            println!("\n🔄 Tarea #{} marcada como en progreso!\n", task_id);
            Ok(())
        }
        None => Err(format!("Tarea con ID {} no encontrada", task_id)),
    }
}

/// Completa una tarea
fn complete_task(storage: &Storage, task_id: u32) -> Result<(), String> {
    let mut tasks = storage.load_tasks()?;

    match tasks.iter_mut().find(|t| t.id == task_id) {
        Some(task) => {
            task.complete();
            storage.save_tasks(&tasks)?;
            println!("\n✅ Tarea #{} completada!\n", task_id);
            Ok(())
        }
        None => Err(format!("Tarea con ID {} no encontrada", task_id)),
    }
}

/// Elimina una tarea
fn delete_task(storage: &Storage, task_id: u32) -> Result<(), String> {
    storage.delete_task(task_id)?;
    println!("\n🗑️  Tarea #{} eliminada!\n", task_id);
    Ok(())
}

/// Actualiza el título de una tarea
fn update_task(storage: &Storage, task_id: u32, new_title: String) -> Result<(), String> {
    let mut tasks = storage.load_tasks()?;

    match tasks.iter_mut().find(|t| t.id == task_id) {
        Some(task) => {
            task.update_title(new_title);
            storage.save_tasks(&tasks)?;
            println!("\n✏️  Tarea #{} actualizada!\n", task_id);
            Ok(())
        }
        None => Err(format!("Tarea con ID {} no encontrada", task_id)),
    }
}

/// Función principal
///
/// CONCEPTO RUST:
/// - main() es el entry point, devuelve Result para manejo de errores
/// - env::args() obtiene los argumentos de línea de comandos
fn main() -> Result<(), String> {
    // Crear instancia de storage
    let storage = Storage::new("tasks.json");

    // Obtener argumentos de línea de comandos
    // CONCEPTO: collect() convierte el iterador en un Vec
    let args: Vec<String> = env::args().skip(1).collect();

    // Si no hay argumentos, mostrar ayuda
    if args.is_empty() {
        show_help();
        return Ok(());
    }

    // CONCEPTO: match es como switch pero exhaustivo y más poderoso
    // as_str() convierte &String a &str para pattern matching
    match args[0].as_str() {
        "help" | "-h" | "--help" => show_help(),

        "list" | "ls" => list_tasks(&storage)?,

        "add" => add_task(&storage, &args[1..])?,

        "start" => {
            if args.len() < 2 {
                return Err("Falta el ID de la tarea".to_string());
            }
            let id = args[1].parse::<u32>()
                .map_err(|_| "ID inválido, debe ser un número".to_string())?;
            start_task(&storage, id)?;
        }

        "complete" | "done" => {
            if args.len() < 2 {
                return Err("Falta el ID de la tarea".to_string());
            }
            let id = args[1].parse::<u32>()
                .map_err(|_| "ID inválido, debe ser un número".to_string())?;
            complete_task(&storage, id)?;
        }

        "delete" | "rm" => {
            if args.len() < 2 {
                return Err("Falta el ID de la tarea".to_string());
            }
            let id = args[1].parse::<u32>()
                .map_err(|_| "ID inválido, debe ser un número".to_string())?;
            delete_task(&storage, id)?;
        }

        "update" => {
            if args.len() < 3 {
                return Err("Uso: task-manager update <id> <nuevo_título>".to_string());
            }
            let id = args[1].parse::<u32>()
                .map_err(|_| "ID inválido, debe ser un número".to_string())?;
            let new_title = args[2..].join(" ");
            update_task(&storage, id, new_title)?;
        }

        unknown => {
            println!("\n❌ Comando desconocido: {}\n", unknown);
            show_help();
        }
    }

    Ok(())
}
