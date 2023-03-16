Step 1: Setting Up Your Rust Development Environment
Before we can start building our to-do app in Rust, we need to set up our development environment. Rust is available on Windows, macOS, and Linux, and you can download it from the official Rust website. Once you've installed Rust, you'll also need to install Cargo, the Rust package manager that manages Rust projects and dependencies.

Step 2: Creating a New Rust Project
To create a new Rust project, we'll use Cargo's new command. Open up your terminal and navigate to the directory where you want to create your project. Then, type the following command:
cargo new todo_app
This command creates a new Rust project called todo_app, which includes a src directory where we'll put our Rust code.

Step 3: Defining the To-Do Item Struct
In Rust, we define data types using structs. To represent a to-do item in our app, we'll create a new struct called TodoItem. Open up the src/main.rs file in your text editor and add the following code:
struct TodoItem {
    id: u32,
    name: String,
    completed: bool,
}
This code defines a new struct called TodoItem that has three fields: id, name, and completed. The id field is a unique identifier for each to-do item, the name field is a string that contains the name of the to-do item, and the completed field is a boolean that indicates whether the to-do item is completed or not.

Step 4: Adding To-Do Items to a Vector
In Rust, we can use vectors to store collections of items. To add to-do items to our app, we'll create a new vector called todo_list and add some to-do items to it. Add the following code to the main function in your src/main.rs file:
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
This code creates a new vector called todo_list that contains two to-do items: "Buy milk" and "Walk the dog". We then push these to-do items onto the vector using the push method. Finally, we print out the contents of the vector using Rust's println macro.

Step 5: Updating To-Do Items
To update a to-do item in our app, we'll create a new function called complete_item that takes a mutable reference to a TodoItem and sets its completed field to true. Add the following code to your src/main.rs file:
fn complete_item(item: &mut TodoItem) {
    item.completed = true;
}
This code defines a new function called complete_item that takes a mutable reference

to a TodoItem and sets its completed field to true.

Step 6: Displaying To-Do Items
To display the to-do items in our app, we'll create a new function called display_items that takes a vector of TodoItems and prints out each item's id, name, and completed status. Add the following code to your src/main.rs file:
fn display_items(items: &Vec<TodoItem>) {
    for item in items {
        println!("{} - {} ({})", item.id, item.name, item.completed);
    }
}
This code defines a new function called display_items that takes a reference to a vector of TodoItems and iterates over each item in the vector using a for loop. For each item, it prints out its id, name, and completed status using Rust's println macro.

Step 7: Accepting User Input
To allow users to add and complete to-do items in our app, we'll use Rust's std::io module to accept user input from the command line. Add the following code to your src/main.rs file:
use std::io;

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        println!("What would you like to do?");
        println!("1. Add a to-do item");
        println!("2. Complete a to-do item");
        println!("3. Display to-do items");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice = choice.trim().parse::<u32>().expect("Invalid input");

        match choice {
            1 => {
                println!("Enter the name of the to-do item:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                let id = todo_list.len() as u32 + 1;

                let item = TodoItem {
                    id,
                    name,
                    completed: false,
                };

                todo_list.push(item);
            },
            2 => {
                println!("Enter the ID of the to-do item you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");

                let item = todo_list.iter_mut().find(|i| i.id == id).unwrap();
                complete_item(item);
            },
            3 => {
                display_items(&todo_list);
            },
            4 => {
                println!("Goodbye!");
                return;
            },
            _ => {
                println!("Invalid choice");
            },
        }
    }
}
This code adds a use statement for Rust's std::io module, which allows us to read user input from the command line. We then define a new main function that contains a loop that runs until the user chooses to quit. Inside the loop, we display a menu of options using Rust's println macro and read the user's choice using Rust's stdin function.

Depending on the user's choice, we either add a new to-do item to the todo_list vector, complete an existing to-do item, display the contents of the todo_list vector, or quit the app. To add a new to-do item, we read the user's input for the item's name, generate a new id for the item by adding one to the length of the todo_list vector, create a new TodoItem struct with the id, name, and completed fields, and push the new item onto the end of the todo_list vector.

To complete an existing to-do item, we read the user's input for the item's id, find the corresponding item in the todo_list vector using the iter_mut method and Rust's find function, call the complete_item function to mark the item as completed, and print out a confirmation message.

To display the contents of the todo_list vector, we call the display_items function and pass it a reference to the todo_list vector.

Step 8: Testing the App
Now that we've implemented all the core functionality of our to-do app, let's test it out! In your terminal, navigate to your project directory and run the following command to build and run the app:
cargo run
This command will compile your Rust code and run the resulting binary. You should see the following menu:
What would you like to do?
1. Add a to-do item
2. Complete a to-do item
3. Display to-do items
4. Quit
Try adding some to-do items, completing them, and displaying the list of items. You should see the list of items displayed in the terminal, with each item's id, name, and completed status printed out.
Congratulations! You've built a fully functional to-do app in Rust.