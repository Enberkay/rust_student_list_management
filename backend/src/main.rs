use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Student {
    id: String,
    name: String,
    age: u8,
    grade: f32,
}

impl Student {
    fn new(name: &str, age: u8, grade: f32) -> Self {
        Student {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            age,
            grade,
        }
    }
}

struct StudentManager {
    students: HashMap<String, Student>,
}

impl StudentManager {
    fn new() -> Self {
        let mut manager = StudentManager {
            students: HashMap::new(),
        };
        
        // Add sample students
        manager.add_student(Student::new("Leon S. Kennedy", 21, 3.75));
        manager.add_student(Student::new("Claire Redfield", 22, 3.60));
        manager.add_student(Student::new("Jill Valentine", 23, 3.85));
        
        manager
    }

    fn add_student(&mut self, student: Student) {
        self.students.insert(student.id.clone(), student);
    }

    fn get_all_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }

    fn get_student(&self, id: &str) -> Option<&Student> {
        self.students.get(id)
    }

    fn update_student(&mut self, id: &str, student: Student) -> bool {
        if self.students.contains_key(id) {
            self.students.insert(id.to_string(), student);
            true
        } else {
            false
        }
    }

    fn delete_student(&mut self, id: &str) -> bool {
        self.students.remove(id).is_some()
    }

    fn calculate_average(&self) -> f32 {
        if self.students.is_empty() {
            return 0.0;
        }
        let total: f32 = self.students.values().map(|s| s.grade).sum();
        total / self.students.len() as f32
    }

    fn min_grade(&self) -> f32 {
        self.students
            .values()
            .map(|s| s.grade)
            .fold(f32::INFINITY, f32::min)
    }

    fn max_grade(&self) -> f32 {
        self.students
            .values()
            .map(|s| s.grade)
            .fold(f32::NEG_INFINITY, f32::max)
    }
}

#[derive(Serialize)]
struct Statistics {
    total_students: usize,
    average_grade: f32,
    highest_grade: f32,
    lowest_grade: f32,
}

// API Handlers
async fn get_students(data: web::Data<Mutex<StudentManager>>) -> Result<HttpResponse> {
    let manager = data.lock().unwrap();
    let students = manager.get_all_students();
    Ok(HttpResponse::Ok().json(students))
}

async fn get_student(
    path: web::Path<String>,
    data: web::Data<Mutex<StudentManager>>,
) -> Result<HttpResponse> {
    let manager = data.lock().unwrap();
    let id = path.into_inner();
    
    match manager.get_student(&id) {
        Some(student) => Ok(HttpResponse::Ok().json(student)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Student not found"
        }))),
    }
}

async fn create_student(
    student: web::Json<Student>,
    data: web::Data<Mutex<StudentManager>>,
) -> Result<HttpResponse> {
    let mut manager = data.lock().unwrap();
    let mut new_student = student.into_inner();
    new_student.id = Uuid::new_v4().to_string();
    
    manager.add_student(new_student.clone());
    Ok(HttpResponse::Created().json(new_student))
}

async fn update_student(
    path: web::Path<String>,
    student: web::Json<Student>,
    data: web::Data<Mutex<StudentManager>>,
) -> Result<HttpResponse> {
    let mut manager = data.lock().unwrap();
    let id = path.into_inner();
    let mut updated_student = student.into_inner();
    updated_student.id = id.clone();
    
    if manager.update_student(&id, updated_student.clone()) {
        Ok(HttpResponse::Ok().json(updated_student))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Student not found"
        })))
    }
}

async fn delete_student(
    path: web::Path<String>,
    data: web::Data<Mutex<StudentManager>>,
) -> Result<HttpResponse> {
    let mut manager = data.lock().unwrap();
    let id = path.into_inner();
    
    if manager.delete_student(&id) {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": "Student deleted successfully"
        })))
    } else {
        Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Student not found"
        })))
    }
}

async fn get_statistics(data: web::Data<Mutex<StudentManager>>) -> Result<HttpResponse> {
    let manager = data.lock().unwrap();
    let stats = Statistics {
        total_students: manager.students.len(),
        average_grade: manager.calculate_average(),
        highest_grade: manager.max_grade(),
        lowest_grade: manager.min_grade(),
    };
    Ok(HttpResponse::Ok().json(stats))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Student Management API server...");
    println!("Server running at http://localhost:8080");
    
    let student_manager = web::Data::new(Mutex::new(StudentManager::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(student_manager.clone())
            .service(
                web::scope("/api")
                    .route("/students", web::get().to(get_students))
                    .route("/students", web::post().to(create_student))
                    .route("/students/{id}", web::get().to(get_student))
                    .route("/students/{id}", web::put().to(update_student))
                    .route("/students/{id}", web::delete().to(delete_student))
                    .route("/statistics", web::get().to(get_statistics))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
