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

    def captureRule(self, board, x, y, color):
        target = 1 if color == 2 else 2
        removedStone = []
        if board[x][y - 3] == color and (board[x][y - 2] == target and board[x][y - 1] == target):
            removedStone.append((x, y - 2))
            removedStone.append((x, y - 1))
        if board[x - 3][y - 3] == color and (board[x - 2][y - 2] == target and board[x - 1][y - 1] == target):
            removedStone.append((x - 2, y - 2))
            removedStone.append((x - 1, y - 1))
        if board[x - 3][y] == color and (board[x - 2][y] == target and board[x - 1][y] == target):
            removedStone.append((x - 2, y))
            removedStone.append((x - 1, y))
        if board[x - 3][y + 3] == color and (board[x - 2][y + 2] == target and board[x - 1][y + 1] == target):
            removedStone.append((x - 2, y + 2))
            removedStone.append((x - 1, y + 1))
        if board[x][y + 3] == color and (board[x][y + 2] == target and board[x][y + 1] == target):
            removedStone.append((x, y + 2))
            removedStone.append((x, y + 1))
        if board[x + 3][y + 3] == color and (board[x + 2][y + 2] == target and board[x + 1][y + 1] == target):
            removedStone.append((x + 2, y + 2))
            removedStone.append((x + 1, y + 1))
        if board[x + 3][y] == color and (board[x + 2][y] == target and board[x + 1][y] == target):
            removedStone.append((x + 2, y))
            removedStone.append((x + 1, y))
        if board[x + 3][y - 3] == color and (board[x + 2][y - 2] == target and board[x + 1][y - 1] == target):
            removedStone.append((x + 2, y - 2))
            removedStone.append((x + 1, y - 1))
        return removedStone

    def gameEndingCaputreRule(self, board, v1, v2, color):
        #Regarder pour chaque point de la ligne si ils peuvent etre capturer au prochaine tour
        return ()

    def getValidPoints(self, board, color):
        return self.getBasicRule(board, color)
        # validsPoint = []
