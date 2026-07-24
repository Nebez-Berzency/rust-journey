fn main() {
    let student_name = "Ali";
    let score = 69;
    let student_status;
    let final_grade;

    if score < 60 {
        final_grade = "F";
        student_status = "Failed";
    } else if score <= 69 {
        final_grade = "D";
        student_status = "Passed";
    } else if score <= 79 {
        final_grade = "C";
        student_status = "Passed";
    } else if score <= 89 {
        final_grade = "B";
        student_status = "Passed";
    } else if score <= 100 {
        final_grade = "A";
        student_status = "Passed";
    } else {
        final_grade = "O";
        student_status = "Unknown status!";
    }

    println!("========== Grade Report ==========");
    println!();
    println!("Student : {student_name}");
    println!("Score : {score}");
    println!("Grade : {final_grade}");
    println!("Status :{student_status}");
    println!();
    println!("==================================");
}
