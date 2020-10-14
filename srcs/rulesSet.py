import numpy as np

class Rules():
    def __init__(self, options):
        self.options = options
        self.activeRules = options.rulesSet

    def checkBasicRule(self, board, x, y, color):
        target = 1 if color == 2 else 2
        tmpX = -1
        while tmpX < 2:
            if x + tmpX < 0 or x + tmpX > 18:
                tmpX += 1
                continue
            tmpY = -1
            while tmpY < 2:
                if y + tmpY < 0 or y + tmpY > 18:
                    tmpY += 1
                    continue
                if board[x + tmpX, y + tmpY] == target:
                    return True
                tmpY += 1
            tmpX += 1
        return False

    def getBasicRule(self, board, color):
        target = 1 if color == 2 else 2
        ret = []
        tmp = np.where(board == target)
        if len(tmp[0]) == 0 or len(tmp[1]) == 0:
            return
        for index in range(tmp[0].size):
            x = tmp[0][index]
            y = tmp[1][index]
            if x > 0 and y > 0 and board[x - 1][y - 1] == 0:
                ret.append((x - 1, y - 1))
            if x > 0 and board[x - 1][y] == 0:
                ret.append((x - 1, y))
            if x > 0 and y < 18 and board[x - 1][y + 1] == 0:
                ret.append((x - 1, y + 1))

            if y > 0 and board[x][y - 1] == 0:
                ret.append((x, y - 1))
            if y < 18 and board[x][y + 1] == 0:
                ret.append((x, y + 1))

            if x < 18 and y > 0 and board[x + 1][y - 1] == 0:
                ret.append((x + 1, y - 1))
            if x < 18 and board[x + 1][y] == 0:
                ret.append((x + 1, y))
            if x < 18 and y < 18 and board[x + 1][y + 1] == 0:
                ret.append((x + 1, y + 1))
        return ret

    def getValidPoints(self, board, color):
        return self.getBasicRule(board, color)
        # validsPoint = []
