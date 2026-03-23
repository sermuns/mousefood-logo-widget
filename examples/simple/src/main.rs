use mousefood_logo_widget::MouseFoodLogo;
use ratatui::crossterm::event;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget(MouseFoodLogo::new(), frame.area()))?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}
