# Lab6Web
Lab 6 api backendo partidos de la liga
# La Liga Tracker API

Esta API, desarrollada en Rust con Actix Web y SQLx, permite gestionar partidos de La Liga.  
Se implementan endpoints para listar, crear, actualizar y eliminar partidos, así como para realizar operaciones adicionales (registrar goles, tarjetas y establecer tiempo extra).

## Características

- **Endpoints RESTful:**  
  - `GET /api/matches`: Listar todos los partidos.
  - `GET /api/matches/{id}`: Obtener un partido por ID.
  - `POST /api/matches`: Crear un nuevo partido.
  - `PUT /api/matches/{id}`: Actualizar un partido existente.
  - `DELETE /api/matches/{id}`: Eliminar un partido.
  - `PATCH /api/matches/{id}/goals`: Registrar un gol.
  - `PATCH /api/matches/{id}/yellowcards`: Registrar una tarjeta amarilla.
  - `PATCH /api/matches/{id}/redcards`: Registrar una tarjeta roja.
  - `PATCH /api/matches/{id}/extratime`: Establecer tiempo extra.

- **Documentación Swagger:**  
  El archivo `openapi.yaml` en la raíz documenta la API en formato OpenAPI 3.0.3. Puedes visualizarlo en [Swagger Editor](https://editor.swagger.io/).

- **CORS Configurado:**  
  Se utiliza `actix-cors` para permitir peticiones desde cualquier origen.

## Requisitos

- Rust (versión 1.65 o superior)
- SQLite
- Docker (opcional)

## Instalación y Ejecución

### Local (sin Docker)
1. Clona el repositorio o copia los archivos en tu carpeta de trabajo.
2. Asegúrate de tener instalado SQLite y crea la base de datos manualmente (opcional para validación en tiempo de compilación):
   ```bash
   sqlite3 la_liga.db "CREATE TABLE IF NOT EXISTS matches (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       home_team TEXT NOT NULL,
       away_team TEXT NOT NULL,
       match_date TEXT NOT NULL,
       goals INTEGER NOT NULL DEFAULT 0,
       yellow_cards INTEGER NOT NULL DEFAULT 0,
       red_cards INTEGER NOT NULL DEFAULT 0,
       extra_time BOOLEAN NOT NULL DEFAULT 0
   );"
**Pruebas de Funcionamiento:**  

Link Coleccion Postman: https://drive.google.com/drive/folders/1FJrC0tdCgByGJ8tyhU6mXtZIIrd7J4y-?usp=sharing
![Screenshot 2025-03-27 212430](https://github.com/user-attachments/assets/26279574-47ec-4d18-a789-f101da07aa5a)
Link del funcionamiento: https://drive.google.com/file/d/1UKvuY8zhHcEqf9sJ7UfteQrmLVP08Crw/view?usp=sharing
Imagen de la api funcionando con el frontend funcionan los 5 endpoints
![Screenshot 2025-03-27 214215](https://github.com/user-attachments/assets/8eaeeebb-8e38-4237-a61a-a19caba256b5)
