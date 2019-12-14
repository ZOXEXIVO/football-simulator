#[derive(Debug, Clone)]
pub trait PlayerMessage{

}

#[derive(Debug, Clone)]
pub struct PlayerMailbox {
    messages: Vec<Box<dyn PlayerMessage>>
}

