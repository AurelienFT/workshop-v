field_height = 20
field_width = 10

def init(g):
    for _ in range(0, field_height + 2):
        row = [0] * (field_width + 2)
        row[0] = -1
        row[field_width + 1] = -1
        g.field.append(row)
    for j in range (0, field_width + 2):
        g.field[0][j] = -1
        g.field[field_height + 1][j] = -1
