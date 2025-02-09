use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::lib::send_push_notification;

// Configuración de rutas
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/notify_status_change").route(web::post().to(notify_status_change)));
    cfg.service(web::resource("/login_notify").route(web::post().to(notify_login)));
    cfg.service(web::resource("/appointments").route(web::get().to(list_appointments)));
    cfg.service(web::resource("/notify_download").route(web::post().to(notify_download)));
}

// Payload para cambios de estado
#[derive(serde::Deserialize)]
pub struct StatusChangePayload {
    pub appointment_id: i32,
    pub new_status: String,
    pub token: String,
}

// ✅ **Modificación: Usar `query()` en lugar de `query_unchecked!`**
async fn notify_status_change(
    pool: web::Data<PgPool>,
    payload: web::Json<StatusChangePayload>,
    google_credentials: web::Data<String>,
) -> impl Responder {
    let result = sqlx::query(
        "UPDATE appointments SET status = $1 WHERE id = $2"
    )
    .bind(&payload.new_status)
    .bind(payload.appointment_id)
    .execute(pool.get_ref())
    .await;

    if let Err(err) = result {
        eprintln!("Error al actualizar estado: {}", err);
        return HttpResponse::InternalServerError().finish();
    }

    // Enviar la notificación push
    if let Err(e) = send_push_notification(
        "Cambio de estado",
        &format!(
            "El estado del turno ID {} ha cambiado a {}",
            payload.appointment_id, payload.new_status
        ),
        &payload.token,
        google_credentials.get_ref(),
    )
    .await
    {
        eprintln!("Error enviando notificación: {}", e);
    }

    HttpResponse::Ok().body(format!(
        "Turno ID {} actualizado a {}",
        payload.appointment_id, payload.new_status
    ))
}

// Payload para notificaciones de inicio de sesión
#[derive(serde::Deserialize, Debug)]
pub struct LoginNotificationRequest {
    pub username: String,
    pub login_time: String,
    pub token: String,
}

// Endpoint para notificar inicio de sesión
pub async fn notify_login(
    data: web::Json<LoginNotificationRequest>,
    google_credentials: web::Data<String>,
) -> impl Responder {
    let title = format!("Login Alert for {}", data.username);
    let body = format!(
        "You logged in at {}. If this wasn't you, please contact the admin.",
        data.login_time
    );

    match send_push_notification(&title, &body, &data.token, &google_credentials).await {
        Ok(_) => HttpResponse::Ok().json("Login notification sent successfully."),
        Err(err) => {
            eprintln!("Error sending login notification: {}", err);
            HttpResponse::InternalServerError().body("Error sending login notification")
        }
    }
}

// Respuesta para la lista de turnos
#[derive(serde::Serialize, sqlx::FromRow)]
pub struct AppointmentResponse {
    pub id: i32,
    pub patient_name: String,
    pub doctor_name: String,
    pub appointment_date: String,
    pub status: String,
}

// ✅ **Modificación: Usar `query_as()` en lugar de `query_unchecked!`**
pub async fn list_appointments(pool: web::Data<PgPool>) -> impl Responder {
    let query = "
        SELECT
            a.id,
            CONCAT(p.first_name, ' ', p.last_name) AS patient_name,
            CONCAT(d.first_name, ' ', d.last_name) AS doctor_name,
            to_char(a.appointment_date, 'YYYY-MM-DD HH24:MI:SS') AS appointment_date,
            a.status
        FROM
            appointments a
        JOIN
            patients p ON a.patient_id = p.id
        JOIN
            doctors d ON a.doctor_id = d.id
    ";

    let appointments = sqlx::query_as::<_, AppointmentResponse>(query)
        .fetch_all(pool.get_ref())
        .await;

    match appointments {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            eprintln!("Error al obtener los turnos: {}", err);
            HttpResponse::InternalServerError().body("Error al obtener los turnos")
        }
    }
}

// Payload para notificaciones de descarga
#[derive(serde::Deserialize, Debug)]
pub struct DownloadNotificationRequest {
    pub appointment_id: i32,
    pub patient_name: String,
    pub doctor_name: String,
    pub appointment_date: String,
    pub status: String,
    pub token: String,
}

// Endpoint para notificar descargas
pub async fn notify_download(
    data: web::Json<DownloadNotificationRequest>,
    google_credentials: web::Data<String>,
) -> impl Responder {
    let title = format!("Descarga completa: Turno ID {}", data.appointment_id);
    let body = format!(
        "El turno de {} con el doctor {} fue descargado con éxito.",
        data.patient_name, data.doctor_name
    );

    match send_push_notification(&title, &body, &data.token, &google_credentials).await {
        Ok(_) => HttpResponse::Ok().json("Download notification sent successfully."),
        Err(err) => {
            eprintln!("Error sending download notification: {}", err);
            HttpResponse::InternalServerError().body("Error sending download notification")
        }
    }
}
