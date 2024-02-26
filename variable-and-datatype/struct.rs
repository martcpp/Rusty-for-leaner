// classic struct
#[derive(Debug)]
struct Student {
    name: String,
    age: usize,
    roll_no: u32,
    marks: f32,
}
// tuple struct
#[derive(Debug)]
struct Studentgrades (String,char,char,char,char,char);

// unit struct
#[derive(Debug)]
struct UnitStruct;


fn main() {
    // Classic struct with named fields
    let student1 = Student {
        name: String::from("John"), // String::from() is used to create a new string from a string literal in the heap memory
        age: 25,
        roll_no: 1,
        marks: 3.8,
    };
    let student2 = Student {
        name: String::from("Doe"), // String::from() is used to create a new string from a string literal in the heap memory
        age: 25,
        roll_no: 2,
        marks: 3.8,
    };

    let student1grades = Studentgrades("Ag".to_string(),'B','C','D','E','F');
    let student2grades = Studentgrades("Ah".to_string(),'B','C','D','E','F');

    println!("{} who is {} with roll number {} has the fellowing grades {:?} that make his total make {}", 
    student1.name, student1.age, student1.roll_no, student1grades, student1.marks);

    println!("{} who is {} with roll number {} has the fellowing grades {:?} that make his total make {}", 
    student2.name, student2.age, student2.roll_no, student2grades, student2.marks);

}