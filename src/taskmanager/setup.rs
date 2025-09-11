use super::{database::Database, task_manager::TaskManager};

pub fn run() {
    let input = std::io::stdin();

    println!("=-=-=seja bem vindo ao gerenciador de tarefas=-=-=\n");

    init();

    let mut task_manager = TaskManager::new();

    loop {
        let mut command = String::new();

        println!(
            "Qual ação deseja realizar?\n1 - Listar tarefas\n2 - Criar tarefa\n3 - Remover tarefa\n4 - Encontrar tarefa (Nome)\n5 - Encontrar tarefa (Posição)\n6 - Encontrar tarefa (Código)"
        );

        input.read_line(&mut command).unwrap();

        let command = command.trim().parse::<u8>().unwrap();

        if command == 0 {
            task_manager.close();
            break;
        }

        println!();

        task_manager.action(command);
    }

    println!("Encerrando...");
}

fn init() {
    Database::confirm_database_file();
}
