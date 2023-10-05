from py2048 import Board

board = Board.from_seed(290797)
print(board)
moves = board.moves()
print(moves.down)
print(moves.down.seed())

data = bytes(board)
board = Board(data)
print(board)

for value in board:
    print(value)

print(moves)

up = moves.left
while True:
    print(up)
    moves = up.moves()
    if up == moves.up:
        break
    up = moves.up
data = list(up)
print(data)