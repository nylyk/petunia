pub mod linking;
pub mod main;

pub use linking::Linking;
pub use main::Main;

pub enum Screen {
    Linking(Linking),
    Main(Box<Main>),
}
