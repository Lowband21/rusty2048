pub mod game;
pub mod ai;

use crate::game::oxydized2048::{Game, GameState, Action};
use crate::ai::ai::*;

use std::collections::HashMap;
use std::process;

use requestty::{Question};
use rurel::AgentTrainer;

fn main() {
    let human_or_machine  = Question::select("Human or machine")
        .message("What do you want to do?")
        .choice("Play")
        .choice("Train")
        .choice("Train and Test")
        .choice("Test")
        .build();
    
    let binding = &requestty::prompt_one(human_or_machine);
    let answer = answer_or_exit(binding).as_list_item().unwrap().text.as_str();
    
    match answer {
        "Play" => {
            play();
        },
        "Train" => {
            /*println!("Loading learned state from file...");
            let learned_state: HashMap<Game, HashMap<Action, f64>> = serde_any::from_file("learned_state.ron").unwrap();
            let mut trainer: AgentTrainer<Game> = AgentTrainer::new();

            println!("Importing state...");
            trainer.import_state(learned_state);*/

            let mut trainer = AgentTrainer::new();
            let trainer = train(&mut trainer, 0.2, 0.6, 0.5, 10000, 1000);
            let learned_state = trainer.export_learned_values();

            println!("Saving learned state to file...");
            serde_any::to_file("learned_state.ron", &learned_state).unwrap();
        },
        "Train and Test" => {
            println!("Loading learned state from file...");
            let learned_state: HashMap<Game, HashMap<Action, f64>> = serde_any::from_file("learned_state.ron").unwrap();
            let mut trainer: AgentTrainer<Game> = AgentTrainer::new();

            println!("Importing state...");
            trainer.import_state(learned_state);

            let (trainer, high_score) = test_and_train(&mut trainer, 0.2, 0.6, 0.5, 10000, 1);
            println!("High score: {}", high_score);

            let learned_state = trainer.export_learned_values();

            println!("Saving learned state to file...");
            serde_any::to_file("learned_state.ron", &learned_state).unwrap();
        },
        "Test" => {
            println!("Loading learned state from file...");
            let learned_state: HashMap<Game, HashMap<Action, f64>> = serde_any::from_file("learned_state.ron").unwrap();
            let mut trainer: AgentTrainer<Game> = AgentTrainer::new();

            println!("Importing state...");
            trainer.import_state(learned_state);
            for _ in 0..1000 {
                let high_score = test(&trainer);
                println!("High score: {}", high_score);
            }
        },
        _ => process::exit(1),
    }
    
    
}

fn play(){
    let mut game = Game::new();
    let mut state = GameState::Ok;
    

    while state != GameState::Gameover {
        loop {
            game.display();

            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line).unwrap();
            //println!("{}", line.len());

            state = char_to_action(&line, &mut game);

            if state != GameState::InvalidMove {
                break;
            }
            println!("Invalid move, please try again.");
        }

        println!("Score: {:?}", game.get_score());
    }
}
fn answer_or_exit(
    binding: &std::result::Result<requestty::Answer, requestty::ErrorKind>,
) -> &requestty::Answer {
    let answer = match binding {
        Ok(x) => x,
        Err(e) => match e {
            requestty::ErrorKind::IoError(_) => panic!("IO Error"),
            requestty::ErrorKind::Interrupted => process::exit(1),
            requestty::ErrorKind::Eof => panic!("IO Error"),
            requestty::ErrorKind::Aborted => process::exit(1),
        },
    };
    answer
}

fn char_to_action(char: &String, game: &mut Game) -> GameState {
    match char.trim() {
        "w" => game.action(&Action::MergeUp),
        "a" => game.action(&Action::MergeLeft),
        "s" => game.action(&Action::MergeDown),
        "d" => game.action(&Action::MergeRight),
        _ => GameState::InvalidMove
    }
}
