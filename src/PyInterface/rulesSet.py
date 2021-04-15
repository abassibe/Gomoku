import numpy as np
import copy as cp


class Rules:
    def __init__(self, options):
        self.options = options
        self.activeRules = options.rulesSet
        self.isWinner = 0
        self.winStart = None
        self.winEnd = None

    def checkBasicRule(self, board, x, y):
        if board[x][y] != 0:
            return False
        return True

    def getBasicRule(self, board):
        return [tuple(coord) for coord in np.argwhere(np.array(board) == 0).tolist()]


    def captureRule(self, board, x, y, color):
        target = 1 if color == 2 else 2
        removed_stones = []
        if y > 2 and board[x][y - 3] == color and (board[x][y - 2] == target and board[x][y - 1] == target):
            removed_stones.append((x, y - 2))
            removed_stones.append((x, y - 1))
        if x > 2 and y > 2 and board[x - 3][y - 3] == color and (
                board[x - 2][y - 2] == target and board[x - 1][y - 1] == target):
            removed_stones.append((x - 2, y - 2))
            removed_stones.append((x - 1, y - 1))
        if x > 2 and board[x - 3][y] == color and (board[x - 2][y] == target and board[x - 1][y] == target):
            removed_stones.append((x - 2, y))
            removed_stones.append((x - 1, y))
        if x > 2 and y < 16 and board[x - 3][y + 3] == color and (
                board[x - 2][y + 2] == target and board[x - 1][y + 1] == target):
            removed_stones.append((x - 2, y + 2))
            removed_stones.append((x - 1, y + 1))
        if y < 16 and board[x][y + 3] == color and (board[x][y + 2] == target and board[x][y + 1] == target):
            removed_stones.append((x, y + 2))
            removed_stones.append((x, y + 1))
        if x < 16 and y < 16 and board[x + 3][y + 3] == color and (
                board[x + 2][y + 2] == target and board[x + 1][y + 1] == target):
            removed_stones.append((x + 2, y + 2))
            removed_stones.append((x + 1, y + 1))
        if x < 16 and board[x + 3][y] == color and (board[x + 2][y] == target and board[x + 1][y] == target):
            removed_stones.append((x + 2, y))
            removed_stones.append((x + 1, y))
        if x < 16 and y > 2 and board[x + 3][y - 3] == color and (
                board[x + 2][y - 2] == target and board[x + 1][y - 1] == target):
            removed_stones.append((x + 2, y - 2))
            removed_stones.append((x + 1, y - 1))
        return removed_stones

    def hasSameColorNeighbor(self, board, x, y, color, result):
        opponent = 1 if color == 2 else 2
        if (x > 0 and y > 0 and board[x - 1][y - 1] == opponent) and (
                x < 17 and y < 17 and board[x + 1][y + 1] == color and board[x + 2][y + 2] == 0):
            result.append((x + 2, y + 2))
        elif (x > 0 and y > 0 and board[x - 1][y - 1] == 0) and (
                x < 17 and y < 17 and board[x + 1][y + 1] == color and board[x + 2][y + 2] == opponent):
            result.append((x - 1, y - 1))
        if (x > 0 and board[x - 1][y] == opponent) and (x < 17 and board[x + 1][y] == color and board[x + 2][y] == 0):
            result.append((x + 2, y))
        elif (x > 0 and board[x - 1][y] == 0) and (x < 17 and board[x + 1][y] == color and board[x + 2][y] == opponent):
            result.append((x - 1, y))
        if (x > 0 and y < 18 and board[x - 1][y + 1] == opponent) and (
                x < 17 and y > 1 and board[x + 1][y - 1] == color and board[x + 2][y - 2] == 0):
            result.append((x + 2, y - 2))
        elif (x > 0 and y < 18 and board[x - 1][y + 1] == 0) and (
                x < 17 and y > 1 and board[x + 1][y - 1] == color and board[x + 2][y - 2] == opponent):
            result.append((x - 1, y + 1))
        if (y < 18 and board[x][y + 1] == opponent) and (y > 1 and board[x][y - 1] == color and board[x][y - 2] == 0):
            result.append((x, y - 2))
        elif (y < 18 and board[x][y + 1] == 0) and (y > 1 and board[x][y - 1] == color and board[x][y - 2] == opponent):
            result.append((x, y + 1))
        if (x < 18 and y < 18 and board[x + 1][y + 1] == opponent) and (
                x > 1 and y > 1 and board[x - 1][y - 1] == color and board[x - 2][y - 2] == 0):
            result.append((x - 2, y - 2))
        elif (x < 18 and y < 18 and board[x + 1][y + 1] == 0) and (
                x > 1 and y > 1 and board[x - 1][y - 1] == color and board[x - 2][y - 2] == opponent):
            result.append((x + 1, y + 1))
        if (x < 18 and board[x + 1][y] == opponent) and (x > 1 and board[x - 1][y] == color and board[x - 2][y] == 0):
            result.append((x - 2, y))
        elif (x < 18 and board[x + 1][y] == 0) and (x > 1 and board[x - 1][y] == color and board[x - 2][y] == opponent):
            result.append((x + 1, y))
        if (x < 18 and y > 0 and board[x + 1][y - 1] == opponent) and (
                x > 1 and y < 17 and board[x - 1][y + 1] == color and board[x - 2][y + 2] == 0):
            result.append((x - 2, y + 2))
        elif (x < 18 and y > 0 and board[x + 1][y - 1] == 0) and (
                x > 1 and y < 17 and board[x - 1][y + 1] == color and board[x - 2][y + 2] == opponent):
            result.append((x + 1, y - 1))
        if (y > 0 and board[x][y - 1] == opponent) and (y < 17 and board[x][y + 1] == color and board[x][y + 2] == 0):
            result.append((x, y + 2))
        elif (y > 0 and board[x][y - 1] == 0) and (y < 17 and board[x][y + 1] == color and board[x][y + 2] == opponent):
            result.append((x, y - 1))
        return result

    def gameEndingCaptureRule(self, board, being_win_pos, end_win_pos, color):
        self.winStart = being_win_pos
        self.winEnd = end_win_pos
        start_x = being_win_pos[0]
        end_x = end_win_pos[0]
        start_y = being_win_pos[1]
        end_y = end_win_pos[1]

        result = []
        while start_x != end_x or start_y != end_y:
            result = self.hasSameColorNeighbor(board, start_x, start_y, color, result)
            if start_x < end_x:
                start_x += 1
            elif start_x > end_x:
                start_x -= 1
            if start_y < end_y:
                start_y += 1
            elif start_y > end_y:
                start_y -= 1
        result = self.hasSameColorNeighbor(board, start_x, start_y, color, result)
        if result:
            self.isWinner = color
        return result

    def searchThreePoint(self, board, x, y, color):
        pattern = [[0, 1, 1, 1, 0],
                   [0, 1, 1, 0, 1, 0],
                   [0, 1, 0, 1, 1, 0]]
        if color == 2:
            pattern = [[0, 2, 2, 2, 0],
                       [0, 2, 2, 0, 2, 0],
                       [0, 2, 0, 2, 2, 0]]

        h_line = cp.deepcopy(board[x])
        h_line[y] = color
        v_line = cp.deepcopy(board[:, y])
        v_line[x] = color

        d1_line = cp.deepcopy(np.diag(board, y - x))
        tmp = np.full(18 - (d1_line.size - 1), 1, dtype=int)
        if y < x:
            d1_line = np.concatenate((d1_line, tmp), axis=None)
        else:
            d1_line = np.concatenate((tmp, d1_line), axis=None)
        d1_line[y] = color

        d2_line = cp.deepcopy(np.diag(np.fliplr(board), (9 + (9 - y)) - x))
        d2_line = np.flip(d2_line)
        tmp = np.full(18 - (d2_line.size - 1), 1, dtype=int)
        if y >= 19 - x:
            d2_line = np.concatenate((tmp, d2_line), axis=None)
        else:
            d2_line = np.concatenate((d2_line, tmp), axis=None)
        d2_line[y] = color

        number_three = 0
        line_type = ""
        for i in range(y - 4, y + 1):
            if line_type != "hLine" and h_line[i:i + len(pattern[0])].tolist() == pattern[0]:
                number_three += 1
                line_type = "hLine"
            elif line_type != "hLine" and h_line[i:i + len(pattern[1])].tolist() == pattern[1]:
                number_three += 1
                line_type = "hLine"
            elif line_type != "hLine" and h_line[i:i + len(pattern[2])].tolist() == pattern[2]:
                number_three += 1
                line_type = "hLine"
            if line_type != "d2Line" and d2_line[i:i + len(pattern[0])].tolist() == pattern[0]:
                number_three += 1
                line_type = "d2Line"
            elif line_type != "d2Line" and d2_line[i:i + len(pattern[1])].tolist() == pattern[1]:
                number_three += 1
                line_type = "d2Line"
            elif line_type != "d2Line" and d2_line[i:i + len(pattern[2])].tolist() == pattern[2]:
                number_three += 1
                line_type = "d2Line"
            if line_type != "d1Line" and d1_line[i:i + len(pattern[0])].tolist() == pattern[0]:
                number_three += 1
                line_type = "d1Line"
            elif line_type != "d1Line" and d1_line[i:i + len(pattern[1])].tolist() == pattern[1]:
                number_three += 1
                line_type = "d1Line"
            elif line_type != "d1Line" and d1_line[i:i + len(pattern[2])].tolist() == pattern[2]:
                number_three += 1
                line_type = "d1Line"
            if number_three > 1:
                return number_three
        for i in range(x - 4, x + 1):
            if v_line[i:i + len(pattern[0])].tolist() == pattern[0]:
                number_three += 1
                break
            elif v_line[i:i + len(pattern[1])].tolist() == pattern[1]:
                number_three += 1
                break
            elif v_line[i:i + len(pattern[2])].tolist() == pattern[2]:
                number_three += 1
                break
            if number_three > 1:
                return number_three
        if number_three > 1:
            return number_three
        return number_three

    def doubleThreeRule(self, board, x, y, color):
        lst = []
        y_min = y if y < 3 else 3
        y_max = (19 - y) if y > 16 else 4
        for x1 in range(-3, 4):
            if x + x1 < 0 or x + x1 > 18:
                continue
            lst.append(board[x + x1][y - y_min:y + y_max].tolist())

        if self.searchThreePoint(board, x, y, color) > 1:
            return False
        return True

    def getValidPoints(self, board):
        if self.isWinner != 0:
            return self.gameEndingCaptureRule(board, self.winStart, self.winEnd, self.isWinner)
        return self.getBasicRule(board)
