from time import time
from random import randint
from PyQt5 import QtGui, QtCore, QtWidgets
import windowBuilding
import numpy as np

grid = np.zeros(shape=(19, 19))
isBlack = True
savedPlacedPoint = []


def dropHint(window, x, y):
    global isBlack
    global grid

    if grid[y, x] != 0:
        return None
    dropPoint = window.boardGrid.itemAtPosition(y, x)
    if isBlack:
        grid[y, x] = 1
        img = QtGui.QPixmap("ressources/pictures/blackStone.png")
        p = QtGui.QPainter()
        p.begin(img)
        p.setCompositionMode(QtGui.QPainter.CompositionMode_DestinationIn)
        p.fillRect(img.rect(), QtGui.QColor(0, 0, 0, 100))
        p.end()
        dropPoint.widget().setPixmap(img)
    else:
        grid[y, x] = 2
        img = QtGui.QPixmap("ressources/pictures/whiteStone.png")
        p = QtGui.QPainter()
        p.begin(img)
        p.setCompositionMode(QtGui.QPainter.CompositionMode_DestinationIn)
        p.fillRect(img.rect(), QtGui.QColor(0, 0, 0, 100))
        p.end()
        dropPoint.widget().setPixmap(img)
    isBlack = not isBlack
    savedPlacedPoint.append(dropPoint)
    window.update()
    return 1


def dropStone(window, x, y, computerMove):
    global isBlack
    global grid
    global savedPlacedPoint

    scaledX = 0
    scaledY = 0
    if computerMove:
        scaledX = x
        scaledY = y
    else:
        boardWidth = window.boardGrid.contentsRect().width()
        scaledX = x - window.layoutWidget.geometry().x()
        blockWidth = (boardWidth / 19)
        scaledX = int(scaledX / blockWidth)
        boardHeight = window.boardGrid.contentsRect().height()
        scaledY = y - window.layoutWidget.geometry().y()
        blockHeight = (boardHeight / 19)
        scaledY = int(scaledY / blockHeight)
    # if grid[scaledY, scaledX] != 0 or not isValidMove():
    if grid[scaledY, scaledX] != 0:
        return None
    dropPoint = window.boardGrid.itemAtPosition(scaledY, scaledX)
    if isBlack:
        grid[scaledY, scaledX] = 1
        dropPoint.widget().setPixmap(QtGui.QPixmap("ressources/pictures/blackStone.png"))
    else:
        grid[scaledY, scaledX] = 2
        dropPoint.widget().setPixmap(QtGui.QPixmap("ressources/pictures/whiteStone.png"))
    isBlack = not isBlack
    savedPlacedPoint.append(dropPoint)
    # if isWinner():
    #     winner()
    window.update()
    window.gameManager.playerTurn = not window.gameManager.playerTurn
    return 1


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
    
    def start(self):
        self.timerText.setText("00:00:00")
        self.window.layoutWidget.setCursor(self.cursor)

    def startTurn(self):
        self.window.layoutWidget.setCursor(self.cursor)
        windowBuilding.playerTurnEffect(self.window, self.color)
        self.turnTime.start()
        self.startTime = time()

    def endTurn(self, x, y):
        self.turnTime.stop()
        dropStone(self.window, x, y, False)

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

    def start(self):
        self.window.playerTwoTimer.setText("00:00:00")

    def startTurn(self):
        self.turnTime.start()
        self.startTime = time()
        x, y = self.window.algoPointer(grid, self.color, False)
        self.turnTime.stop()
        dropStone(self.window, x, y, True)

    def end(self):
        self.turnTime.stop()


class GameBoard():
    def __init__(self):
        self.grid = np.zeros(shape=(19, 19))
        self.placedPoint = []

    def placeStone(self, stone):
        self.placedPoint.append(stone)

    def clear(self):
        for stone in self.placedPoint:
            stone.widget().clear()


class GameManager():
    def __init__(self, window, option, hintButtonBool):
        self.isPlayer1Turn = True if randint(0, 1) == 0 else False
        self.player1 = HumanPlayer(window, 1 if self.isPlayer1Turn == True else 2)
        self.player1.timerText = window.playerOneTimer
        self.options = option
        if self.options.gameMode == "PVE":
            self.player2 = ComputerPlayer(window, 1 if self.isPlayer1Turn == False else 2)
        else:
            self.player2 = HumanPlayer(window, 1 if self.isPlayer1Turn == False else 2)
            self.player2.timerText = window.playerTwoTimer
        self.hintButtonBool = hintButtonBool
        self.window = window
        self.window.playerOneTimer.setText("00:00:00")
        self.window.playerTwoTimer.setText("00:00:00")
        self.gameBoard = GameBoard()
        self.turnCount = 0
        self.gameRuning = False
        self.globalTimer = QtCore.QTimer()
        self.globalTimer.setInterval(10)
        self.startGameTimer = 0.0
        self.globalTimer.timeout.connect(lambda: windowBuilding.updateTimerGame(window,
            self.globalTimer, self.startGameTimer, self.window.gameTimer))
        self._observers = [self.nextTurn]

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
        self.turnCount += 1

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
