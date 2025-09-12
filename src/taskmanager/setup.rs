use crate::taskmanager::{database::Database, task_manager::TaskManager, utils::read_number};

pub fn run() {
    let input = std::io::stdin();

    println!("=-=-=seja bem vindo ao gerenciador de tarefas=-=-=\n");

    init();

    let mut task_manager = TaskManager::new();

    loop {
        println!(
            "Qual ação deseja realizar?\n1ºListar tarefas\n2ºCriar tarefa\n3ºRemover tarefa\n4ºEncontrar tarefa (Nome)\n5ºMudar status de uma tarefa"
        );

        let command = read_number();

        let command = match command {
            Some(number) => number,
            None => {
                println!("A ação deve ser um número\n");
                continue;
            }
        };

        if command == 0 {
            task_manager.close();
            break;
        }

        task_manager.action(command);
    }

    println!("Encerrando...");
}

fn init() {
    Database::confirm_database_file();
}