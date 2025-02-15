use std::io::Result;

use items::Bow;
use persistence::{GameObject, TxtFileSaver};
use player::Player;

pub mod items {
    use std::io::Result;

    use crate::persistence::{GameObject, GameObjectVisitor};

    pub struct Bow {
        pub damage: i32,
        pub range: f32,
    }

    impl GameObject for Bow {
        fn accept(&self, visitor: &dyn GameObjectVisitor) -> Result<()> {
            visitor.visit_bow(&self)
        }
    }
}

pub mod player {
    use std::io::Result;

    use crate::persistence::{GameObject, GameObjectVisitor};

    pub struct Player {
        pub position: (f32, f32),
        pub health: i32,
    }

    impl GameObject for Player {
        fn accept(&self, visitor: &dyn GameObjectVisitor) -> Result<()> {
            visitor.visit_player(&self)
        }
    }
}

pub mod persistence {
    use std::{
        fs::{self, File, OpenOptions},
        io::{Result, Write},
    };

    use crate::{items::Bow, player::Player};

    pub trait GameObjectVisitor {
        fn visit_player(&self, player: &Player) -> Result<()>;
        fn visit_bow(&self, bow: &Bow) -> Result<()>;
    }

    pub trait GameObject {
        fn accept(&self, visitor: &dyn GameObjectVisitor) -> Result<()>;
    }

    pub struct TxtFileSaver {
        save_dir: String,
    }

    impl TxtFileSaver {
        pub fn new(save_dir: &str) -> Result<TxtFileSaver> {
            fs::create_dir_all(save_dir)?;
            let txt_file_saver = TxtFileSaver {
                save_dir: save_dir.to_string(),
            };
            Ok(txt_file_saver)
        }

        fn open_file(&self, struct_name: &str) -> Result<File> {
            let filename = format!("{}/{}.txt", self.save_dir, struct_name);
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(filename)
        }

        fn write_to_file(&self, struct_name: &str, content: String) -> Result<()> {
            let mut file = self.open_file(struct_name)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }
    }

    impl GameObjectVisitor for TxtFileSaver {
        fn visit_player(&self, player: &Player) -> Result<()> {
            let mut content = format!("Position: [{}, {}]\n", player.position.0, player.position.1);
            content.push_str(&format!("Health: {}", player.health));

            self.write_to_file("player", content)
        }

        fn visit_bow(&self, bow: &Bow) -> Result<()> {
            let mut content = format!("Damage: {}\n", bow.damage);
            content.push_str(&format!("Range: {}", bow.range));

            self.write_to_file("bow", content)
        }
    }
}

fn main() -> Result<()> {
    let player = Player {
        position: (50.0, 50.0),
        health: 100,
    };

    let bow = Bow {
        damage: 12,
        range: 18.3,
    };

    let game_objects: Vec<&dyn GameObject> = vec![&player, &bow];
    let txt_file_saver = TxtFileSaver::new("save_files")?;

    for game_object in game_objects {
        game_object.accept(&txt_file_saver)?;
    }

    Ok(())
}
