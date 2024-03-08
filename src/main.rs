use std::fmt;
//use std::sync::{ Arc, RwLock };
use std::cell::RefCell;
use std::rc::Rc;

struct TodoItem {
    id: u64,
    name: String,
    completed: bool,
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ id => {}, name => {}, completed => {} }}",
            self.id, self.name, self.completed
        )
    }
}

struct TodoList {
    items: Vec<Rc<RefCell<TodoItem>>>,
}

#[allow(dead_code)]
impl TodoList {
    fn new() -> TodoList {
        TodoList { items: vec![] }
    }
    fn print_items(&self) {
        println!("******");
        for item in self.items.iter() {
            //let temp: &Rc<RefCell<TodoItem>> = item.borrow();
            println!("{}", item.borrow());
        }
    }
    fn print_items_v2(&mut self) {
        println!("******");
        for item in self.items.iter_mut() {
            println!("{}", item.borrow());
        }
    }
    fn add_item(&mut self, name: &String, completed: bool) -> u64 {
        let mut id = 1;
        let mut prev_id: u64 = id;
        for item in self.items.iter() {
            if prev_id < item.borrow().id - 1 {
                id = prev_id;
                break;
            } else if item.borrow().id >= id {
                prev_id = id;
                id = item.borrow().id + 1;
            }
        }

        let vec_len = self.items.len();
        let my_index = usize::try_from(id).unwrap_or(vec_len + 1);

        //if vec_len > 0 {
        self.items.insert(
            my_index - 1,
            Rc::new(RefCell::new(TodoItem {
                id: id,
                name: name.clone(),
                completed,
            })),
        );
        /*} else {
            self.items.push(TodoItem {
                id: id,
                name: name.clone(),
                completed,
            });
        }*/

        id
    }
    fn remove_item(&mut self, id: u64) -> Option<Rc<RefCell<TodoItem>>> {
        let mut result = None;
        for (index, item) in self.items.iter().enumerate() {
            if item.borrow().id == id {
                result = Some(self.items.remove(index));
                break;
            }
        }
        result
    }
    fn mark_item_v1(&mut self, id: u64, completed: bool) -> Option<&Rc<RefCell<TodoItem>>> {
        let mut result = None;
        for item in self.items.iter_mut() {
            if item.borrow().id == id {
                item.borrow_mut().completed = completed;
                let temp: &Rc<RefCell<TodoItem>> = item; //remove the mutability to var item
                result = Some(temp);
            }
        }
        result
    }
    fn mark_item_v2(&mut self, id: u64, completed: bool) -> Option<&mut Rc<RefCell<TodoItem>>> {
        let mut result = None;
        for item in self.items.iter_mut() {
            if item.borrow().id == id {
                item.borrow_mut().completed = completed;
                //let temp: &TodoItem = item; //remove the mutability to var item
                result = Some(item);
            }
        }
        result
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_item(&"Test01".to_string(), false);
    todo_list.add_item(&"Test02".to_string(), false);
    todo_list.add_item(&"Test03".to_string(), false);

    todo_list.print_items();

    match todo_list.remove_item(10) {
        Some(todo_item) => {
            println!(
                "Sucess removed item with id {} from list {}",
                10,
                todo_item.borrow()
            );
        }
        None => {
            println!("Item with id {} not found", 10);
        }
    }

    todo_list.print_items();

    match todo_list.remove_item(1) {
        Some(todo_item) => {
            println!(
                "Sucess removed item with id {}. TodoItem: {}",
                1,
                todo_item.borrow()
            );
        }
        None => {
            println!("Item with id {} not found", 1);
        }
    }

    todo_list.print_items();

    todo_list.add_item(&"Test01".to_string(), false);

    todo_list.print_items();

    //let todo_item = todo_list.mark_item_v1(2, true);

    //todo_list.print_items();

    // match todo_list.mark_item_v1(10, true) {
    //     Some(todo_item) => {
    //         println!(
    //             "Sucess removed item with id {}. TodoItem: {}",
    //             10, todo_item
    //         );
    //     }
    //     None => {
    //         println!("Item with id {} not found", 10);
    //     }
    // };

    // if  todo_item.is_some() {

    //     let mut x: &mut TodoItem = todo_item.unwrap();
    //     x.completed = false;

    // }

    todo_list.print_items();

    let todo_item = todo_list.mark_item_v2(2, true);

    let todo_item_cloned = todo_item.cloned(); //Important to allow the next line. With no borrow checker complaints

    todo_list.print_items();

    if todo_item_cloned.is_some() {
        println!(
            "Sucess marked item with id {} to {}. TodoItem: {}",
            2,
            true,
            todo_item_cloned.as_ref().unwrap().borrow()
        );

        let temp = todo_item_cloned.unwrap();

        let todo_item: &mut TodoItem = &mut temp.borrow_mut();
        todo_item.completed = false;
    }

    todo_list.print_items();
}
