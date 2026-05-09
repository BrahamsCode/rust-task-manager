# 🦀 Task Manager - Proyecto de Aprendizaje de Rust

Un gestor de tareas CLI simple pero funcional para aprender los conceptos fundamentales de Rust.

## 📚 Conceptos de Rust que aprenderás

### 1. **Ownership (Propiedad)**
El concepto más importante de Rust. Cada valor tiene un único "dueño":

```rust
let s1 = String::from("hola");  // s1 es el dueño
let s2 = s1;                     // s1 ya NO es válido, ownership se movió a s2
// println!("{}", s1);           // ❌ ERROR: s1 ya no es válido
println!("{}", s2);              // ✅ OK
```

**En el proyecto:** Mira cómo en `storage.rs` el método `add_task` toma ownership de `task: Task`.

### 2. **Borrowing (Préstamo)**
Puedes "prestar" referencias sin transferir ownership:

```rust
fn calcular_largo(s: &String) -> usize {  // &String es una referencia prestada
    s.len()
}

let mi_string = String::from("hola");
let largo = calcular_largo(&mi_string);    // Prestamos con &
println!("{}", mi_string);                 // ✅ Aún podemos usar mi_string
```

- `&T` = referencia inmutable (solo lectura)
- `&mut T` = referencia mutable (puede modificar)

**En el proyecto:** Los métodos como `status_str(&self)` usan referencias.

### 3. **Result y Option - Manejo de Errores**
Rust no tiene `null` ni excepciones. Usa tipos:

```rust
// Option para valores opcionales
let opcional: Option<String> = Some("valor".to_string());
let vacio: Option<String> = None;

// Result para operaciones que pueden fallar
fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("División por cero".to_string())
    } else {
        Ok(a / b)
    }
}
```

**En el proyecto:** Todos los métodos de `storage.rs` devuelven `Result<T, String>`.

### 4. **Pattern Matching**
Manejo de casos exhaustivo y poderoso:

```rust
match resultado {
    Ok(valor) => println!("Éxito: {}", valor),
    Err(e) => println!("Error: {}", e),
}

// if let para un solo caso
if let Some(valor) = opcional {
    println!("Tiene: {}", valor);
}
```

**En el proyecto:** Mira el `match` en `main.rs` para los comandos.

### 5. **Structs y Enums**

```rust
// Struct: agrupar datos relacionados
struct Persona {
    nombre: String,
    edad: u32,
}

// Enum: uno de varios valores posibles
enum Estado {
    Activo,
    Inactivo,
    Suspendido,
}
```

**En el proyecto:** `Task` es una struct, `TaskStatus` es un enum.

### 6. **Traits y Derives**
Los traits son como interfaces:

```rust
#[derive(Debug, Clone, Serialize)]  // Auto-implementa estos traits
struct MiStruct {
    campo: String,
}
```

Traits comunes:
- `Debug` - permite imprimir con `{:?}`
- `Clone` - permite clonar con `.clone()`
- `Serialize/Deserialize` - para JSON (de la crate serde)

### 7. **Iteradores y Funciones de Alto Nivel**

```rust
let numeros = vec![1, 2, 3, 4, 5];

// map, filter, collect
let dobles: Vec<i32> = numeros.iter()
    .map(|x| x * 2)
    .filter(|x| x > &5)
    .collect();
```

**En el proyecto:** Mira `get_next_id()` en `storage.rs`.

### 8. **String vs &str**
Dos tipos de strings en Rust:

- `String` - propiedad, mutable, heap
- `&str` - referencia, inmutable, stack o literal

```rust
let literal: &str = "hola";              // String literal
let propiedad: String = String::from("hola");  // String con ownership
let referencia: &str = &propiedad;       // Referencia a String
```

## 🚀 Instalación de Rust

### Windows (tu caso)
```powershell
# Descargar e instalar desde:
# https://rustup.rs/

# O con winget:
winget install rustlang.rustup
```

Después de instalar, reinicia la terminal y verifica:
```bash
rustc --version
cargo --version
```

## 🛠️ Compilar y Ejecutar

```bash
# Navegar al proyecto
cd rust-task-manager

# Compilar (modo desarrollo)
cargo build

# Compilar y ejecutar
cargo run -- help

# Compilar optimizado (release)
cargo build --release
```

## 📖 Uso del Programa

```bash
# Ver ayuda
cargo run -- help

# Agregar tarea
cargo run -- add "Aprender Rust" "Leer el código del task manager"

# Listar tareas
cargo run -- list

# Iniciar tarea
cargo run -- start 1

# Completar tarea
cargo run -- complete 1

# Actualizar tarea
cargo run -- update 1 "Nuevo título"

# Eliminar tarea
cargo run -- delete 1
```

## 📁 Estructura del Proyecto

```
rust-task-manager/
├── Cargo.toml          # Configuración y dependencias (como package.json)
├── src/
│   ├── main.rs         # Punto de entrada, CLI
│   ├── task.rs         # Modelo de datos Task
│   └── storage.rs      # Persistencia en JSON
├── tasks.json          # Almacenamiento (se crea automáticamente)
└── target/             # Binarios compilados (se crea con cargo build)
```

## 🎯 Ejercicios para Practicar

Una vez que entiendas el código base, intenta:

1. **Agregar prioridades a las tareas**
   - Modifica `Task` para incluir un enum `Priority { Low, Medium, High }`
   - Actualiza la visualización para mostrar la prioridad

2. **Filtrar tareas por estado**
   - Agrega comando: `cargo run -- list pending`
   - Practica con iteradores: `.filter(|t| t.status == TaskStatus::Pending)`

3. **Fechas de vencimiento**
   - Agrega campo `due_date: Option<DateTime<Utc>>` a Task
   - Implementa comando para listar tareas vencidas

4. **Búsqueda de tareas**
   - Comando para buscar por palabra clave en título/descripción
   - Practica con `.contains()` y pattern matching

5. **Exportar a CSV**
   - Crea un nuevo módulo `export.rs`
   - Implementa función para exportar tareas a CSV
   - Practica con file I/O y formateo de strings

## 📚 Recursos Adicionales

- [The Rust Book (oficial)](https://doc.rust-lang.org/book/) - La mejor guía
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Aprender con ejemplos
- [Rustlings](https://github.com/rust-lang/rustlings) - Ejercicios interactivos

## 🔑 Conceptos Clave a Recordar

1. **El compilador es tu amigo** - Los mensajes de error son muy descriptivos
2. **No hay null** - Usa `Option<T>`
3. **No hay excepciones** - Usa `Result<T, E>`
4. **Ownership** - Un valor, un dueño
5. **Borrowing** - Presta con `&` o `&mut`
6. **Inmutable por defecto** - Usa `mut` para mutabilidad
7. **Pattern matching** - Más poderoso que switch/if-else

## 💡 Tips de Desarrollo

```bash
# Ver documentación de una crate
cargo doc --open

# Formatear código automáticamente
cargo fmt

# Linter (muy útil para aprender buenas prácticas)
cargo clippy

# Ejecutar tests
cargo test
```

---

**Creado por:** Brahams  
**Propósito:** Proyecto educativo para aprender Rust desde cero  
**Stack:** Rust 2021 edition + serde + chrono
