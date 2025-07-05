use macroquad::prelude::*;
use std::collections::HashMap;

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

#[macroquad::main("Student Manager GUI")]
async fn main() {
    let mut manager = StudentManager::new();

    // Sample students
    manager.add_student(Student::new("Somchai", 20, 3.25));
    manager.add_student(Student::new("Somying", 19, 3.75));
    manager.add_student(Student::new("Wichai", 21, 2.80));

    let mut current_screen = "menu";

    loop {
        clear_background(WHITE);

        match current_screen {
            "menu" => {
                draw_text("Student Manager", 20.0, 40.0, 40.0, DARKBLUE);

                if draw_button(40.0, 80.0, "View Students") {
                    current_screen = "list";
                }
                if draw_button(40.0, 140.0, "View Statistics") {
                    current_screen = "stats";
                }
                if draw_button(40.0, 200.0, "Exit") {
                    break;
                }
            }
            "list" => {
                draw_text("Student List", 20.0, 40.0, 32.0, DARKBLUE);
                let mut y = 80.0;
                for (i, student) in manager.get_all_students().iter().enumerate() {
                    let line = format!(
                        "{}. {} (Age: {}, Grade: {:.2})",
                        i + 1,
                        student.name,
                        student.age,
                        student.grade
                    );
                    draw_text(&line, 40.0, y, 24.0, BLACK);
                    y += 30.0;
                }

                if draw_button(40.0, screen_height() - 60.0, "Back to Menu") {
                    current_screen = "menu";
                }
            }
            "stats" => {
                draw_text("Statistics", 20.0, 40.0, 32.0, DARKBLUE);

                let total = manager.students.len();
                let avg = manager.calculate_average();
                let max = manager.max_grade();
                let min = manager.min_grade();

                draw_text(&format!("Total students: {}", total), 40.0, 90.0, 24.0, BLACK);
                draw_text(&format!("Average grade: {:.2}", avg), 40.0, 130.0, 24.0, BLACK);
                draw_text(&format!("Highest grade: {:.2}", max), 40.0, 170.0, 24.0, BLACK);
                draw_text(&format!("Lowest grade: {:.2}", min), 40.0, 210.0, 24.0, BLACK);

                if draw_button(40.0, screen_height() - 60.0, "Back to Menu") {
                    current_screen = "menu";
                }
            }
            _ => {}
        }

        next_frame().await;
    }
}

/// Simple button drawing and click detection
fn draw_button(x: f32, y: f32, label: &str) -> bool {
    let width = 250.0;
    let height = 40.0;

    let (mx, my) = mouse_position();
    let hovered = mx >= x && mx <= x + width && my >= y && my <= y + height;

    draw_rectangle(
        x,
        y,
        width,
        height,
        if hovered { LIGHTGRAY } else { GRAY },
    );
    draw_text(label, x + 10.0, y + 28.0, 24.0, BLACK);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}
