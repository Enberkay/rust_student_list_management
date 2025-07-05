use std::collections::HashMap;

use macroquad::prelude::*;
use egui_macroquad::egui;
use egui_macroquad::macroquad;

#[derive(Debug, Clone)]
struct Student {
    name: String,
    age: u8,
    grade: f32,
}

impl Student {
    fn new(name: &str, age: u8, grade: f32) -> Self {
        Student {
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
        StudentManager {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) {
        let id = format!("{}-{}", student.name, student.age);
        self.students.insert(id, student);
    }

    fn get_all_students(&self) -> Vec<&Student> {
        self.students.values().collect()
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

// Application states
enum AppScreen {
    Menu,
    List,
    Stats,
    Add,
}

#[macroquad::main("Student Manager GUI with Input")]
async fn main() {
    let mut manager = StudentManager::new();

    // Add sample students
    manager.add_student(Student::new("Leon S. Kennedy", 21, 3.75));
    manager.add_student(Student::new("Claire Redfield", 22, 3.60));
    manager.add_student(Student::new("Jill Valentine", 23, 3.85));


    let mut current_screen = AppScreen::Menu;

    // Fields for input form
    let mut input_name = String::new();
    let mut input_age = String::new();
    let mut input_grade = String::new();
    let mut form_error = None;

    loop {
        // Start egui frame
        egui_macroquad::ui(|ctx| {
            match current_screen {
                AppScreen::Menu => {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Student Manager");

                        if ui.button("View Students").clicked() {
                            current_screen = AppScreen::List;
                        }
                        if ui.button("View Statistics").clicked() {
                            current_screen = AppScreen::Stats;
                        }
                        if ui.button("Add New Student").clicked() {
                            input_name.clear();
                            input_age.clear();
                            input_grade.clear();
                            form_error = None;
                            current_screen = AppScreen::Add;
                        }
                        if ui.button("Exit").clicked() {
                            std::process::exit(0);
                        }
                    });
                }

                AppScreen::List => {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Student List");

                        for (i, student) in manager.get_all_students().iter().enumerate() {
                            ui.label(format!(
                                "{}. {} (Age: {}, Grade: {:.2})",
                                i + 1,
                                student.name,
                                student.age,
                                student.grade
                            ));
                        }

                        if ui.button("Back").clicked() {
                            current_screen = AppScreen::Menu;
                        }
                    });
                }

                AppScreen::Stats => {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Statistics");

                        let total = manager.students.len();
                        let avg = manager.calculate_average();
                        let max = manager.max_grade();
                        let min = manager.min_grade();

                        ui.label(format!("Total students: {}", total));
                        ui.label(format!("Average grade: {:.2}", avg));
                        ui.label(format!("Highest grade: {:.2}", max));
                        ui.label(format!("Lowest grade: {:.2}", min));

                        if ui.button("Back").clicked() {
                            current_screen = AppScreen::Menu;
                        }
                    });
                }

                AppScreen::Add => {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.heading("Add New Student");

                        ui.horizontal(|ui| {
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut input_name);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Age:");
                            ui.text_edit_singleline(&mut input_age);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Grade:");
                            ui.text_edit_singleline(&mut input_grade);
                        });

                        if let Some(err) = &form_error {
                            ui.colored_label(egui::Color32::RED, err);
                        }

                        if ui.button("Submit").clicked() {
                            // Validate and parse input
                            let parsed_age = input_age.trim().parse::<u8>();
                            let parsed_grade = input_grade.trim().parse::<f32>();

                            match (parsed_age, parsed_grade) {
                                (Ok(age), Ok(grade)) if !input_name.trim().is_empty() && grade <= 4.0 => {
                                    manager.add_student(Student::new(&input_name.trim(), age, grade));
                                    current_screen = AppScreen::Menu;
                                }
                                _ => {
                                    form_error = Some("Invalid input. Please check name, age (0-99), and grade (0.0-4.0).".to_string());
                                }
                            }
                        }

                        if ui.button("Back").clicked() {
                            current_screen = AppScreen::Menu;
                        }
                    });
                }
            }
        });

        // Draw macroquad output (background, etc.)
        clear_background(WHITE);

        // Draw egui on top
        egui_macroquad::draw();

        next_frame().await;
    }
}
