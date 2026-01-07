use crate::files::find::find_all_not_files;
use crate::{annotations::models::Annotation, annotations::parse::parse_annotation};
use regex::Regex;
use std::io::Result;
use std::path::PathBuf;

pub fn extract_field_from_annotation(annotation: &str, field: &str) -> Option<String> {
    let re = Regex::new(&format!(r#"{}:'(?P<value>[^']+)'"#, field)).unwrap();
    if let Some(caps) = re.captures(annotation) {
        if let Some(value) = caps.name("value") {
            return Some(value.as_str().to_string());
        }
    }
    None
}

pub fn extract_annotations_from_path(path: PathBuf) -> Result<Vec<Annotation>> {
    let paths = match find_all_not_files(path) {
        Ok(p) => p,
        Err(e) => {
            return Err(e);
        }
    };

    // get all annotations of the month
    let mut raw_annotations = Vec::new();
    for path in paths {
        let annotations_for_current_file = extract_annotations_from_one_file(&path).unwrap();
        raw_annotations.extend(annotations_for_current_file);
    }

    // refine the annotation removing the not prefix
    let re = Regex::new(r#"\[//\]: # "not:(\{.*\})""#).unwrap();

    let filtered_annotations: Vec<String> = raw_annotations
        .into_iter()
        .filter_map(|annotation| {
            re.captures(&annotation)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        })
        .collect();

    // convert text annotations into Annotation structs
    let mut annotations = Vec::new();

    filtered_annotations
        .into_iter()
        .for_each(|annotation| match parse_annotation(&annotation) {
            Ok(annotation_struct) => annotations.push(annotation_struct),
            Err(e) => eprintln!(
                "Failed to parse annotation: {}\n Annotation: {}\n",
                e, annotation
            ),
        });

    Ok(annotations)
}

pub fn extract_annotations_from_one_file(file_path: &PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(file_path)?;
    let re = Regex::new(r#"^\[//\]: # "not.*"\s*$"#).unwrap();

    let extracted: Vec<String> = content
        .lines()
        .filter_map(|line| {
            if re.is_match(line) {
                Some(line.to_string())
            } else {
                None
            }
        })
        .collect();

    Ok(extracted)
}

#[cfg(test)]
mod tests {
    use crate::annotations::extract::extract_field_from_annotation;

    #[test]
    #[serial_test::serial]
    fn extract_uid_from_annotation() {
        let annotation = "not:{uid:'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff',created_at:'2025-09-29T00:00:43.245684903+02:00',event:'START_WORK'}".to_string();
        let uid = extract_field_from_annotation(&annotation, "uid");
        assert_eq!(
            uid.unwrap().to_string(),
            "b86bc6ed-50a5-4ef2-bdd3-e17baef11eff"
        );
    }

    #[test]
    #[serial_test::serial]
    fn extract_datetime_from_annotation() {
        let annotation = "not:{uid:'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff',created_at:'2025-09-29T00:00:43.245684903+02:00',event:'START_WORK'}".to_string();
        let datetime = extract_field_from_annotation(&annotation, "created_at");
        assert_eq!(
            datetime.unwrap().to_string(),
            "2025-09-29T00:00:43.245684903+02:00"
        );
    }

    #[test]
    #[serial_test::serial]
    fn extract_event_from_annotation() {
        let annotation = "not:{uid:'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff',created_at:'2025-09-29T00:00:43.245684903+02:00',event:'START_WORK'}".to_string();
        let event = extract_field_from_annotation(&annotation, "event");
        assert_eq!(event.unwrap().to_string(), "START_WORK");
    }
}
