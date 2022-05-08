let field_height = 20
let field_width = 10

function init(g) {
    for (var i = 0; i <= field_height + 2; i++ ) {
        let row = Array(field_width + 2).fill(0);
        row[0] = -1;
        row[field_width + 1] = -1
        g.field.push(row);
    }
    for (var j = 0; j <= field_width + 2; j++ ) {
        g.field[0][j] = -1
        g.field[field_height + 1][j] = -1
    }
}
