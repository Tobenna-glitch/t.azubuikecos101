struct MinisterRecord {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() {

    let names: Vec<&str> = vec![
    "Aigbogun Alamba Daudu",
    "Murtala Afeez Bendu",
    "Okorocha Calistus Ogbona",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etieye"];

    let ministries: Vec<&str> = vec![
    "Internal Affairs",
    "Justice",
    "Defense",
    "Power & Steel",
    "Petroleum"];

    let geopolitical_Zone: Vec<&str> = vec![
    "South West",
    "North East",
    "South South",
    "South West",
    "South East"];

    let num_records = names.len();
    let mut merged_data: Vec<MinisterRecord> = Vec::new();

    for i in 0..num_records {
        let name_data = names[i];
        let ministry_data = ministries[i];
        let geopolitical_zone_data = geopolitical_Zone[i];

        let record = MinisterRecord {
            name: name_data.to_string(),
            ministry: ministry_data.to_string(),
            geopolitical_zone: geopolitical_zone_data.to_string(),
        };

        merged_data.push(record);
    }
    println!("\n EFCC Consulidated Convicted Minister Files");
    println!("-------------------------------------------------------------------------------------------");
    println!("{:<3} | {:<30} | {:<20} | {:<20}", "S/N", "NAME OF MINISTER", "MINISTRY", "GEOPOLITICAL ZONE");
    println!("--------------------------------------------------------------------------------------------");

    for (i, record) in merged_data.iter().enumerate() {
        println!("{:<3} | {:<30} | {:<20} | {:<20}", 
                 i + 1, // Start S/N from 1
                 record.name, 
                 record.ministry, 
                 record.geopolitical_zone);
    }
    println!("---------------------------------------------------------------------------------");
}



