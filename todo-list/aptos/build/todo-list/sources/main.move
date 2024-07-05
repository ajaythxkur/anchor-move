module todolist_addr::main{
    use std::string::String;
    use std::table::{Self,Table};
    use std::signer::address_of;
    use std::error;

    struct TodoList has key{
        tasks: Table<u64,Task>,
        task_counter: u64
    }

    struct Task has store, drop, copy{
        task_id: u64,
        address: address,
        content: String,
        completed: bool,
    }

    const ENOT_FOUND: u64 = 1;
    const ETASK_NOT_FOUND: u64 = 2;
    const ETASK_ALREADY_COMPLETED: u64 = 3;

    public entry fun create_task(account: &signer, content: String) acquires TodoList{
        let account_addr = address_of(account);
        if(exists<TodoList>(account_addr)){
            let todo_list = borrow_global_mut<TodoList>(account_addr);
            let counter = todo_list.task_counter + 1;
            let new_task = Task {
                task_id: counter,
                address: account_addr,
                content,
                completed: false,
            };
            table::add(&mut todo_list.tasks, counter, new_task);
            todo_list.task_counter = counter;
        }else{
            let task_holder = TodoList{
                tasks: table::new<u64, Task>(),
                task_counter: 1
            };
            let new_task = Task{
                task_id: 1,
                address: account_addr,
                content,
                completed: false
            };
            table::add(&mut task_holder.tasks, 1, new_task);
            move_to(account, task_holder);
        }
    }

    public entry fun complete_task(account: &signer, task_id: u64) acquires TodoList{
        let account_addr = address_of(account);
        assert!(exists<TodoList>(account_addr), error::not_found(ENOT_FOUND));
        let todo_list = borrow_global_mut<TodoList>(account_addr);
        assert!(table::contains(&todo_list.tasks, task_id), error::not_found(ETASK_NOT_FOUND));
        let task_record = table::borrow_mut(&mut todo_list.tasks, task_id);
        assert!(task_record.completed == false, ETASK_ALREADY_COMPLETED);
        task_record.completed = true;
    }

    #[view]
    public fun get_task(account_addr: address, task_id: u64): (u64, address, String, bool) acquires TodoList{
        assert!(exists<TodoList>(account_addr), error::not_found(ENOT_FOUND));
        let todo_list = borrow_global_mut<TodoList>(account_addr);
        assert!(table::contains(&todo_list.tasks, task_id), error::not_found(ETASK_NOT_FOUND));
        let task_record = table::borrow(&todo_list.tasks, task_id);
        (task_record.task_id, task_record.address, task_record.content, task_record.completed)
    }
}