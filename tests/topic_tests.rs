use nodex::topic::Topic;

#[test]
fn test_topic_from_json() {
    let topic: Topic = Topic::from_json("tests/test_files/topic_1.json");
    println!("{:?}", topic)
}
