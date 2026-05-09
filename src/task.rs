use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Representa el estado de una tarea
///
/// CONCEPTO RUST: Enums en Rust son más poderosos que en otros lenguajes.
/// Pueden contener datos y usarse con pattern matching.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

/// Estructura principal de una tarea
///
/// CONCEPTO RUST:
/// - String vs &str: String es propiedad (heap), &str es prestada (stack/string literal)
/// - Option<T>: Rust no tiene null, usa Option para valores opcionales
/// - derive: macros que auto-generan código (Debug para print, Clone para copiar, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Crea una nueva tarea
    ///
    /// CONCEPTO RUST:
    /// - 'impl' define métodos para una struct
    /// - 'Self' es un alias del tipo (Task en este caso)
    /// - No hay 'new' keyword, usamos funciones asociadas
    pub fn new(id: u32, title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Task {
            id,
            title,
            description,
            status: TaskStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    /// Marca la tarea como en progreso
    ///
    /// CONCEPTO RUST:
    /// - '&mut self' significa que el método MODIFICA la instancia
    /// - '&self' sería solo lectura
    /// - 'self' consumiría/movería la instancia (ownership)
    pub fn start(&mut self) {
        self.status = TaskStatus::InProgress;
        self.updated_at = Utc::now();
    }

    /// Marca la tarea como completada
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.updated_at = Utc::now();
    }

    /// Actualiza el título de la tarea
    ///
    /// CONCEPTO RUST:
    /// - Tomamos ownership del nuevo título (title: String)
    /// - No necesitamos devolver nada, la modificación es in-place
    pub fn update_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    /// Actualiza la descripción
    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    /// Devuelve una representación legible del estado
    ///
    /// CONCEPTO RUST:
    /// - '&self' solo lee, no modifica
    /// - Devolvemos &str (prestado), no String (propiedad)
    /// - Pattern matching exhaustivo: el compilador obliga a cubrir todos los casos
    pub fn status_str(&self) -> &str {
        match self.status {
            TaskStatus::Pending => "⏳ Pendiente",
            TaskStatus::InProgress => "🔄 En Progreso",
            TaskStatus::Completed => "✅ Completada",
        }
    }
}
