mod states;
mod tests;

use iced;
use iced::Theme;
use crate::states::note_finder_state::NoteFinder;

//TODO a main interface composited of the other parts should be the one getting instanced

pub fn main() -> iced::Result
{
    // TODO run main interface
    iced::application("Noot-Noot", NoteFinder::update, NoteFinder::view).theme(theme).run()
}

fn theme(state: &NoteFinder) -> Theme {
    Theme::Ferra
}