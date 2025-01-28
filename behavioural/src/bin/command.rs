mod using_trait_objects {
    pub trait Migration {
        fn execute(&self) -> &str;
        fn rollback(&self) -> &str;
    }

    pub struct CreateTable;
    impl Migration for CreateTable {
        fn execute(&self) -> &str {
            "create table"
        }

        fn rollback(&self) -> &str {
            "drop table"
        }
    }

    pub struct AddField;
    impl Migration for AddField {
        fn execute(&self) -> &str {
            "add field"
        }

        fn rollback(&self) -> &str {
            "remove field"
        }
    }

    struct Schema {
        commands: Vec<Box<dyn Migration>>,
    }

    impl Schema {
        fn new() -> Self {
            Self { commands: vec![] }
        }

        fn add_migration(&mut self, cmd: Box<dyn Migration>) {
            self.commands.push(cmd);
        }

        fn execute(&self) -> Vec<&str> {
            self.commands.iter().map(|cmd| cmd.execute()).collect()
        }

        fn rollback(&self) -> Vec<&str> {
            self.commands
                .iter()
                .rev()
                .map(|cmd| cmd.rollback())
                .collect()
        }
    }

    pub fn test() {
        let mut schema = Schema::new();

        let cmd = Box::new(CreateTable);
        schema.add_migration(cmd);
        let cmd = Box::new(AddField);
        schema.add_migration(cmd);

        println!("{:?}", schema.execute());
        println!("{:?}", schema.rollback());
    }
}

mod using_function_pointers {
    type FnPtr = fn() -> String;
    struct Command {
        execute: FnPtr,
        rollback: FnPtr,
    }

    struct Schema {
        commands: Vec<Command>,
    }

    impl Schema {
        fn new() -> Self {
            Self { commands: vec![] }
        }

        fn add_migration(&mut self, execute: FnPtr, rollback: FnPtr) {
            self.commands.push(Command { execute, rollback });
        }

        fn execute(&self) -> Vec<String> {
            self.commands.iter().map(|cmd| (cmd.execute)()).collect()
        }

        fn rollback(&self) -> Vec<String> {
            self.commands
                .iter()
                .rev()
                .map(|cmd| (cmd.rollback)())
                .collect()
        }
    }

    fn add_field() -> String {
        "add field".to_string()
    }

    fn remove_field() -> String {
        "remove field".to_string()
    }

    pub fn test() {
        let mut schema = Schema::new();
        schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
        schema.add_migration(add_field, remove_field);

        println!("{:?}", schema.execute());
        println!("{:?}", schema.rollback());
    }
}

mod using_fn_trait_onjects {
    struct Schema<'a> {
        executes: Vec<Box<dyn Fn() -> &'a str>>,
        rollbacks: Vec<Box<dyn Fn() -> &'a str>>,
    }

    impl<'a> Schema<'a> {
        fn new() -> Self {
            Self {
                executes: vec![],
                rollbacks: vec![],
            }
        }

        fn add_migration<E, R>(&mut self, execute: E, rollback: R)
        where
            E: Fn() -> &'a str + 'static,
            R: Fn() -> &'a str + 'static,
        {
            self.executes.push(Box::new(execute));
            self.rollbacks.push(Box::new(rollback));
        }

        fn execute(&self) -> Vec<&str> {
            self.executes.iter().map(|cmd| cmd()).collect()
        }

        fn rollback(&self) -> Vec<&str> {
            self.rollbacks.iter().rev().map(|cmd| cmd()).collect()
        }
    }

    fn add_field() -> &'static str {
        "add field"
    }

    fn remove_field() -> &'static str {
        "remove field"
    }

    pub fn test() {
        let mut schema = Schema::new();
        schema.add_migration(|| "create table", || "drop table");
        schema.add_migration(add_field, remove_field);

        println!("{:?}", schema.execute());
        println!("{:?}", schema.rollback());
    }
}

fn main() {
    using_trait_objects::test();
    using_function_pointers::test();
    using_fn_trait_onjects::test();
}
