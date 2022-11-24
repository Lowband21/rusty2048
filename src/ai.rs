pub mod ai {
    use std::fs;
    use std::io::Write;

    use crate::game::oxydized2048::{Action, Game, GameState};
    use rurel::mdp::{Agent, State};
    use rurel::strategy::explore::{RandomExploration};
    use rurel::strategy::learn::QLearning;
    use rurel::strategy::terminate::{FixedIterations, TerminationStrategy};
    use rurel::AgentTrainer;
    use std::time::{Instant};

    impl State for Game {
        type A = Action;
        fn reward(&self) -> f64 {
            let score = self.calc_score();
            if self.is_gameover() {
                println!("Gameover");
                -(score as f64)
            } else {
                Into::<f64>::into(score) - Into::<f64>::into(self.prev_score)
            }
        }
        fn actions(&self) -> Vec<Action> {
            let actions = self.get_valid_actions();
            if actions.contains(&Action::MergeDown) && actions.contains(&Action::MergeRight) {
                return vec![Action::MergeDown, Action::MergeRight];
            } else if actions.contains(&Action::MergeDown) {
                return vec![Action::MergeDown];
            } else if actions.contains(&Action::MergeRight) {
                return vec![Action::MergeRight];
            } else {
                return actions;
            }
        }
    }  

    struct MyAgent {
        state: Game,
    }
    impl Agent<Game> for MyAgent {
        fn current_state(&self) -> &Game {
            &self.state
        }
        fn take_action(&mut self, action: &Action) -> () {
            self.state.action(action);
        }
    }


    pub fn train(
        trainer: &mut AgentTrainer<Game>,
        alpha: f64,
        gamma: f64,
        initial_value: f64,
        num_iter: u32,
        num_runs: u32,
    ) -> &mut AgentTrainer<Game> {
        let learning_strat = &QLearning::new(alpha, gamma, initial_value);
        let exploration_strat = &RandomExploration::new();

        let start = Instant::now();
        let mut sum = 0;
        for i in 0..num_runs {
            let now = Instant::now();

            println!("Run: {}/{}", i, num_runs);
            let mut agent = MyAgent { state: Game::new() };
            trainer.train(
                &mut agent,
                learning_strat,
                &mut FixedIterations::new(num_iter),
                exploration_strat,
            );

            sum += (Instant::now() - now).as_millis();
            let average_runtime = (sum as f64) / ((i+1) as f64);
            let estimated_remaining = (average_runtime/1000.0) * ((num_runs - (i+1)) as f64);
            let total = (Instant::now() - start).as_secs();
            print!("Elapsed: {}:{} | ", total/60, total%60);
            println!("Remaining: {}:{}", (estimated_remaining/60.0) as i64, (estimated_remaining%60.0) as i64);
        }
        trainer
    }

    pub fn test_and_train(
        trainer: &mut AgentTrainer<Game>,
        alpha: f64,
        gamma: f64,
        initial_value: f64,
        num_iter: u32,
        num_games: u32,
    ) -> (&mut AgentTrainer<Game>, u32) {
        let mut test_game = Game::new();
        let mut high_score = 0;
        let learning_strat = &QLearning::new(alpha, gamma, initial_value);
        let exploration_strat = &RandomExploration::new();

        let mut games_played = 0;
        let mut steps = 0;
        loop {
            steps += 1;
            if let Some(action) = trainer.best_action(&test_game) {
                //println!("Got action from trainer");
                let action_result = test_game.action(&action);
                match action_result {
                    GameState::Gameover => {
                        println!("Gameover");
                        games_played += 1;

                        let mut agent = MyAgent { state: Game::new() };
                        trainer.train(
                            &mut agent,
                            learning_strat,
                            &mut FixedIterations::new(num_iter),
                            exploration_strat,
                        );

                        if games_played > num_games {
                            return (trainer, high_score);
                        }
                    }
                    GameState::InvalidMove => test_game.reset(),
                    GameState::Ok => {
                        let score = test_game.calc_score();
                        if score >= high_score {
                            high_score = score;
                            println!("New high score: {}", score);
                            test_game.display();
                        } else if steps % 10 == 0 {
                            println!("Score: {}", score);
                            test_game.display();
                        }
                    }
                }
            } else {
                let mut agent = MyAgent {
                    state: test_game.clone(),
                };
                trainer.train(
                    &mut agent,
                    learning_strat,
                    &mut NumGames::new(num_iter),
                    exploration_strat,
                );
            }
        }
    }

    pub fn test(trainer: &AgentTrainer<Game>) -> u32 {
        let mut high_score = 0;
        let mut test_game = Game::new();
        let mut steps = 0;
        loop {
            steps += 1;
            if let Some(action) = trainer.best_action(&test_game) {
                let action_result = test_game.action(&action);
                match action_result {
                    GameState::Gameover => {
                        println!("Gameover");
                        return high_score;
                    }
                    GameState::InvalidMove => {
                        println!("Invalid move");
                        return high_score;
                    }
                    GameState::Ok => {
                        let score = test_game.calc_score();
                        if score >= high_score {
                            high_score = score;
                            println!("New high score: {}", score);
                            test_game.display();
                        } else if steps % 10 == 0 {
                            println!("Score: {}", score);
                            test_game.display();
                        }
                    }
                }
            } else {
                return high_score;
            }
        }
    }

    fn _log(message: String) {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("Results".to_owned() + ".log")
            .unwrap();
        file.write(message.as_bytes()).unwrap();
        println!("{:?}", message);
    }

    struct NumGames {
        curr_game: u32,
        target_games: u32,
    }

    impl NumGames {
        pub fn new(target_games: u32) -> NumGames {
            return NumGames {
                curr_game: 0,
                target_games: target_games,
            };
        }
    }

    impl<S: State> TerminationStrategy<S> for NumGames {
        fn should_stop(&mut self, state: &S) -> bool {
            if state.reward() < 0.0 {
                self.curr_game += 1;
            }
            return self.curr_game == self.target_games;
        }
    }
}
