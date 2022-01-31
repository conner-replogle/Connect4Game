use std::{io::{self,Write}, time::Instant};

use self::u8ArrState::State;
mod u8ArrState;
// use self::bitVecState::State;
// mod bitVecState;
pub struct Board{
  pub state: State,
  pub team: u8
}

impl Board {
  pub fn new() -> Board{
    return Board{
    team: 1,
    state: State::new(),
    }
  }
  pub fn display_board(&self){
    let mut col: u8 = 1; 
    let now = Instant::now();

   // it prints '2'
    let mut board = String::new();
    for i in 0..(self.state.size) {
        let pos = self.state.get(i as usize).unwrap();
        board.push_str("|");
        board.push_str(&pos.to_string());
        board.push_str("|");
        if col == self.state.width{
            board.push_str("\n");
            col = 1;
        }else{
            col += 1;
        }
    }
    println!("{}",board);
    println!("took {} seconds to write to console", now.elapsed().as_secs_f64());
    for i in 0..self.state.width{
      if i >= 10{
        print!("{}|",i);
      }else{
        print!("|{}|",i);
      }
      
    }
    println!("")
  }
}
impl Board {
  pub fn check_win(&self) -> Option<u8> { 
    
    let w = self.state.width as usize;
    for i in 0..(self.state.size as u16 ){
      let index = i as usize;
      let team: u8 = self.state.get(index).unwrap();
      if team == 0{
        continue;
      }
      let row:u8 = (i/self.state.width as u16) as u8;
      let col:u8 = (i%self.state.width as u16) as u8;
      
      if row <self.state.height-3{
          if team == self.state.get(index + w).unwrap() && team == self.state.get(index + w*2).unwrap() && team == self.state.get(index + w*3).unwrap(){
              return Some(team);
          }
          if col <= self.state.width-4{
              if team == self.state.get(index + (w+1)).unwrap() && team == self.state.get(index + (w+1)*2).unwrap() && team == self.state.get(index + (w+1)*3).unwrap(){
                  return Some(team);
              }
          }
          if col >= 4{
              if team == self.state.get(index + (w-1)).unwrap() && team == self.state.get(index + (w-1)*2).unwrap() && team == self.state.get(index + (w-1)*3).unwrap(){
                  return Some(team);
              }
          }
      }
      if col < self.state.width-3{
          if team == self.state.get(index + 1).unwrap() && team == self.state.get(index + 2).unwrap() && team == self.state.get(index + 3).unwrap(){
              return Some(team);
          }
      }
  }

  return None
    
  }
  pub fn place(&mut self,place: usize,team: u8) -> Result<(), &str>{
    for i in (0..(self.state.height as usize )).rev() {
        let spot:usize = (place+i*(self.state.width as usize)).into();
        match self.state.get(spot) {
            Some(0) => {
                if i == (self.state.height as usize)-1{
                    self.state.set(spot,team);
                    return Ok(())
                }
                continue;
            }
            _ => {
                if i == 0{
                    return Err("coloumn filled")
                }
                let a = self.state.get((place+(i-1)*(self.state.width as usize)) as usize);
                if a == Some(0){
                  self.state.set(place+(i-1)*(self.state.width as usize) as usize,team);
                  return Ok(())
                }
                
            }
            
        }
    }
    return Err("No spots left");
  }
}