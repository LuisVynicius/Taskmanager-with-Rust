use crate::taskmanager::{database::Database, utils::read_number};

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

                let option = read_number();

                let option = match option {
                    Some(number) => number,
                    None => {
                        println!("A opção deve ser um número");
                        return;
                    }
                };

                match option {
                    1 => self.database.delete_by_name(),
                    2 => self.database.delete_by_position(),
                    3 => self.database.delete_by_code(),
                    _ => println!("A opção deve ser um número de 1 à 3.")
                }
            }
            4 => {
                println!("Encontrar tarefa\n1ºPor nome\n2ºPor posição\n3ºPor código");

                let option = read_number();

                let option = match option {
                    Some(number) => number,
                    None => {
                        println!("A opção deve ser um número");
                        return;
                    }
                };

                match option {
                    1 => self.database.find_by_name(),
                    2 => self.database.find_by_position(),
                    3 => self.database.find_by_code(),
                    _ => println!("A opção deve ser um número de 1 à 4.")
                }
            }
            5 => {
                println!("Encontrar tarefa por código");

                self.database.change_status();
            },
            _ => {
                println!("A ação deve ser um número de 1 à 5.")
            }
        }

        println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n");
    }

    pub fn close(&mut self) {
        self.database.save();
    }
}
