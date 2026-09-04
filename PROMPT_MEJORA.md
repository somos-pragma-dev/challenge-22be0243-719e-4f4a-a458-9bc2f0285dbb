# Prompt para Mejorar el Codigo Base

Copia y pega el siguiente contenido completo en un asistente de IA (Claude, ChatGPT, etc.)
para obtener un ZIP con el proyecto corregido y listo para compilar.

---

```
Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Exportación en ZIP
Empaqueta todos los archivos en un único archivo ZIP descargable respetando exactamente
la estructura de rutas indicada por los marcadores.
El ZIP debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: El ZIP debe estar listo para descargar al finalizar. No preguntes si el usuario
quiere generarlo. Simplemente genera el archivo y proporciona el enlace de descarga; No debes desplegar en el chat el resumen de lo que arreglaste al Zip, solo entregalo.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación o descripciones sin código, genera los archivos
correspondientes sin aplicar análisis de compilación
No agregues texto después del enlace de descarga del ZIP
No preguntes si el usuario quiere el ZIP: simplemente generalo siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:
// === ARCHIVO: Cargo.toml ===
[package]
name = "bank-api"
version = "0.1.0"
edition = "2018"

[dependencies]
actix-web = "4.0"
diesel = { version = "2.0", features = ["postgres", "r2d2"] }
dotenvy = "0.15"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"


// === ARCHIVO: src/main.rs ===
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod domain;
mod application;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
dotenv().ok();

let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
infrastructure::establish_connection(&database_url);

HttpServer::new(|| {
    App::new()
       .configure(application::config)
})
.bind("127.0.0.1:8080")?
.run()
.await
}


// === ARCHIVO: src/domain/models.rs ===
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub account_number: String,
    pub holder_name: String,
    pub balance: f64,
    pub account_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewAccount {
    pub account_number: String,
    pub holder_name: String,
    pub balance: f64,
    pub account_type: String,
}


// === ARCHIVO: src/application/handlers.rs ===
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

use crate::domain::models::{Account, NewAccount};
use crate::infrastructure::establish_connection;

pub async fn create_account(new_account: web::Json<NewAccount>) -> impl Responder {
    use crate::infrastructure::schema::accounts::dsl::*;

    let conn = establish_connection();

    let new_account = NewAccount {
        account_number: new_account.account_number.clone(),
        holder_name: new_account.holder_name.clone(),
        balance: new_account.balance,
        account_type: new_account.account_type.clone(),
    };

    diesel::insert_into(accounts)
       .values(&new_account)
       .execute(&conn)
       .expect("Error saving new account");

    HttpResponse::Ok().json(new_account)
}

pub async fn get_account(info: web::Path<(i32,)>) -> impl Responder {
    use crate::infrastructure::schema::accounts::dsl::*;

    let conn = establish_connection();
    let id = info.0;

    accounts
       .filter(id.eq(id))
       .first::<Account>(&conn)
       .optional()
       .map(|account| {
            match account {
                Some(acc) => HttpResponse::Ok().json(acc),
                None => HttpResponse::NotFound().finish(),
            }
        })
       .expect("Error loading account")
}

pub async fn update_account(info: web::Path<(i32,)>, new_account: web::Json<NewAccount>) -> impl Responder {
    use crate::infrastructure::schema::accounts::dsl::*;

    let conn = establish_connection();
    let id = info.0;

    diesel::update(accounts.find(id))
       .set(&new_account)
       .execute(&conn)
       .expect("Error updating account");

    HttpResponse::Ok().json(new_account)
}

pub async fn delete_account(info: web::Path<(i32,)>) -> impl Responder {
    use crate::infrastructure::schema::accounts::dsl::*;

    let conn = establish_connection();
    let id = info.0;

    diesel::delete(accounts.find(id))
       .execute(&conn)
       .expect("Error deleting account");

    HttpResponse::Ok().finish()
}


// === ARCHIVO: src/infrastructure/database.rs ===
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
       .build(manager)
       .expect("Failed to create pool.")
}


// === ARCHIVO: src/application/errors.rs ===
use actix_web::{error, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum AppError {
    NotFound,
    InternalError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::NotFound => HttpResponse::NotFound().finish(),
            AppError::InternalError => HttpResponse::InternalServerError().finish(),
        }
    }
}


// === ARCHIVO: src/infrastructure/diesel.rs ===
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn run_migrations(conn: &PgConnection) {
    embedded_migrations::run(conn).expect("Failed to run migrations");
}


// === ARCHIVO: tests/integration_tests.rs ===
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use diesel::prelude::*;

    #[actix_web::test]
    async fn test_create_account() {
        let app = test::init_service(App::new().configure(application::config)).await;
        let new_account = web::Json(NewAccount {
            account_number: "1234567890".to_string(),
            holder_name: "John Doe".to_string(),
            balance: 100.0,
            account_type: "Savings".to_string(),
        });

        let req = test::TestRequest::post()
           .uri="/accounts"
           .set_json(&new_account)
           .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}


// === ARCHIVO: src/application/endpoints.rs ===
use actix_web::{web, App};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/accounts")
       .route(web::post().to(handlers::create_account))
       .route(web::get().to(handlers::get_accounts)))
       .service(web::resource("/accounts/{id}")
       .route(web::get().to(handlers::get_account))
       .route(web::put().to(handlers::update_account))
       .route(web::delete().to(handlers::delete_account)));
}

```
