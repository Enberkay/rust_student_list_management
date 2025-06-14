use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Student {
    name: String,
    age: u8,
    grade: f32,
}

impl Student {
    fn new(name: String, age: u8, grade: f32) -> Self {
        Student { name, age, grade }
    }
    
    fn display(&self) {
        println!("ชื่อ: {}, อายุ: {} ปี, เกรด: {:.2}", self.name, self.age, self.grade);
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
        println!("✅ เพิ่มนักเรียนเรียบร้อยแล้ว!");
    }
    
    fn list_students(&self) {
        if self.students.is_empty() {
            println!("📝 ยังไม่มีข้อมูลนักเรียน");
            return;
        }
        
        println!("\n📋 รายชื่อนักเรียนทั้งหมด:");
        println!("{:-<50}", "");
        for (i, student) in self.students.values().enumerate() {
            print!("{}. ", i + 1);
            student.display();
        }
        println!("{:-<50}", "");
    }
    
    fn find_student(&self, name: &str) {
        let found: Vec<&Student> = self.students
            .values()
            .filter(|s| s.name.to_lowercase().contains(&name.to_lowercase()))
            .collect();
            
        if found.is_empty() {
            println!("❌ ไม่พบนักเรียนชื่อ '{}'", name);
        } else {
            println!("\n🔍 ผลการค้นหา:");
            for student in found {
                student.display();
            }
        }
    }
    
    fn calculate_average(&self) -> f32 {
        if self.students.is_empty() {
            return 0.0;
        }
        
        let total: f32 = self.students.values().map(|s| s.grade).sum();
        total / self.students.len() as f32
    }
    
    fn show_statistics(&self) {
        if self.students.is_empty() {
            println!("📊 ไม่มีข้อมูลสำหรับสถิติ");
            return;
        }
        
        let grades: Vec<f32> = self.students.values().map(|s| s.grade).collect();
        let avg = self.calculate_average();
        let max = grades.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let min = grades.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        
        println!("\n📊 สstatisticsข้อมูล:");
        println!("จำนวนนักเรียน: {} คน", self.students.len());
        println!("เกรดเฉลี่ย: {:.2}", avg);
        println!("เกรดสูงสุด: {:.2}", max);
        println!("เกรดต่ำสุด: {:.2}", min);
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn show_menu() {
    println!("\n🎓 ระบบจัดการรายชื่อนักเรียน");
    println!("1. เพิ่มนักเรียน");
    println!("2. แสดงรายชื่อทั้งหมด");
    println!("3. ค้นหานักเรียน");
    println!("4. แสดงสถิติ");
    println!("5. ออกจากโปรแกรม");
    println!("{:=<30}", "");
}

fn main() {
    let mut manager = StudentManager::new();
    
    // เพิ่มข้อมูลตัวอย่าง
    manager.add_student(Student::new("สมชาย".to_string(), 20, 3.25));
    manager.add_student(Student::new("สมหญิง".to_string(), 19, 3.75));
    manager.add_student(Student::new("วิชัย".to_string(), 21, 2.80));
    
    loop {
        show_menu();
        let choice = get_input("เลือกเมนู (1-5): ");
        
        match choice.as_str() {
            "1" => {
                println!("\n➕ เพิ่มนักเรียนใหม่");
                let name = get_input("ชื่อ: ");
                
                if name.is_empty() {
                    println!("❌ กรุณาใส่ชื่อ");
                    continue;
                }
                
                let age_str = get_input("อายุ: ");
                let age: u8 = match age_str.parse() {
                    Ok(a) if a > 0 && a < 100 => a,
                    _ => {
                        println!("❌ อายุไม่ถูกต้อง");
                        continue;
                    }
                };
                
                let grade_str = get_input("เกรด (0.00-4.00): ");
                let grade: f32 = match grade_str.parse() {
                    Ok(g) if g >= 0.0 && g <= 4.0 => g,
                    _ => {
                        println!("❌ เกรดไม่ถูกต้อง");
                        continue;
                    }
                };
                
                manager.add_student(Student::new(name, age, grade));
            },
            "2" => manager.list_students(),
            "3" => {
                let name = get_input("ค้นหาชื่อ: ");
                if !name.is_empty() {
                    manager.find_student(&name);
                }
            },
            "4" => manager.show_statistics(),
            "5" => {
                println!("👋 ขอบคุณที่ใช้บริการ!");
                break;
            },
            _ => println!("❌ กรุณาเลือกเมนู 1-5 เท่านั้น"),
        }
        
        println!("\nกด Enter เพื่อดำเนินการต่อ...");
        let _ = get_input("");
    }
}