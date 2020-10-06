from time import time
from random import randint
from PyQt5 import QtGui, QtCore, QtWidgets
import windowBuilding
import numpy as np

player1TurnTime = QtCore.QTimer()
player2TurnTime = QtCore.QTimer()
player1TurnTime.setInterval(10)
player2TurnTime.setInterval(10)
isPlayer1Turn = None
optins = None
turnCount = 0
grid = np.zeros(shape=(19, 19))
isBlack = True
savedPlacedPoint = []


def dropHint(window, x, y):
    global isBlack
    global grid

    # if grid[scaledY, scaledX] != 0 or not isValidMove():
    if grid[y, x] != 0:
        return None
    dropPoint = window.boardGrid.itemAtPosition(y, x)
    if isBlack:
        grid[y, x] = 1
        dropPoint.widget().setPixmap(QtGui.QPixmap("ressources/pictures/blackStone.png"))
    else:
        grid[y, x] = 2
        dropPoint.widget().setPixmap(QtGui.QPixmap("ressources/pictures/whiteStone.png"))
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
    img = dropPoint.widget().pixmap().createMaskFromColor(QtGui.QColor(0, 0, 0, 255))
    isBlack = not isBlack
    savedPlacedPoint.append(dropPoint)
    # if isWinner():
    #     winner()
    window.update()
    return 1


def nextTurn(window, x, y, computerMove, hint):
    global isPlayer1Turn
    global player1TurnTime
    global player2TurnTime
    global options
    global turnCount
    global grid

    if isPlayer1Turn:
        if dropStone(window, x, y, computerMove) == None:
            return None
        player1TurnTime.stop()
        windowBuilding.playerTurnEffect(window, 2)
        player2TurnTime.start()
        startTurnTimer = time()
        player2TurnTime.timeout.connect(lambda: windowBuilding.updateTimerGame(window, player2TurnTime, startTurnTimer, window.playerTwoTimer))
    else:
        if dropStone(window, x, y, computerMove) == None:
            return None
        player2TurnTime.stop()
        windowBuilding.playerTurnEffect(window, 1)
        player1TurnTime.start()
        startTurnTimer = time()
        player1TurnTime.timeout.connect(lambda: windowBuilding.updateTimerGame(window, player1TurnTime, startTurnTimer, window.playerOneTimer))
    if hint:
        x, y = window.algoPointer(grid, hint)
        dropHint(window, x, y)
    turnCount += 1
    if options.gameMode == "PVP" and turnCount % 2 == 1:
        window.layoutWidget.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/whiteStone.png")))
    elif options.gameMode == "PVP":
        window.layoutWidget.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/blackStone.png")))
    isPlayer1Turn = not isPlayer1Turn
    if not isPlayer1Turn and options.gameMode == "PVE":
        return 1


def gameManager(window, option, _hintButtonBool):
    global player1TurnTime
    global player2TurnTime
    global isPlayer1Turn
    global options
    global isBlack

    isBlack = True
    turnCount = 0
    options = option
    if randint(0, 1) == 0:
        isPlayer1Turn = True
        windowBuilding.playerTurnEffect(window, 1)
        player1TurnTime.start()
    else:
        isPlayer1Turn = False
        windowBuilding.playerTurnEffect(window, 2)
        player2TurnTime.start()
    if options.gameMode == "PVE" and not isPlayer1Turn:
        window.layoutWidget.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/whiteStone.png")))
        x, y = window.algoPointer(grid, _hintButtonBool)
        nextTurn(window, x, y, True, _hintButtonBool)
    else:
        window.layoutWidget.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/blackStone.png")))
