
// Basic neural network: Take in value X to guess value Y: If X is somewhat cloudy then it's somewhat likely to rain
// X is a value from 0 (Very cloudy) to 1 (Very sunny) and Y is a statistical classification that guesses the probability
// of it being a certain wheather. It takes the most likely one.
// Input(0-256) -> Output("There's a 71% chance of it being very cloudy")

use rand::prelude::*;

#[derive(Debug)]
struct WeatherTable {
    day: u8,
    info: String,
}

const MARGIN: usize = 7;

// Might want to return reference and give read-only soon
fn generate_data() -> std::vec::Vec<[WeatherTable; 1]> {

    let mut vec_struct = Vec::new();
    
    // Special case scenario if randomized numbers are above guess
    let w_t = [
        WeatherTable {
            day: 0 as u8,
            info: "Very Windy!".to_string(),
        }
    ];
    vec_struct.push(w_t);


    for i in 0..50 {
        let mut rng = rand::thread_rng();
        let y: u8 = rng.gen();
        if y == 0 {
            continue;
        }

        let s = match y {
            1 ..= 5 => "Very windy!",
            6 ..= 20 => "Pretty windy",
            21 ..= 60 => "Casually windy",
            61 ..= 120 => "Cloudy",
            121 ..= 150 => "Half Sunny Half Cloudy!",
            151 ..= 190  => "Decently sunny",
            _ => "Sweet and sunny!",
        };
        
        let w_t = [
            WeatherTable {
                day: y,
                info: s.to_string(),
            }
        ];
        vec_struct.push(w_t);
    }
    vec_struct
}

// Let X be the day, and Y be hotness. As X increases, so does the weather comments in hotness
// Instructions: Take in Z number of unordered elements of X and Y
// And predict Y with respect to X by returning a comment as closely in relevance to 
// Also print the statistical outcome of all weathers: Eg: "0.1% of very windy", "3% of very sunny"...

// Concrete implementation: Insert table[1, "Very Windy"], 

// Algorithmic implementation: Count the steps in sortered table from 

fn predict_outcome(data: std::vec::Vec<[WeatherTable; 1]>, uguess: u8) -> String{
    let mut default_ans = String::from("Default");
    let mut vec = Vec::new();
    let mut clamp_lower = 0;
    let mut clamp_higher = 0;
    
    
    for i in data {
        vec.push(i[0].day);
    }

    vec.sort();

    for i in 1..vec.len() {
        if uguess > vec[i] {
            for j in 1..vec.len(){
                if uguess < vec[j] {
                    // Not the perfect solution because this nested
                    // loop is ran more than once but does the job
                    clamp_lower = vec[i];
                    clamp_higher = vec[j];
                    break;
                }
            }
        }
    }

    for i in 0..20 {
        println!("first twenty: {}", vec[i]);
    }
    
    println!("1: lower: {} higher: {} guess: {}", clamp_lower, clamp_higher, uguess);

    // TODO: Calculate percentage of Y outcome being the case, not just say "more likely"
    if (uguess - clamp_lower) > (clamp_higher - uguess) {
        // Steps from lower to guess > higher to guess
        println!("Guess ({}) is more likely to be with lower bound ({})", uguess, clamp_lower);
    }
    
    if (uguess - clamp_lower) < (clamp_higher - uguess) {
        // Steps from lower to guess > higher to guess
        println!("Guess ({}) is more likely to be with higher bound ({})", uguess, clamp_higher);
    }

    if (uguess - clamp_lower) == (clamp_higher - uguess) {
        // Same number of steps from both clamps
        println!("Same likelyhood");
    }

    default_ans
}


fn main() {
    let s = generate_data();
    predict_outcome(s, 69); // If the guess (currently 33) is too low then it's more likely to not run
}
