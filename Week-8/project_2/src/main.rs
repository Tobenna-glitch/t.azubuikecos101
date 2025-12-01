use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // Student data stored in vectors
    let names = vec![
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];

    let matrics = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001",
    ];

    let departments = vec![
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];

    let levels = vec![300, 100, 200, 200, 100];

    // Display the records
    println!("PAU SMIS - Student Records\n");

    for i in 0..names.len() {
        println!(
            "Name: {}\nMatric: {}\nDepartment: {}\nLevel: {}\n",
            names[i], matrics[i], departments[i], levels[i]
        );
    }

    // Save to a file (THIS MUST BE INSIDE main)
    let mut file = File::create("students.txt")?;

    writeln!(file, "PAU SMIS - Student Records\n")?;
    writeln!(file, "Student Name | Matric Number | Department | Level")?;
    writeln!(file, "---------------------------------------------------")?;

    for i in 0..names.len() {
        writeln!(
            file,
            "{} | {} | {} | {}",
            names[i], matrics[i], departments[i], levels[i]
        )?;
    }

    println!("Saved to students.txt");

    Ok(())
}
``````````````````````````````````````````````````````````````````````````````````````````````````````````````````````````````` 