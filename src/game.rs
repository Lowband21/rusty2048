pub mod oxydized2048 {
    use rand::prelude::*;
    use serde::{Serialize, Deserialize};

    #[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
    pub struct Game {
        pub board: [[u32; 4]; 4],
        pub prev_board: [[u32; 4]; 4],
        pub score: u32,
        pub prev_score: u32,
        pub merged_last: u32,
        pub last_action: Action,
    }
    
    #[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
    pub enum Action {
        MergeLeft,
        MergeRight,
        MergeUp,
        MergeDown,
    }

    #[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
    pub enum GameState {
        Gameover,
        InvalidMove,
        Ok,
    }
    
    impl Game {
        pub fn new() -> Game {
            let mut game = Game {
                board: [[0; 4]; 4],
                prev_board: [[0; 4]; 4],
                score: 0,
                prev_score: 0,
                merged_last: 0,
                last_action: Action::MergeDown,
            };
            game.place_next();
            game
        }

        pub fn undo(&mut self) {
            self.board = self.prev_board;
        }

        pub fn action(&mut self, action: &Action) -> GameState {
            let original = self.board;
            self.prev_score = self.calc_score();
            self.last_action = action.clone();
            match action {
                Action::MergeLeft => self.merge_left(),
                Action::MergeRight => self.merge_right(),
                Action::MergeUp => self.merge_up(),
                Action::MergeDown => self.merge_down(),
            }
            if self.board == original {
                if self.is_gameover() {
                    self.reset();
                    GameState::Gameover
                } else {
                    GameState::InvalidMove
                }
            } else {
                self.place_next();
                self.score = self.calc_score();
                if self.is_gameover() {
                    self.reset();
                    GameState::Gameover
                } else {
                    GameState::Ok
                }
            }
        }
        
        pub fn reset(&mut self){
            *self = Game::new();
        }
        
        fn place_next(&mut self){
            let mut zeros = 0;
            let mut picked = false;
            let mut rng = thread_rng();
    
            for row in self.board.iter(){
                for col in row.iter(){
                    if *col == 0_u32 {
                        zeros += 1;
                        break;
                    }
                }
                if zeros > 0 {
                    break;
                }
            }
    
            while !picked && zeros > 0{
                for row in self.board.iter_mut() {
                    for col in row.iter_mut() {
                        if rng.gen_range(0..16) == 0 && *col == 0_u32 && !picked{
                            if rng.gen_range(0..10) == 0 {
                                *col += 4;
                            }
                            else {
                                *col += 2;
                            }
                            picked = true;
                        }
                    }
                }
            }
        }
    
        fn merge_left(&mut self){
            self.merged_last = 0;
            for row in self.board.iter_mut(){
                let mut non_zeros = 0;
                for col in row.iter(){
                    if *col != 0{
                        non_zeros += 1;
                    }
                }
        
                if non_zeros > 0 && non_zeros < row.len()-1{
                    for j in 0..non_zeros{
                        let mut k = 1;
                        while row[j] == 0 && j+k < row.len(){
                            if row[j+k] > 0{
                                row[j] = row[j+k];
                                row[j+k] = 0;
                            }
                            k += 1
                    }
                }
        
                }
        
                for j in 0..row.len()-1{
                    let (cur, next) = (row[j], row[j+1]);
                    if cur == next && cur != 0 && next != 0{
                        row[j] = cur + next;
                        self.merged_last += cur*2;
                        row[j+1] = 0;
                    }
                    else if cur == 0 && next != 0{
                        row[j] = next;
                        row[j+1] = 0;
        
                    }
                }
            }
        }
        
        fn merge_right(&mut self){
            self.reverse();
            self.merge_left();
            self.reverse();
        }
        fn merge_up(&mut self){
            self.transpose();
            self.merge_left();
            self.transpose();
        
        }
        fn merge_down(&mut self){
            self.transpose();
            self.merge_right();
            self.transpose();
        }
        
        fn reverse(&mut self){
            for row in self.board.iter_mut(){
                row.reverse();
            }
        }
        
        fn transpose(&mut self){
            let board = self.board;
            let mut new_board = [[0; 4]; 4];
            // outer for loop to traverse rows
        	for i in 0..new_board.len()
            {
                // inner for loop to traverse column
                for j in 0..new_board[0].len()
                {
                    // insert arr[row][col] to transpose[col][row]
                    new_board[j][i] = board[i][j];
                }
            }
            self.board = new_board;
        }
        
        pub fn display(&self){
            let board = self.board;
            let mut max: u32 = 0;
            for row in board.iter(){
                for col in row.iter(){
                    if *col > max{
                        max = *col;
                    }
        
                }
            }
            let board = board;
            for row in board.iter(){
                for col in row.iter(){
                    if *col == 0_u32{
                        print!("|{}", " ".repeat(max.to_string().len()));
        
                    }else{
                        print!("|{}{}", *col, " ".repeat(max.to_string().len() - col.to_string().len()));
        
                    }
                }
                println!("|");
            }
            println!();
        }
        
        pub fn calc_score(&self) -> u32 { 
            let mut score: u32 = 0;
            let mut max: u32 = 0;
            let mut maxes: Vec<(usize, usize)> = Vec::new();
            let mut second_maxes: Vec<(usize, usize)> = Vec::new();
            for (row_n, row) in self.board.iter().enumerate(){
                for (col_n, col) in row.iter().enumerate(){
                    if *col > max {
                        maxes.push((row_n, col_n));
                        
                        max = *col;
                    }
                    score += fast_math::log2(*col as f32) as u32;
                    
                }
            }
            for (row_n, row) in self.board.iter().enumerate(){
                for (col_n, col) in row.iter().enumerate(){
                    if *col == max/2{
                        second_maxes.push((row_n, col_n));
                    }
                }
            }

            let mut applied = false;
            let mut x_prev: usize = 0;
            let mut y_prev: usize = 0;
            for (i, (x, y)) in maxes.iter().enumerate(){
                if i > 0 {
                    if x > &0 {
                        if x - 1 == x_prev || x + 1 == x_prev && *y == y_prev {
                            score += max*10;

                        }
                    }
                    if y > &0 {
                        if y - 1 == y_prev || y + 1 == y_prev && *x == x_prev {
                            score += max*10;
                        }
                    }
                }


                if *x == 3 && !applied{
                    //max *= 10;
                    if *y == 3 && !applied {
                        //println!("max: {}", max);
                        if max >= 128 {
                            max *= 10;
                            applied = true;
                        
                        }
                    }
                }
                x_prev = *x;
                y_prev = *y;
            }

            if maxes.len() == 1 && second_maxes.len() > 1{
                for (i, (x, y)) in second_maxes.iter().enumerate(){
                    if i > 0 {
                        if x > &0 {
                            if x - 1 == x_prev || x + 1 == x_prev && *y == y_prev {
                                score += (max/2)*10;

                            }
                        }
                        if y > &0 {
                            if y - 1 == y_prev || y + 1 == y_prev && *x == x_prev {
                                score += (max/2)*10;
                            }
                        }
                    }
                    x_prev = *x;
                    y_prev = *y;
                }

            }
            /* Additional feature I didn't end up using
            let row4 = self.board[3].iter().sum::<u32>();
            let row3 = self.board[2].iter().sum::<u32>();
            let row2 = self.board[1].iter().sum::<u32>();
            let row1 = self.board[0].iter().sum::<u32>();
            if row4 > row3+row2+row1 {
                score += row4*10;
                if row3 > row2+row1 {
                    score += row3*10;
                    if row2 > row1 {
                        score += row2*10;
                    }
                }
            }*/
            if self.last_action == Action::MergeDown || self.last_action == Action::MergeRight {
                (max*10) + score*2 + self.merged_last
            } else {
                (max*10) + score + self.merged_last
            }
        }
        
        pub fn get_score(&self) -> u32 {
            self.score
        }
        
        pub fn remaining_empty(&self) -> u8 {
            let mut zeros = 0;
            for row in self.board.iter(){
                for col in row.iter(){
                    if *col == 0_u32 {
                        zeros += 1;
                    }
        
                }
            }
            zeros
        }
        
        pub fn is_gameover(&self) -> bool{
            let mut new = self.clone();
            for row in self.board.iter(){
                for col in row.iter(){
                    if *col == 0_u32 {
                        return false;
                    }
        
                }
            }
            new.merge_up();
            if self.board != new.board{
                return false;
            }
        
            new.board = self.board;
            new.merge_down();
            if self.board != new.board{
                return false;
            }
        
            new.board = self.board;
            new.merge_right();
            if self.board != new.board{
                return false;
            }
        
            new.board = self.board;
            new.merge_left();
            if self.board != new.board{
                return false;
            }
            true 
        }
        
        pub fn get_valid_actions(&self) -> Vec<Action> {
            let mut new = self.clone();
            let mut valid: Vec<Action> = Vec::new();
            new.merge_up();
            if self.board != new.board{
                valid.push(Action::MergeUp);
            }
        
            new.board = self.board;
            new.merge_down();
            if self.board != new.board{
                valid.push(Action::MergeDown);
            }
        
            new.board = self.board;
            new.merge_right();
            if self.board != new.board{
                valid.push(Action::MergeRight);
            }
        
            new.board = self.board;
            new.merge_left();
            if self.board != new.board{
                valid.push(Action::MergeLeft);
            }
            valid

        }
    
    }
}