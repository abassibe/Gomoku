from time import time
from random import randint
from PyQt5 import QtGui, QtCore, QtWidgets
import windowBuilding
import rulesSet
import numpy as np


# def dropHint(window, x, y):
#     global isBlack
#     global grid

#     if grid[y, x] != 0:
#         return None
#     dropPoint = window.boardGrid.itemAtPosition(y, x)
#     if isBlack:
#         grid[y, x] = 1
#         img = QtGui.QPixmap("ressources/pictures/blackStone.png")
#         p = QtGui.QPainter()
#         p.begin(img)
#         p.setCompositionMode(QtGui.QPainter.CompositionMode_DestinationIn)
#         p.fillRect(img.rect(), QtGui.QColor(0, 0, 0, 100))
#         p.end()
#         dropPoint.widget().setPixmap(img)
#     else:
#         grid[y, x] = 2
#         img = QtGui.QPixmap("ressources/pictures/whiteStone.png")
#         p = QtGui.QPainter()
#         p.begin(img)
#         p.setCompositionMode(QtGui.QPainter.CompositionMode_DestinationIn)
#         p.fillRect(img.rect(), QtGui.QColor(0, 0, 0, 100))
#         p.end()
#         dropPoint.widget().setPixmap(img)
#     isBlack = not isBlack
#     savedPlacedPoint.append(dropPoint)
#     window.update()
#     return 1


class HumanPlayer():
    def __init__(self, window, color):
        self.turnTime = QtCore.QTimer()
        self.turnTime.setInterval(10)
        self.color = color
        self.window = window
        self.timerText = None
        self.startTime = 0.0
        if color == 1:
            self.cursor = QtGui.QCursor(QtGui.QPixmap("ressources/pictures/blackStone.png"))
        else:
            self.cursor = QtGui.QCursor(QtGui.QPixmap("ressources/pictures/whiteStone.png"))
        self.turnTime.timeout.connect(lambda: windowBuilding.updateTimerGame(self.window, self.turnTime, self.startTime, self.timerText))
        self.stonePlacedLabel = None
        self.stonePlacedCount = 0
    
    def start(self):
        self.timerText.setText("00:00:00")
        self.window.layoutWidget.setCursor(self.cursor)

    def startTurn(self):
        if self.window.gameManager.hintButtonBool:
            x, y = self.window.algoPointer(self.window.gameManager.gameBoard.grid, self.color, True)
            self.window.gameManager.gameBoard.dropHint(x, y, self.color)
        self.window.layoutWidget.setCursor(self.cursor)
        windowBuilding.playerTurnEffect(self.window, self.color)
        self.turnTime.start()
        self.startTime = time()

    def endTurn(self, x, y):
        if self.stonePlacedCount >= 60:
            return
        self.turnTime.stop()
        self.window.gameManager.gameBoard.clearHint()
        if self.window.gameManager.gameBoard.placeStone(x, y, self.color, False) == None:
            return
        self.stonePlacedCount += 1
        self.stonePlacedLabel.setText(str(self.stonePlacedCount) + "/60")

    def end(self):
        self.turnTime.stop()


class ComputerPlayer():
    def __init__(self, window, color):
        self.turnTime = QtCore.QTimer()
        self.turnTime.setInterval(10)
        self.color = color
        self.window = window
        self.startTime = 0.0
        self.turnTime.timeout.connect(lambda: windowBuilding.updateTimerGame(self.window, self.turnTime, self.startTime, self.window.playerTwoTimer))
        self.stonePlacedLabel = None
        self.stonePlacedCount = 0

    def start(self):
        self.window.playerTwoTimer.setText("00:00:00")

    def startTurn(self):
        if self.stonePlacedCount >= 60:
            return
        self.turnTime.start()
        self.startTime = time()
        x, y = self.window.algoPointer(self.window.gameManager.gameBoard.grid, self.color, False)
        self.turnTime.stop()
        if self.window.gameManager.gameBoard.placeStone(x, y, self.color, True) == None:
            return
        self.stonePlacedCount += 1
        self.stonePlacedLabel.setText(str(self.stonePlacedCount) + "/60")

    def end(self):
        self.turnTime.stop()


