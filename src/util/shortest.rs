// Get the first shortest value in a vector
pub fn get_shortest_value(data: Vec<&str>) -> &str {
    let mut value: &str = "";

    for i in data.into_iter() {
        if value.len() == 0 {
            value = i
        }

        if &value.len() > &i.len() {
            value = i;
        }
    }

    &value
}
