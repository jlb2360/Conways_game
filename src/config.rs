



pub fn config() -> &'static str{


    let picking = true;
    println!("Welcome to the game of life!");


    while picking {

        println!("Please select a preset group of cells to start the game:");
        println!("1. Blinker");
        println!("2. Toad");
        println!("3. Exit");



        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();


        match input.trim() {
            "1" => {
                return "blinker";
            },
            "2" => {
                return "toad";
            },
            "3" => {
                println!("Goodbye!");
                std::process::exit(0);
            },
            _ => {
                println!("Invalid selection. Please try again.");
            },
        }
    }

    return "blinker";


}
