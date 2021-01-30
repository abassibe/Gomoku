import numpy as np
import copy as cp

class Rules():
    def __init__(self, options):
        self.options = options
        self.activeRules = options.rulesSet
        self.isWinner = 0
        self.winStart = None
        self.winEnd = None

    def checkBasicRule(self, board, x, y, color):
        if board[x][y] != 0:
            return False
        return True
        # target = 1 if color == 2 else 2
        # tmpX = -1
        # while tmpX < 2:
        #     if x + tmpX < 0 or x + tmpX > 18:
        #         tmpX += 1
        #         continue
        #     tmpY = -1
        #     while tmpY < 2:
        #         if y + tmpY < 0 or y + tmpY > 18:
        #             tmpY += 1
        #             continue
        #         if board[x + tmpX, y + tmpY] == target:
        #             return True
        #         tmpY += 1
        #     tmpX += 1
        # return False

    def getBasicRule(self, board, color):
        return [tuple(coord) for coord in np.argwhere(np.array(board) == 0).tolist()]
        # target = 1 if color == 2 else 2
        # ret = []
        # tmp = np.where(board == target)
        # if len(tmp[0]) == 0 or len(tmp[1]) == 0:
        #     return
        # for index in range(tmp[0].size):
        #     x = tmp[0][index]
        #     y = tmp[1][index]
        #     if x > 0 and y > 0 and board[x - 1][y - 1] == 0:
        #         ret.append((x - 1, y - 1))
        #     if x > 0 and board[x - 1][y] == 0:
        #         ret.append((x - 1, y))
        #     if x > 0 and y < 18 and board[x - 1][y + 1] == 0:
        #         ret.append((x - 1, y + 1))

        #     if y > 0 and board[x][y - 1] == 0:
        #         ret.append((x, y - 1))
        #     if y < 18 and board[x][y + 1] == 0:
        #         ret.append((x, y + 1))

        #     if x < 18 and y > 0 and board[x + 1][y - 1] == 0:
        #         ret.append((x + 1, y - 1))
        #     if x < 18 and board[x + 1][y] == 0:
        #         ret.append((x + 1, y))
        #     if x < 18 and y < 18 and board[x + 1][y + 1] == 0:
        #         ret.append((x + 1, y + 1))
        # return ret

    def captureRule(self, board, x, y, color):
        target = 1 if color == 2 else 2
        removedStone = []
        if y > 2 and board[x][y - 3] == color and (board[x][y - 2] == target and board[x][y - 1] == target):
            removedStone.append((x, y - 2))
            removedStone.append((x, y - 1))
        if x > 2 and y > 2 and board[x - 3][y - 3] == color and (board[x - 2][y - 2] == target and board[x - 1][y - 1] == target):
            removedStone.append((x - 2, y - 2))
            removedStone.append((x - 1, y - 1))
        if x > 2 and board[x - 3][y] == color and (board[x - 2][y] == target and board[x - 1][y] == target):
            removedStone.append((x - 2, y))
            removedStone.append((x - 1, y))
        if x > 2 and y < 16 and board[x - 3][y + 3] == color and (board[x - 2][y + 2] == target and board[x - 1][y + 1] == target):
            removedStone.append((x - 2, y + 2))
            removedStone.append((x - 1, y + 1))
        if y < 16 and board[x][y + 3] == color and (board[x][y + 2] == target and board[x][y + 1] == target):
            removedStone.append((x, y + 2))
            removedStone.append((x, y + 1))
        if x < 16 and y < 16 and board[x + 3][y + 3] == color and (board[x + 2][y + 2] == target and board[x + 1][y + 1] == target):
            removedStone.append((x + 2, y + 2))
            removedStone.append((x + 1, y + 1))
        if x < 16 and board[x + 3][y] == color and (board[x + 2][y] == target and board[x + 1][y] == target):
            removedStone.append((x + 2, y))
            removedStone.append((x + 1, y))
        if x < 16 and y > 2 and board[x + 3][y - 3] == color and (board[x + 2][y - 2] == target and board[x + 1][y - 1] == target):
            removedStone.append((x + 2, y - 2))
            removedStone.append((x + 1, y - 1))
        return removedStone

    def hasSameColorNeighbor(self, board, x, y, color, result):
        opponent = 1 if color == 2 else 2
        if (x > 0 and y > 0 and board[x - 1][y - 1] == opponent) and (x < 17 and y < 17 and board[x + 1][y + 1] == color and board[x + 2][y + 2] == 0):
                result.append((x + 2, y + 2))
        elif (x > 0 and y > 0 and board[x - 1][y - 1] == 0) and (x < 17 and y < 17 and board[x + 1][y + 1] == color and board[x + 2][y + 2] == opponent):
                result.append((x - 1, y - 1))
        if (x > 0 and board[x - 1][y] == opponent) and (x < 17 and board[x + 1][y] == color and board[x + 2][y] == 0):
                result.append((x + 2, y))
        elif (x > 0 and board[x - 1][y] == 0) and (x < 17 and board[x + 1][y] == color and board[x + 2][y] == opponent):
                result.append((x - 1, y))
        if (x > 0 and y < 18 and board[x - 1][y + 1] == opponent) and (x < 17 and y > 1 and board[x + 1][y - 1] == color and board[x + 2][y - 2] == 0):
                result.append((x + 2, y - 2))
        elif (x > 0 and y < 18 and board[x - 1][y + 1] == 0) and (x < 17 and y > 1 and board[x + 1][y - 1] == color and board[x + 2][y - 2] == opponent):
                result.append((x - 1, y + 1))
        if (y < 18 and board[x][y + 1] == opponent) and (y > 1 and board[x][y - 1] == color and board[x][y - 2] == 0):
                result.append((x, y - 2))
        elif (y < 18 and board[x][y + 1] == 0) and (y > 1 and board[x][y - 1] == color and board[x][y - 2] == opponent):
                result.append((x, y + 1))
        if (x < 18 and y < 18 and board[x + 1][y + 1] == opponent) and (x > 1 and y > 1 and board[x - 1][y - 1] == color and board[x - 2][y - 2] == 0):
                result.append((x - 2, y - 2))
        elif (x < 18 and y < 18 and board[x + 1][y + 1] == 0) and (x > 1 and y > 1 and board[x - 1][y - 1] == color and board[x - 2][y - 2] == opponent):
                result.append((x + 1, y + 1))
        if (x < 18 and board[x + 1][y] == opponent) and (x > 1 and board[x - 1][y] == color and board[x - 2][y] == 0):
                result.append((x - 2, y))
        elif (x < 18 and board[x + 1][y] == 0) and (x > 1 and board[x - 1][y] == color and board[x - 2][y] == opponent):
                result.append((x + 1, y))
        if (x < 18 and y > 0 and board[x + 1][y - 1] == opponent) and (x > 1 and y < 17 and board[x - 1][y + 1] == color and board[x - 2][y + 2] == 0):
                result.append((x - 2, y + 2))
        elif (x < 18 and y > 0 and board[x + 1][y - 1] == 0) and (x > 1 and y < 17 and board[x - 1][y + 1] == color and board[x - 2][y + 2] == opponent):
                result.append((x + 1, y - 1))
        if (y > 0 and board[x][y - 1] == opponent) and (y < 17 and board[x][y + 1] == color and board[x][y + 2] == 0):
                result.append((x, y + 2))
        elif (y > 0 and board[x][y - 1] == 0) and (y < 17 and board[x][y + 1] == color and board[x][y + 2] == opponent):
                result.append((x, y - 1))
        return result

    def gameEndingCaptureRule(self, board, v1, v2, color):
        self.winStart = v1
        self.winEnd = v2
        start_x = v1[0]
        end_x = v2[0]
        start_y = v1[1]
        end_y = v2[1]

        result = []
        while start_x != end_x or start_y != end_y:
            self.hasSameColorNeighbor(board, start_x, start_y, color, result)
            if start_x < end_x:
                start_x += 1
            elif start_x > end_x:
                start_x -= 1
            if start_y < end_y:
                start_y += 1
            elif start_y > end_y:
                start_y -= 1
        self.hasSameColorNeighbor(board, start_x, start_y, color, result)
        if result != []:
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

        hLine = cp.deepcopy(board[x])
        hLine[y] = color
        vLine = cp.deepcopy(board[:,y])
        vLine[x] = color

        d1Line = cp.deepcopy(np.diag(board, y - x))
        tmp = np.full(18 - (d1Line.size - 1), 1, dtype=int)
        if y < x:
            d1Line = np.concatenate((d1Line, tmp), axis=None)
        else:
            d1Line = np.concatenate((tmp, d1Line), axis=None)
        d1Line[y] = color

        d2Line = cp.deepcopy(np.diag(np.fliplr(board), (9 + (9 - y)) - x))
        d2Line = np.flip(d2Line)
        tmp = np.full(18 - (d2Line.size - 1), 1, dtype=int)
        if y >= 19 - x:
            d2Line = np.concatenate((tmp, d2Line), axis=None)
        else:
            d2Line = np.concatenate((d2Line, tmp), axis=None)
        d2Line[y] = color

        numberThree = 0
        for i in range(y - 4, y + 1):
            if hLine[i:i + len(pattern[0])].tolist() == pattern[0]:
                numberThree += 1
            elif hLine[i:i + len(pattern[1])].tolist() == pattern[1]:
                numberThree += 1
            elif hLine[i:i + len(pattern[2])].tolist() == pattern[2]:
                numberThree += 1
            if d2Line[i:i + len(pattern[0])].tolist() == pattern[0]:
                numberThree += 1
            elif d2Line[i:i + len(pattern[1])].tolist() == pattern[1]:
                numberThree += 1
            elif d2Line[i:i + len(pattern[2])].tolist() == pattern[2]:
                numberThree += 1
            if d1Line[i:i + len(pattern[0])].tolist() == pattern[0]:
                numberThree += 1
            elif d1Line[i:i + len(pattern[1])].tolist() == pattern[1]:
                numberThree += 1
            elif d1Line[i:i + len(pattern[2])].tolist() == pattern[2]:
                numberThree += 1
        for i in range(x - 4, x + 1):
            if vLine[i:i + len(pattern[0])].tolist() == pattern[0]:
                numberThree += 1
            elif vLine[i:i + len(pattern[1])].tolist() == pattern[1]:
                numberThree += 1
            elif vLine[i:i + len(pattern[2])].tolist() == pattern[2]:
                numberThree += 1
            if numberThree > 1:
                return numberThree
        return numberThree

    def doubleThreeRule(self, board, x, y, color):
        lst = []
        yMin = y if y < 3 else 3
        yMax = (19 - y) if y > 16 else 4
        for x1 in range(-3, 4):
            if x + x1 < 0 or x + x1 > 18:
                continue
            lst.append(board[x + x1][y - yMin:y + yMax].tolist())

        if self.searchThreePoint(board, x, y, color) > 1:
            return False
        return True

    def getValidPoints(self, board, color):
        if self.isWinner != 0:
            return self.gameEndingCaptureRule(board, self.winStart, self.winEnd, self.isWinner)
        return self.getBasicRule(board, color)
