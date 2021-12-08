use std::collections::HashMap;

fn main(){
    // stores moves in key value pairs
    // key is the location and the value is the symbol played
    let mut board = HashMap::from([
       ("1", " "),
       ("2", " "),
       ("3", " "),
       ("4", " "),
       ("5", " "),
       ("6", " "),
       ("7", " "),
       ("8", " "),
       ("9", " "),
       ("0", " ")
    ]);
    // A vector of positions that represent three in a row.
    let win_pos = vec![
        (1,2,3),
        (4,5,6),
        (7,8,9),
        (7,4,1),
        (8,5,2),
        (9,6,3),
        (7,5,3),
        (9,5,1)
    ];
    // Message to start the game
    display_coord();
    println!("The board is aligned with a standard number pad");
    println!("You may quit anytime by entering a non number character");

    let mut game_on = true;
    //turn incrementing variable 
    let mut turn = 0;
    // Main game loop
    while game_on {
        turn += 1;

        println!("choose a position");
        //Receives input from the terminal
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        // If modulus retruns 1, it is player 1's turn and the symbol used is 'x'
        if turn % 2 > 0 {
            match choice.trim() {
                "1" => board.insert("1", "x"),
                "2" => board.insert("2", "x"),
                "3" => board.insert("3", "x"),
                "4" => board.insert("4", "x"),
                "5" => board.insert("5", "x"),
                "6" => board.insert("6", "x"),
                "7" => board.insert("7", "x"),
                "8" => board.insert("8", "x"),
                "9" => board.insert("9", "x"),
                _ => board.insert("0", "end"),
            };
        } else {
            // Player 2's turn
            match choice.trim() {
                "1" => board.insert("1", "o"),
                "2" => board.insert("2", "o"),
                "3" => board.insert("3", "o"),
                "4" => board.insert("4", "o"),
                "5" => board.insert("5", "o"),
                "6" => board.insert("6", "o"),
                "7" => board.insert("7", "o"),
                "8" => board.insert("8", "o"),
                "9" => board.insert("9", "o"),
                _ => board.insert("0", "end"),
            };
        }
        
        display_board(&board);
        if board.get("0").unwrap() == &"end" || turn == 9 {
            game_on = false;
        }
    }
 }
 
 fn display_coord(){
    // This function shows an example board detailing which numbers correspond to which locations     
    println!(" 7 | 8 | 9 ");
    println!("---|---|---");
    println!(" 4 | 5 | 6 ");
    println!("---|---|---");
    println!(" 1 | 2 | 3 ");
 }
 
 fn display_board(board : &HashMap<&str, &str>){
    // Displays the current board to the terminal
    println!(" {} | {} | {} ", board.get("7").unwrap(), board.get("8").unwrap(), board.get("9").unwrap());
    println!("---|---|---");
    println!(" {} | {} | {} ", board.get("4").unwrap(), board.get("5").unwrap(), board.get("6").unwrap());
    println!("---|---|---");
    println!(" {} | {} | {} ", board.get("1").unwrap(), board.get("2").unwrap(), board.get("3").unwrap());
 }
 