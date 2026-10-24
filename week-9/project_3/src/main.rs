use std::io::Write;

fn main() {
    let commisioner:[&str;6] = [
        "Commisioner",
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye"
    ];

    let ministry:[&str;6] = [
        "Ministry",
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum"
    ];

    let zone:[&str;6] = [
        "Zone",
        "South West",
        "North East",
        "South South",
        "South West",
        "South East"
    ];

    let mut output = String::new();

    for i in 0..6 {
        let line = format!("{}, {}, {}\n",commisioner[i], ministry[i], zone[i]);
        output.push_str(&line);
    }

    let mut file = std::fs::File::create("EFCC_Criminals_merge_table.csv").expect("create failed");
    file.write_all(output.as_bytes()).expect("write failed");

    println!("\nYour table has been successfully created.");
}
