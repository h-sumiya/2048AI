from dataclasses import dataclass
from .py2048 import *


@dataclass
class Board:
    _data: bytes
    _value: bytes | None = None

    def from_data(data: bytes, seed: int) -> 'Board':
        return Board(_from_data(data, seed))

    def from_seed(seed: int) -> 'Board':
        return Board(_init_board(seed))

    def moves(self) -> 'Moves':
        moves = _moves(self._data)
        return Moves(
            Board(moves[0]),
            Board(moves[1]),
            Board(moves[2]),
            Board(moves[3]),
        )
    
    @property
    def seed(self) -> int:
        return _seed(self._data)

    def load_value(self) -> None:
        if self._value is None:
            self._value = _to_data(self._data)

    def __str__(self) -> str:
        return _display(self._data)

    def __getitem__(self, index: int) -> int:
        self.load_value()
        return self._value[index]

    def __int__(self) -> int:
        return self.seed()

    def __bytes__(self) -> bytes:
        return self._data

    def __eq__(self, other: 'Board') -> bool:
        return self._data[:8] == other._data[:8]

    def __ne__(self, other: 'Board') -> bool:
        return self._data[:8] != other._data[:8]

    class ValueIter:
        def __init__(self, board: 'Board'):
            self._board = board
            self._index = 0
            board.load_value()

        def __next__(self) -> int:
            if self._index >= 16:
                raise StopIteration
            value = self._board._value[self._index]
            self._index += 1
            return value

    def __iter__(self) -> 'ValueIter':
        return Board.ValueIter(self)


@dataclass
class Moves:
    up: Board
    down: Board
    left: Board
    right: Board

    def __str__(self) -> str:
        return f"UP: {self.up}\n" \
            f"DOWN: {self.down}\n" \
            f"LEFT: {self.left}\n" \
            f"RIGHT: {self.right}\n"
