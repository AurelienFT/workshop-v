field_width = 10

def check(g, y):
    for x in range(1, field_width):
	    if g.field[y][x] == 0:
		    return
    # Next