class GameBoard():
    def __init__(self, window):
        self.window = window
        self.grid = np.zeros(shape=(19, 19), dtype=int)
        self.placedPoint = []
        self.placedHint = []

    def placeStone(self, x, y, color, computerMove):
        scaledX = 0
        scaledY = 0
        if computerMove:
            scaledX = x
            scaledY = y
        else:
            boardWidth = 761
            scaledX = x - self.window.layoutWidget.geometry().x()
            blockWidth = (boardWidth / 19)
            scaledX = int(scaledX / blockWidth)
            boardHeight = 761
            scaledY = y - self.window.layoutWidget.geometry().y()
            blockHeight = (boardHeight / 19)
            scaledY = int(scaledY / blockHeight)
        if self.grid[scaledX, scaledY] != 0 or not self.isValidMove(scaledX, scaledY, color):
            return None
        dropPoint = self.window.boardGrid.itemAtPosition(scaledX, scaledY)
        if color == 1:
            self.grid[scaledX, scaledY] = 1
            dropPoint.widget().setPixmap(QtGui.QPixmap("ressources/pictures/blackStone.png"))
        else:
            self.grid[scaledX, scaledY] = 2
            dropPoint.widget().setPixmap(QtGui.QPixmap("ressources/pictures/whiteStone.png"))
        self.window.gameManager.turnCount += 1
        self.placedPoint.append(dropPoint)
        if self.isWinner():
            pass
        if self.isDraw():
            pass
        self.window.update()
        self.window.gameManager.playerTurn = not self.window.gameManager.playerTurn
        return True

    def dropHint(self, x, y, color):
        if self.grid[y, x] != 0:
            return None
        dropPoint = self.window.boardGrid.itemAtPosition(y, x)
        img = None
        if color == 1:
            img = QtGui.QPixmap("ressources/pictures/blackStone.png")
        else:
            img = QtGui.QPixmap("ressources/pictures/whiteStone.png")
        p = QtGui.QPainter()
        p.begin(img)
        p.setCompositionMode(QtGui.QPainter.CompositionMode_DestinationIn)
        p.fillRect(img.rect(), QtGui.QColor(0, 0, 0, 100))
        p.end()
        dropPoint.widget().setPixmap(img)
        self.placedHint.append(dropPoint)
        self.window.update()

    def clear(self):
        self.grid = np.zeros(shape=(19, 19), dtype=int)
        for stone in self.placedPoint:
            stone.widget().clear()
        self.placedPoint = []

    def clearHint(self):
        for stone in self.placedHint:
            stone.widget().clear()
        self.placedHint = []

    def isValidMove(self, x, y, color):
        if self.window.gameManager.turnCount == 0:
            if x == 9 and y == 9:
                return True
            return False
        return self.window.gameManager.rules.checkBasicRule(self.grid, x, y, color)

    def isWinner(self):
        test1 = [1, 1, 1, 1, 1]
        # test2 = [2, 2, 2, 2, 2]
        # test3 = [[1][1][1][1][1]]
        # test4 = [[2][2][2][2][2]]
        # ret = tmp[0,(tmp[1:] == test1[:,None]).all(0)]
        ret = np.where(self.grid == test1[0])[0]
        solns = []
        N = len(test1)
        for p in ret:
            check = self.grid[p:p+N]
            if np.all(check == test1):
                solns.append(p)

        print(solns)
        return False

    def isDraw(self):
        pass

class GameManager():
    def __init__(self, window, option, hintButtonBool):
        self.isPlayer1Turn = True if randint(0, 1) == 0 else False
        self.player1 = HumanPlayer(window, 1 if self.isPlayer1Turn == True else 2)
        self.player1.timerText = window.playerOneTimer
        self.player1.stonePlacedLabel = window.player1StoneCount
        self.options = option
        if self.options.gameMode == "PVE":
            self.player2 = ComputerPlayer(window, 1 if self.isPlayer1Turn == False else 2)
        else:
            self.player2 = HumanPlayer(window, 1 if self.isPlayer1Turn == False else 2)
            self.player2.timerText = window.playerTwoTimer
        self.player2.stonePlacedLabel = window.player2StoneCount
        self.hintButtonBool = hintButtonBool
        self.window = window
        self.window.playerOneTimer.setText("00:00:00")
        self.window.playerTwoTimer.setText("00:00:00")
        self.window.player1StoneCount.setText("0/60")
        self.window.player2StoneCount.setText("0/60")
        self.gameBoard = GameBoard(window)
        self.turnCount = 0
        self.gameRuning = False
        self.globalTimer = QtCore.QTimer()
        self.globalTimer.setInterval(10)
        self.startGameTimer = 0.0
        self.globalTimer.timeout.connect(lambda: windowBuilding.updateTimerGame(window,
            self.globalTimer, self.startGameTimer, self.window.gameTimer))
        self._observers = [self.nextTurn]
        self.rules = rulesSet.Rules(self.options)

    @property
    def playerTurn(self):
        return self.isPlayer1Turn

    @playerTurn.setter
    def playerTurn(self, value):
        self.isPlayer1Turn = value
        for callback in self._observers:
            callback(self.isPlayer1Turn)

    def start(self):
        self.startGameTimer = time()
        self.globalTimer.start()
        self.gameRuning = True
        if self.isPlayer1Turn:
            self.player1.start()
            self.player1.startTurn()
        else:
            self.player2.start()
            self.player2.startTurn()

    def nextTurn(self, isPlayer1Turn):
        if isPlayer1Turn:
            self.player1.startTurn()
        else:
            self.player2.startTurn()

    def end(self):
        self.gameRuning = False
        self.globalTimer.stop()
        self.player1.end()
        self.player2.end()