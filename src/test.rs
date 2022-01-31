
#[cfg(test)]
mod tests {
    use std::{mem::size_of_val, time::Instant};

    use crate::Board;
    #[bench]
    fn benchmark(){
      let percentage = 0.25;
      let mut board = Board::new();
      for y in 0..board.state.height{
        if y >= (board.state.height as f32 * percentage) as u8{
          break;
        }
        let mut team;
        if y % 3 == 0{
          team = true
        }else{
          team = false
        }
        for x in 0..board.state.width{
          if team{
            board.place(x as usize, 1).unwrap();
          }else{
            board.place(x as usize, 2).unwrap();
          }
          board.check_win();
          team = !team;
          
        }
      }

  
  }
    #[test]
    fn test() {
      let mut board = Board::new();
      let mut set_total = 0;
      let mut get_total = 0;
      for i in 0..100{
        let time = Instant::now();
        let elapsed = time.elapsed().as_nanos();
        get_total += elapsed;
        let time = Instant::now();
        board.state.set(23,3);
        let elapsed = time.elapsed().as_nanos();
        set_total += elapsed;
      }
      println!("Get took {} nanoseconds",get_total/100);
      println!("Set took {} nanoseconds",set_total/100);

      
      // println!("u8 Board is {} bytes",size_of_val(&old_method));
      // println!("new Board is {} bytes",size_of_val(&board.state));
      // println!("{}",&board.state.state);
      // board.display_board();

    }
}
