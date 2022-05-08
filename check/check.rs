const field_width: usize = 10;

struct Game {
    field: [[u32; field_width]; 5]
}

fn check(g: Game, y: usize) {
    for x in 1..field_width {
	    if g.field[y][x] == 0 {
		    return
        }
    }
}