use reqwest::Client;
use serde_json::json;
use std::fs;
use std::env;
use anyhow::{Result, Context};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};

#[derive(Debug, serde::Serialize)]
struct Claims {
    iss: String,
    sub: String,
    aud: String,
    iat: i64,
    exp: i64,
}

pub async fn send_push_notification(
    title: &str,
    body: &str,
    recipient_token: &str,
    google_credentials_path: &str,
) -> Result<()> {
    // Leer las credenciales desde GOOGLE_APPLICATION_CREDENTIALS
    let credentials_data = fs::read_to_string(google_credentials_path)
        .context("No se pudo leer el archivo de credenciales")?;
    let credentials: serde_json::Value = serde_json::from_str(&credentials_data)
        .context("Error al parsear las credenciales de JSON")?;

    let client_email = credentials["client_email"]
        .as_str()
        .context("Falta 'client_email' en las credenciales")?;
    let private_key = credentials["private_key"]
        .as_str()
        .context("Falta 'private_key' en las credenciales")?
        .replace("\\n", "\n"); // Normalizar saltos de línea

    // Crear el token JWT para autenticar
    let claims = Claims {
        iss: client_email.to_string(),
        sub: client_email.to_string(),
        aud: "https://fcm.googleapis.com/".to_string(),
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + Duration::minutes(60)).timestamp(),
    };

    let jwt_result = encode(
        &Header {
                alg: jsonwebtoken::Algorithm::RS256, // Configurar el algoritmo RS256
                ..Default::default()
            },
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_bytes())
            .context("Error al cargar la clave privada")?,
    );

    let jwt = match jwt_result {
        Ok(token) => {
            println!("JWT generado exitosamente: {}", token);
            token
        }
        Err(err) => {
            println!("Error al generar el JWT: {:?}", err);
            return Err(err.into());
        }
    };

    // Crear cliente HTTP
    let client = Client::new();

    // Construir la solicitud a la API de FCM
    let response = client
        .post("https://fcm.googleapis.com/v1/projects/hospitalturnmanagement/messages:send")
        .bearer_auth(jwt) // Aquí usamos el JWT generado
        .json(&json!({
            "message": {
                "token": recipient_token,
                "notification": {
                    "title": title,
                    "body": body,
                }
            }
        }))
        .send()
        .await
        .context("Error al enviar la solicitud HTTP a FCM")?;

    // Manejar la respuesta de FCM
    if response.status().is_success() {
        println!("Notificación enviada con éxito.");
        Ok(())
    } else {
        let error_message = response.text().await.unwrap_or_default();
        Err(anyhow::anyhow!(
            "Error en la respuesta de FCM: {}",
            error_message
        ))
    }
}
