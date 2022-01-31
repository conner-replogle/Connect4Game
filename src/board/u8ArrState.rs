use crate::{WIDTH, HEIGHT};
const SIZE:u16 = WIDTH as u16 *HEIGHT as u16;
#[derive(Copy,Clone)]
pub struct State{
  pub state: [u8;SIZE as usize],
  pub width: u8,
  pub height: u8,
  pub size: u16
}
impl IntoIterator for State {
  type Item = u8;
  type IntoIter = std::array::IntoIter<Self::Item,{SIZE as usize}>;
  fn into_iter(self) -> Self::IntoIter{
    return self.state.into_iter()
  }
}
impl State {
  pub fn new() -> State {
    return State { state: [0u8; SIZE as usize], width: WIDTH, height: HEIGHT ,size:WIDTH as u16 * HEIGHT as u16}
  } 
  pub fn set(&mut self,index: usize,team: u8){
    self.state[index] = team;
  }
  pub fn get(&self,index: usize) -> Option<u8>{
    return Some(self.state[index]);

  }
}
