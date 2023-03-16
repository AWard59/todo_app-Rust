fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    let item1 = TodoItem {
        id: 1,
        name: String::from("Buy milk"),
        completed: false,
    };

    let item2 = TodoItem {
        id: 2,
        name: String::from("Walk the dog"),
        completed: false,
    };

    todo_list.push(item1);
    todo_list.push(item2);

    println!("{:?}", todo_list);
}

fn complete_item(item: &mut TodoItem) {
    item.completed = true;
}

fn display_items(items: &Vec<TodoItem>) {
    for item in items {
        println!("{} - {} ({})", item.id, item.name, item.completed);
    }
}

struct TodoItem {
    id: u32,
    name: String,
    completed: bool,
}