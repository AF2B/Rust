fn main() {
	let mut board = [[0; 3]; 3];
	let mut turn = 1;
	let mut game_over = false;
	while !game_over {
		println!("Turn {}", turn);
		for row in board.iter() {
			for cell in row.iter() {
				print!("{} ", cell);
			}
			println!("");
		}
		println!("Enter a row and column:");
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		let mut input = input.trim().split_whitespace();
		let row: usize = input.next().unwrap().parse().unwrap();
		let col: usize = input.next().unwrap().parse().unwrap();
		board[row][col] = turn;
		turn = if turn == 1 { 2 } else { 1 };
		game_over = true;
	}
}
