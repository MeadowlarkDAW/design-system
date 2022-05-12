use vizia::prelude::*;

fn main() {
    Application::new(|cx|{
        Label::new(cx, "Test");
    })
    .run();
}
