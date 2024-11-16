#[cfg(test)]
pub mod test_utils {
    pub fn merge_json_objects(target: &mut serde_json::Value, source: serde_json::Value) {
        // Convert both to objects
        let target_obj = target
            .as_object_mut()
            .expect("could not convert target to mutable object");
        let source_obj = source
            .as_object()
            .expect("could not convert source to object");

        // Merge source into target
        for (k, v) in source_obj {
            target_obj.insert(k.clone(), v.clone());
        }
    }
}
