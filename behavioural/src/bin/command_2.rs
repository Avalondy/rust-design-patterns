trait Command {
    fn execute(&self);
}

struct MacroCommand {
    stack: Vec<Box<dyn Command>>,
}

impl MacroCommand {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn append(&mut self, cmd: Box<dyn Command>) {
        self.stack.push(cmd);
    }

    fn undo(&mut self) {
        self.stack.pop();
    }

    fn clear(&mut self) {
        self.stack.clear();
    }
}

impl Command for MacroCommand {
    fn execute(&self) {
        self.stack.iter().for_each(|cmd| cmd.execute());
    }
}

trait Drawable {
    fn draw(&self, x: u32, y: u32);
}

struct DrawCommand {
    drawable: Box<dyn Drawable>,
    x: u32,
    y: u32,
}

impl DrawCommand {
    fn new(drawable: Box<dyn Drawable>, x: u32, y: u32) -> Self {
        Self { drawable, x, y }
    }
}

impl Command for DrawCommand {
    fn execute(&self) {
        self.drawable.draw(self.x, self.y);
    }
}

#[derive(Clone)]
struct DrawCanvas;

impl DrawCanvas {
    fn new() -> Self {
        Self
    }
}

impl Drawable for DrawCanvas {
    fn draw(&self, x: u32, y: u32) {
        println!("draw(x:{}, y:{})", x, y);
    }
}

fn main() {
    let mut history = MacroCommand::new();
    let canvas = Box::new(DrawCanvas::new());

    let cmd1 = Box::new(DrawCommand::new(canvas.clone(), 1, 1));
    let cmd2 = Box::new(DrawCommand::new(canvas.clone(), 2, 2));

    history.append(cmd1);
    history.append(cmd2);

    println!("----------");
    history.execute();
    println!();

    println!("---undo---");
    history.undo();
    history.execute();
    println!();

    println!("---clear---");
    history.clear();
    history.execute();
}
