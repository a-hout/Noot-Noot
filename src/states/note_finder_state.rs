use iced::Element;
use iced::widget::{button, container, row, column};
use iced::widget::container::rounded_box;
/*
STATE LOGIC
*/
#[derive(Default)]
pub struct NoteFinder
{
    note: u8, //semitone difference between reference frequency note
    octave: i8,
    on_play: bool,
}

/*
MESSAGE LOGIC
*/
#[derive(Debug, Clone, Copy)]
pub enum Message // TODO think of a better way to do this...
{
    A,
    As,
    Bb,
    B,
    C,
    Cs,
    Db,
    D,
    Ds,
    Eb,
    E,
    F,
    Fs,
    Gb,
    G,
    Gs,
    Ab,
    Play,
}

/*
TODO implement f * 2^(note/12) and play sound

probably should divide by two or four to get in range of guitar tones
*/


impl NoteFinder
{
    /*
    VIEW LOGIC
    */
    pub fn view(&self) -> Element<Message>
    {
        container(
            column![
        row![
            button("A").on_press(Message::A),
            //button("As").on_press(Message::As),
            button("Bb").on_press(Message::Bb),
            button("B").on_press(Message::B),
            button("C").on_press(Message::C),
            button("Cs").on_press(Message::Cs),
            //button("Db").on_press(Message::Db),
            button("D").on_press(Message::D),
            //button("Ds").on_press(Message::Ds),
            button("Eb").on_press(Message::Eb),
            button("E").on_press(Message::E),
            button("F").on_press(Message::F),
            button("Fs").on_press(Message::Fs),
            //button("Gb").on_press(Message::Gb),
            button("G").on_press(Message::G),
            //button("Gs").on_press(Message::Gs),
            button("Ab").on_press(Message::Ab),],
            button("Play").on_press(Message::Play)]

        ).style(rounded_box).padding(10).into()


    }

    /*
    UPDATE LOGIC
    */
    pub fn update(&mut self, message: Message)
    {
        match message
        {
            Message::A => {self.note = 0;}
            Message::As => {self.note = 1;}
            Message::Bb => {self.note = 1;}
            Message::B => {self.note = 2;}
            Message::C => {self.note = 3;}
            Message::Cs => {self.note = 4;}
            Message::Db => {self.note = 4;}
            Message::D => {self.note = 5;}
            Message::Ds => {self.note = 6;}
            Message::Eb => {self.note = 6;}
            Message::E => {self.note = 7;}
            Message::F => {self.note = 8;}
            Message::Fs => {self.note = 9;}
            Message::Gb => {self.note = 9;}
            Message::G => {self.note = 10;}
            Message::Gs => {self.note = 11;}
            Message::Ab => {self.note = 11;}
            Message::Play => {self.on_play ^= true;}
        }
    }
}