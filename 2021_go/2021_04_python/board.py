class Board:
    def __init__(self, board) -> None:
        self.board = board
        self.marked = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ]
    
    def __str__(self) -> str:
        res = ""
        for row in self.board:
            for num in row:
                res += "%2d" % num + " "
            res += '\n'
        return res

    def from_str_board(str_board: str):
        res = []
        for line in str_board.splitlines():
            row = [int(x) for x in line.split(" ") if x != '']
            res.append(row)   
        return Board(res)

    def mark_number(self, num: int):
        for row in range(5):
            for col in range(5):
                if self.board[row][col] == num:
                    self.marked[row][col] = 1
    
    def unmarked_score(self) -> int:
        score = 0
        for row in range(5):
            for col in range(5):
                if not self.marked[row][col]: score += self.board[row][col]
        return score

    def is_winner(self) -> (bool, int):
        for row in range(5):
            row_sum = sum(self.marked[row])
            if row_sum == 5:
                return (True, self.unmarked_score())

        for col in range(5):
            col_sum = 0
            for row in range(5):
                col_sum += self.marked[row][col]
            if col_sum == 5:
                return (True, self.unmarked_score())

        return (False, 0)