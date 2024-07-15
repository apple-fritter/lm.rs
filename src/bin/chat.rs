use llmrs::transformer::Transformer;
use llmrs::tokenizer::Tokenizer;
use llmrs::functional::sample;

use std::env;
use std::io;
use std::io::Write;
use std::time::Instant;
use std::fs::File;
use memmap2::Mmap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let model_path: &str = &args[1];

    let mut tokenizer = Tokenizer::new("tokenizer.bin");
        
    let file = File::open(model_path).expect("Model file required");
    let data = unsafe { Mmap::map(&file).expect("MMap failed")  };

    let mut model = Transformer::new(&data);

    let mut user_turn = true;
    let mut user_idx: usize = 0;
    let mut pos = 0;
    let mut token: u32;
    let mut next: u32 = 0;
    let mut num_prompt_tokens = 0;

    let mut prompt_tokens: Vec<u32> = Vec::new();

    let mut user_prompt = String::new();

    loop {
        if user_turn {
            user_prompt = String::from("");

            print!("Please enter some text: ");
            io::stdout().flush().unwrap();

            io::stdin().read_line(&mut user_prompt).expect("Failed to read line");

            prompt_tokens = tokenizer.encode(user_prompt.trim(), false, false, false);
            num_prompt_tokens = prompt_tokens.len();

            user_turn = false; 
            user_idx = 0;
            
            print!("Gemma: ");
            io::stdout().flush().unwrap();
        }

        if user_idx < num_prompt_tokens {
            token = prompt_tokens[user_idx];
            user_idx += 1;
        } else {
            token = next;
        }

        if token == 1 { user_turn = true; }

        let logits = model.forward(token, pos);
        next = sample(logits);
        pos += 1;

        if user_idx >= num_prompt_tokens && next != 1 {
            let piece = tokenizer.decode(token);
            print!("{}", piece);
            io::stdout().flush().unwrap();
        }   

        if next == 1 { println!(""); }
    }
}