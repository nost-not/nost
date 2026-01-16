use std::path::PathBuf;

use crate::annotations::models::Annotation;

pub struct WorkAnnotationWithPath {
    pub annotation: Annotation,
    pub path: PathBuf,
}
