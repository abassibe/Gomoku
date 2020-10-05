from time import time
from random import randint
from PyQt5 import QtGui, QtCore
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


def dropStone(window, x, y):
    newendX = 911 - 150
    newX = x - 150
    blockSize = (newendX / 19)
    newX = int(newX / blockSize)
    b = window.boardGrid.itemAtPosition(1, 1)
    print(newX)


def nextTurn(window, x, y):
    global isPlayer1Turn
    global player1TurnTime
    global player2TurnTime
    global options
    global turnCount

    turnCount += 1
    if isPlayer1Turn:
        dropStone(window, x, y)
        # poser le pion du joueur 1
        player1TurnTime.stop()
        windowBuilding.playerTurnEffect(window, 2)
        player2TurnTime.start()
        startTurnTimer = time()
        player2TurnTime.timeout.connect(lambda: windowBuilding.updateTimerGame(window, player2TurnTime, startTurnTimer, window.playerTwoTimer))
    else:
        dropStone(window, x, y)
        # poser le pion du joueur 2
        player2TurnTime.stop()
        windowBuilding.playerTurnEffect(window, 1)
        player1TurnTime.start()
        startTurnTimer = time()
        player1TurnTime.timeout.connect(lambda: windowBuilding.updateTimerGame(window, player1TurnTime, startTurnTimer, window.playerOneTimer))
    if options.gameMode == "PVP" and turnCount % 2 == 1:
        window.gameBoard.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/whiteStone.png")))
    elif options.gameMode == "PVP":
        window.gameBoard.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/blackStone.png")))
    isPlayer1Turn = not isPlayer1Turn


def gameManager(window, option):
    global player1TurnTime
    global player2TurnTime
    global isPlayer1Turn
    global options

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
        window.gameBoard.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/whiteStone.png")))
    else:
        window.gameBoard.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/blackStone.png")))
