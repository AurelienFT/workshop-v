const field_width: usize = 10;
const field_height: usize = 20;

struct Game {
    field: [[i32; field_width + 2]; field_height + 2]
}

fn init(g: Game) {
    for i in 0..field_height + 2 {
        let mut row = [0; field_width + 2];
        row[0] = -1;
        row[field_width + 1] = -1;
        g.field[i] = row;
    }
    for j in 0..field_width + 2 {
        g.field[0][j] = -1;
        g.field[field_height + 1][j] = -1;
    }
}