use crate::{annotations::models::Annotation, events::models::EventName};

pub fn filter_annotation_by_events(
    annotations: Vec<Annotation>,
    event: Vec<EventName>,
) -> Vec<Annotation> {
    annotations
        .into_iter()
        .filter(|annotation| event.contains(&annotation.event))
        .collect()
}
