use uuid::Uuid;

use crate::{dates::get::get_now_as_string, events::models::NotEvent, files::append::append};

pub fn annotate(
    date: Option<&str>,
    event: NotEvent,
    input_uid: Option<&Uuid>,
    not_path: &str,
    workday: Option<&str>,
) {
    let now = get_now_as_string();
    let date = match date {
        Some(d) => d,
        None => &now,
    };

    let new_uid = Uuid::new_v4().to_string();
    let uid = match input_uid {
        Some(u) => u.to_string(),
        None => new_uid,
    };

    workday.unwrap_or_default();

    let content = if Option::<&str>::is_some(&workday) {
        format!(
            "\"not:{{date:'{}',event:'{}',uid:'{}',workday:'{}'}}\"",
            date,
            event,
            uid,
            workday.unwrap()
        )
    } else {
        format!(
            "\"not:{{date:'{}',event:'{}',uid:'{}'}}\"",
            date, event, uid
        )
    };

    let annotation = format!("[//]: # {}\n", content);
    append(not_path.into(), &annotation).expect("🛑 Failed to annotate.");
}

#[cfg(test)]
mod tests {
    #[test]
    #[serial_test::serial]
    fn test_annotate() {
        use std::fs;
        use std::io::Read;
        use tempfile::tempdir;

        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_annotate.txt");

        // Create the file first
        fs::File::create(&file_path).unwrap();

        // Call annotate
        super::annotate(
            None,
            crate::events::models::NotEvent::CreateNot,
            None,
            file_path.to_str().unwrap(),
            None,
        );

        // Read back the content
        let mut file = fs::File::open(&file_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        // The annotation should be wrapped as [//]: # "..."
        let annotation_regex =
            regex::Regex::new(r#"\[//\]: # "not:\{date:'.*',event:'CREATE_NOT',uid:'.*'\}""#)
                .unwrap();
        assert!(
            file_content
                .lines()
                .any(|line| annotation_regex.is_match(line)),
            "Annotation with expected format not found in file content"
        );
    }
}
