use super::database::Database;

pub struct TaskManager {
    database: Database,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            database: Database::new(),
        }
    }

    pub fn action(&mut self, command: u8) {
        println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");

        match command {
            1 => {
                println!("Mostrando tarefas\n");
                self.database.show_tasks();
            }
            2 => {
                println!("Criando tarefa\n");
                self.database.create_task();
            }
            3 => {
                println!("Remover tarefa\n1ºPor nome\n2ºPor posição\n3ºPor código");

                let mut option = String::new();

                std::io::stdin().read_line(&mut option).unwrap();

                let option = option.trim().parse::<u8>().unwrap();

                match option {
                    1 => self.database.delete_by_name(),
                    2 => self.database.delete_by_position(),
                    3 => self.database.delete_by_code(),
                    _ => panic!("The option must be a number between 1 and 3.")
                }
            }
            4 => {
                println!("Encontrar tarefa\n1ºPor nome\n2ºPor posição\n3ºPor código");

                let mut option = String::new();

                std::io::stdin().read_line(&mut option).unwrap();

                let option = option.trim().parse::<u8>().unwrap();

                match option {
                    1 => self.database.find_by_name(),
                    2 => self.database.find_by_position(),
                    3 => self.database.find_by_code(),
                    _ => panic!("The option must be a number between 1 and 3.")
                }
            }
            _ => {}
        }

        println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n");
    }

    pub fn close(&mut self) {
        self.database.save();
    }
}
