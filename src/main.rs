use std::{io::{stdin, stdout, Read, Write}, mem::{size_of, size_of_val}, time::Instant};
use board::Board;
mod board;
mod test;
const WIDTH: u8 = 7;
const HEIGHT: u8 = 6;
fn pause() {
  let mut stdout = stdout();
  stdout.write(b"Press Enter to continue...").unwrap();
  stdout.flush().unwrap();
  stdin().read(&mut [0]).unwrap();
}
fn get_input() -> Result<usize,String>{
  let mut line = String::new();
  println!("Enter your move :");
  std::io::stdin().read_line(&mut line).unwrap();
  let my_int = line.trim_end().parse::<i32>();
  if my_int.is_err() {
    return Err("Incorrect input".to_string())
  }
  
  return Ok(my_int.unwrap() as usize)
  
}

fn main() {

  



    let mut board = Board::new();
    let running = true;
    while running{
      board.display_board();
      let action:usize;
      match get_input() {
        Ok(output) => action = output,
        Err(err) => {println!("{}",err); continue},
      }
      match board.place(action, board.team){
        Ok(()) => {
          println!("Placed in {} coloumn", action);
          match board.check_win() {
            Some(u8) => {board.display_board();println!("player {} won!!!",u8);board = Board::new();pause();},
            None => {board.team = if board.team == 1 {2} else{1};}
          }
          
        }
        Err(err) => {
          println!("{}", err);
        },
        
      }
    }


}
