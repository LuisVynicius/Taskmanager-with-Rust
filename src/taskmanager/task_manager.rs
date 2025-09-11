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
                self.database.show_tasks();
            }
            2 => {
                self.database.create_task();
            }
            3 => {}
            4 => {}
            _ => {}
        }

        println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n");
    }

    pub fn close(&mut self) {
        self.database.save();
    }
}
