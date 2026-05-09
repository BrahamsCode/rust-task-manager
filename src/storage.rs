use std::fs;
use std::io::Write;
use std::path::Path;

use crate::task::Task;

/// Manejador de almacenamiento de tareas
///
/// CONCEPTO RUST:
/// - String almacena el path como dato propiedad
/// - Usamos Result<T, E> para manejo de errores sin excepciones
#[derive(Debug)]
pub struct Storage {
    file_path: String,
}

impl Storage {
    /// Crea una nueva instancia del storage
    pub fn new(file_path: &str) -> Self {
        Storage {
            file_path: file_path.to_string(), // Convertimos &str a String (prestado a propiedad)
        }
    }

    /// Carga todas las tareas del archivo
    ///
    /// CONCEPTO RUST:
    /// - Result<Vec<Task>, String> significa:
    ///   Ok(vector de tareas) o Err(mensaje de error)
    /// - El operador '?' propaga errores automáticamente
    /// - No hay try/catch, usamos Result y pattern matching
    pub fn load_tasks(&self) -> Result<Vec<Task>, String> {
        // Verificar si el archivo existe
        if !Path::new(&self.file_path).exists() {
            // Si no existe, devolver vector vacío
            return Ok(Vec::new());
        }

        // Leer el archivo
        // CONCEPTO: map_err convierte el tipo de error de io::Error a String
        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("Error al leer archivo: {}", e))?;

        // Deserializar JSON a Vec<Task>
        let tasks: Vec<Task> = serde_json::from_str(&content)
            .map_err(|e| format!("Error al parsear JSON: {}", e))?;

        Ok(tasks)
    }

    /// Guarda todas las tareas en el archivo
    ///
    /// CONCEPTO RUST:
    /// - '&[Task]' es un slice (referencia a un array de tareas)
    /// - Es más flexible que Vec<Task> porque acepta cualquier colección
    pub fn save_tasks(&self, tasks: &[Task]) -> Result<(), String> {
        // Serializar tareas a JSON con formato bonito
        let json = serde_json::to_string_pretty(tasks)
            .map_err(|e| format!("Error al serializar JSON: {}", e))?;

        // Escribir al archivo
        // CONCEPTO: creamos el archivo, obtenemos el resultado, luego escribimos
        let mut file = fs::File::create(&self.file_path)
            .map_err(|e| format!("Error al crear archivo: {}", e))?;

        file.write_all(json.as_bytes())
            .map_err(|e| format!("Error al escribir archivo: {}", e))?;

        Ok(()) // () es el "unit type", equivalente a void
    }

    /// Agrega una nueva tarea
    pub fn add_task(&self, task: Task) -> Result<(), String> {
        let mut tasks = self.load_tasks()?;
        tasks.push(task);
        self.save_tasks(&tasks)?;
        Ok(())
    }

    /// Actualiza una tarea existente
    ///
    /// CONCEPTO RUST:
    /// - iter_mut() permite iterar Y modificar
    /// - find() usa un closure (función anónima)
    #[allow(dead_code)]
    pub fn update_task(&self, updated_task: &Task) -> Result<(), String> {
        let mut tasks = self.load_tasks()?;

        // Buscar la tarea por ID y actualizarla
        // CONCEPTO: |t| es un closure, similar a arrow functions en JS
        match tasks.iter_mut().find(|t| t.id == updated_task.id) {
            Some(task) => {
                *task = updated_task.clone(); // Clonamos porque necesitamos ownership
                self.save_tasks(&tasks)?;
                Ok(())
            }
            None => Err(format!("Tarea con ID {} no encontrada", updated_task.id)),
        }
    }

    /// Elimina una tarea por ID
    ///
    /// CONCEPTO RUST:
    /// - retain() es un método funcional que filtra in-place
    pub fn delete_task(&self, task_id: u32) -> Result<(), String> {
        let mut tasks = self.load_tasks()?;
        let initial_len = tasks.len();

        // Mantener solo las tareas cuyo ID no coincide
        tasks.retain(|t| t.id != task_id);

        if tasks.len() == initial_len {
            return Err(format!("Tarea con ID {} no encontrada", task_id));
        }

        self.save_tasks(&tasks)?;
        Ok(())
    }

    /// Obtiene el siguiente ID disponible
    pub fn get_next_id(&self) -> Result<u32, String> {
        let tasks = self.load_tasks()?;

        // CONCEPTO: iter(), map(), max() son funciones de alto nivel
        // unwrap_or(0) maneja el caso de lista vacía
        let max_id = tasks.iter()
            .map(|t| t.id)
            .max()
            .unwrap_or(0);

        Ok(max_id + 1)
    }
}
