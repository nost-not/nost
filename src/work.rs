use crate::not::append;
use crate::not::get_or_create_not;

pub fn start_work() {
    let not_path = get_or_create_not(None).unwrap();
    let work_content = "\"not: {start-work: '<start-date>', salary: '<salary-value>', salary-currency: '<currency-value>'}\"";
    let content = format!("[//]: # {}\n", work_content);
    append(not_path.clone().into(), &content).expect("Failed to append not metatadata.");

    println!("✅ Start work metadata successfully added to the note.");
}

pub fn stop_work() {
    let not_path = get_or_create_not(None).unwrap();
    let content = "not: {end-work: '<end-date>'}";
    append(not_path.clone().into(), &content).expect("Failed to append not metatadata.");

    println!("🛑 Stopping work.");
}
