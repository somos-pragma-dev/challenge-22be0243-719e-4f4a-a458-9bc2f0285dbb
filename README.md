# Construcción de una API REST en Rust con Actix Web y Diesel ORM

En un dominio de banca, se requiere construir una API REST que maneje operaciones de cuentas bancarias. La API debe soportar creación, lectura, actualización y eliminación de cuentas. Debe manejar adecuadamente los errores y asegurar la integridad de los datos. Los actores involucrados son el 'cliente bancario', el'sistema de cuentas' y el'registro de transacciones'. La API debe asegurar que no se puedan crear cuentas con números de cuenta duplicados y que las operaciones de actualización no resulten en saldos negativos. El sistema debe manejar un volumen de 1 500 solicitudes por segundo en hora pico.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | rust-actix-web |
| **Nivel** | junior-l1 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y generará un ZIP listo para usar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Un IDE o editor de código.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y generará un archivo ZIP descargable. Descárgalo y extráelo en la carpeta donde quieras trabajar.
- Verifica que el proyecto arranca sin errores.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no genera el ZIP automáticamente al terminar el análisis, escríbele: "genera el ZIP ahora".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Definición del dominio y modelado de datos

**Objetivo:** Definir el dominio y modelar los datos necesarios para las operaciones de cuentas bancarias.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Identificar los actores y las operaciones necesarias en el dominio de banca.
- Modelar los datos para representar cuentas bancarias, incluyendo atributos como número de cuenta, nombre del titular, saldo y tipo de cuenta.
- Establecer reglas de validación para evitar duplicados y saldos negativos.

**Entregable:** Modelo de datos para cuentas bancarias con reglas de validación definidas.

<details>
<summary>Pistas de conocimiento</summary>

- Considera las restricciones de negocio al modelar los datos.
- Piensa en cómo representarías los errores de validación en el modelo.

</details>

### Fase 2: Implementación de endpoints REST

**Objetivo:** Implementar los endpoints REST para las operaciones CRUD de cuentas bancarias.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Crear endpoints para crear, leer, actualizar y eliminar cuentas bancarias.
- Asegurar que los endpoints manejen adecuadamente los errores y validaciones definidas en la fase anterior.
- Garantizar que la API sea idempotente para las operaciones de creación y actualización de cuentas.

**Entregable:** Endpoints REST implementados para las operaciones CRUD de cuentas bancarias.

<details>
<summary>Pistas de conocimiento</summary>

- Piensa en cómo manejarías los errores de validación en los endpoints.
- Considera la idempotencia al implementar las operaciones de creación y actualización.

</details>

### Fase 3: Integración con el sistema de cuentas y registro de transacciones

**Objetivo:** Integrar la API con el sistema de cuentas y el registro de transacciones para asegurar la integridad de los datos y el manejo de errores.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Integrar la API con el sistema de cuentas para leer y actualizar los datos de las cuentas.
- Integrar la API con el registro de transacciones para guardar un registro de cada operación realizada.
- Asegurar que la integración maneje adecuadamente los errores y garantice la consistencia de los datos entre el sistema de cuentas y el registro de transacciones.

**Entregable:** API integrada con el sistema de cuentas y el registro de transacciones.

<details>
<summary>Pistas de conocimiento</summary>

- Piensa en cómo manejarías los errores de integración con el sistema de cuentas y el registro de transacciones.
- Considera la consistencia de los datos al integrar la API con los sistemas externos.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es una API REST y por qué se usa en este contexto?
- **paraQueSirve**: ¿Para qué sirve cada endpoint implementado en la API?
- **comoSeUsa**: ¿Cómo se usa la idempotencia en las operaciones de creación y actualización de cuentas?
- **erroresComunes**: ¿Cuáles son los errores comunes que pueden ocurrir al integrar la API con el sistema de cuentas y el registro de transacciones?
- **queDecisionesImplica**: ¿Qué decisiones implica la integración de la API con los sistemas externos para garantizar la consistencia de los datos?

## Criterios de Evaluacion

- Definición clara del dominio y modelado de datos para cuentas bancarias.
- Implementación de endpoints REST para operaciones CRUD de cuentas bancarias.
- Manejo adecuado de errores y validaciones en los endpoints.
- Idempotencia en las operaciones de creación y actualización de cuentas.
- Integración exitosa de la API con el sistema de cuentas y el registro de transacciones.
- Garantía de consistencia de los datos entre la API y los sistemas externos.

---

*Reto generado automaticamente por Challenge Generator - Pragma*
