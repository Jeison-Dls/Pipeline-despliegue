use serde::Serialize;

#[derive(Serialize)]
pub struct Appointment {
    pub id: i32,
    pub patient_id: i32,
    pub doctor_id: i32,
    pub appointment_date: chrono::NaiveDateTime,
    pub status: String,
}
