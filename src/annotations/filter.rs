use crate::{annotations::models::Annotation, events::models::NotEvent};

pub fn filter_annotation_by_events(
    annotations: Vec<Annotation>,
    event: Vec<NotEvent>,
) -> Vec<Annotation> {
    annotations
        .into_iter()
        .filter(|annotation| event.contains(&annotation.event))
        .collect()
}
