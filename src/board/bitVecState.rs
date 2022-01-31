
use bitvec::prelude::*;

use crate::{WIDTH, HEIGHT};

const ARR_LEN: usize = ((((WIDTH as u16*HEIGHT as u16)*2)+(((WIDTH as u16 *HEIGHT as u16)*2)%8)) as usize)/8;
const BIT_LEN: usize = (((WIDTH as u16*HEIGHT as u16)*2)+(((WIDTH as u16 *HEIGHT as u16)*2)%8)) as usize;
#[derive(Copy,Clone)]
pub struct State{
  pub state: BitArray<[u8;ARR_LEN]>,
  pub width: u8,
  pub height: u8,
  pub size: u16
}
impl State {
  pub fn new() -> State {
    return State { state: bitarr![u8, Lsb0; 0; BIT_LEN], width: WIDTH, height: HEIGHT ,size:WIDTH as u16 * HEIGHT as u16}
  } 
  pub fn set(&mut self,index: usize,team: u8){
    unsafe{
      self.state.as_mut_bitptr().add(index*2).replace(team == 1);
      self.state.as_mut_bitptr().add((index*2)+1).replace(team == 2);
    }

  }
  pub fn get(&self,index: usize) -> Option<u8>{
    if index as u16>= self.size{
      return None
    }
    unsafe{
      let slice = self.state.get_unchecked((index*2)..((index*2)+2));
      let mut res: u8 = 1;
      res <<= slice[0] as u8;
      res <<= slice[1] as u8;
      res <<= slice[1] as u8;
      res >>= 1;
      return Some(res);
      
    }
    // match [val1,val2] {
    //   [true,false] => {return Some(1)}
    //   [false,true] => {return Some(2)}
    //   [false,false] => {return Some(0)}
    //   _ => {return Some(0)}
    // }

  }
}

