struct CommandLineArguments {
    operation: String,
    id: u32,
    description: String
}

fn main() {
    let operation_arg : String = std::env::args().nth(1).expect("operation required!");
    let id_arg : String = std::env::args().nth(2).expect("id required!");
    let id_parsed : u32 = id_arg.parse::<u32>().unwrap();
    let description_arg : String = std::env::args().nth(3).expect("description required!");

    let cli_args = CommandLineArguments {
        operation: operation_arg,
        id: id_parsed,
        description: description_arg
    };

    println!("args: operation {:?}, id {:?}, description {:?} \n", cli_args.operation, cli_args.id, cli_args.description);

}

