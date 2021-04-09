import pathlib
from time import time
from random import randint
from PyQt5 import QtGui, QtCore, QtWidgets
from PyQt5.QtCore import QObject, QThread, pyqtSignal, QRunnable, pyqtSlot, QThreadPool
from PyQt5.QtWidgets import QWidget, QApplication
import windowBuilding
import rulesSet
import numpy as np
from threading import Thread, Event

last_move_ai = (0, 0)
last_move_human = (0, 0)
continue_game = False


def unSetForbiddenCursor(cursor, window):
    window.layoutWidget.setCursor(cursor)


class Worker(QObject):
    finished = pyqtSignal()
    progress = pyqtSignal(int)

    def setup(self, function, window, color, last_move_human, last_move_ai, computPlayer):
        self.window = window
        self.color = color
        self.last_move_human = last_move_human
        self.last_move_ai = last_move_ai
        self.function = function
        self.computPlayer = computPlayer

    def run(self):
        """Long-running task."""
        self.computPlayer.x, self.computPlayer.y = self.function(self.window.gameManager.gameBoard.grid, self.color,
                                                                 False,
                                                                 self.window.gameManager.player1.stoneRemovedCount,
                                                                 self.window.gameManager.player2.stoneRemovedCount,
                                                                 self.last_move_human, self.last_move_ai)
        self.progress.emit(1)
        self.finished.emit()


class HumanPlayer():
    def __init__(self, window, color):
        self.turnTime = QtCore.QTimer()
        self.turnTime.setInterval(10)
        self.color = color
        self.colorLabel = None
        self.window = window
        self.timerText = None
        self.startTime = 0.0
        if color == 1:
            self.cursor = QtGui.QCursor(QtGui.QPixmap(str(pathlib.Path("ressources/pictures/blackStone.png"))))
        else:
            self.cursor = QtGui.QCursor(QtGui.QPixmap(str(pathlib.Path("ressources/pictures/whiteStone.png"))))
        self.turnTime.timeout.connect(
            lambda: windowBuilding.updateTimerGame(self.window, self.turnTime, self.startTime, self.timerText))
        self.playerCapture = None
        self.stoneRemovedCount = 0
        self.x = 0
        self.y = 0

    def start(self):
        self.timerText.setText("00:00:00")
        if self.color == 1:
            self.colorLabel.setStyleSheet(
                "background-color: rgba(255, 255, 255, 0);color:rgb(0, 0, 0);font: 24pt \"SF Wasabi\";")
        else:
            self.colorLabel.setStyleSheet(
                "background-color: rgba(255, 255, 255, 0);color:rgb(255, 255, 255);font: 24pt \"SF Wasabi\";")

    def rustReturn(self):
        global last_move_human
        last_move_human = (self.x, self.y)
        self.window.gameManager.gameBoard.dropHint(self.x, self.y, self.color)

    def startTurn(self):
        self.window.layoutWidget.setCursor(self.cursor)
        global last_move_human
        if self.window.gameManager.hintButtonBool:
            self.thread = QThread()
            self.worker = Worker()
            self.worker.setup(self.window.algoPointer, self.window, self.color, last_move_human, last_move_ai, self)
            self.worker.moveToThread(self.thread)
            self.thread.started.connect(self.worker.run)
            self.worker.finished.connect(self.thread.quit)
            self.worker.finished.connect(self.worker.deleteLater)
            self.thread.finished.connect(self.thread.deleteLater)
            self.thread.start()

            self.thread.finished.connect(lambda: self.rustReturn())

        self.window.layoutWidget.setCursor(self.cursor)
        windowBuilding.playerTurnEffect(self.window, self.color)
        self.turnTime.start()
        self.startTime = time()

    def endTurn(self, x, y):
        if self.window.gameManager.gameBoard.placeStone(x, y, self.color, False) is None:
            return
        self.turnTime.stop()
        self.playerCapture.setText(str(self.stoneRemovedCount) + "/10")
        self.window.gameManager.playerTurn = not self.window.gameManager.playerTurn

    def end(self):
        self.turnTime.stop()


def test():
    print("Thread end !")


class ComputerPlayer(object):
    def __init__(self, window, color):
        self.turnTime = QtCore.QTimer()
        self.turnTime.setInterval(10)
        self.color = color
        self.colorLabel = window.playerTwoLabel
        self.window = window
        self.startTime = 0.0
        self.x = 0
        self.y = 0

        if self.color == 1:
            self.colorLabel.setStyleSheet(
                "background-color: rgba(255, 255, 255, 0);color:rgb(0, 0, 0);font: 24pt \"SF Wasabi\";")
        else:
            self.colorLabel.setStyleSheet(
                "background-color: rgba(255, 255, 255, 0);color:rgb(255, 255, 255);font: 24pt \"SF Wasabi\";")
        self.turnTime.timeout.connect(lambda: windowBuilding.updateTimerGame(self.window, self.turnTime, self.startTime,
                                                                             self.window.playerTwoTimer))
        self.playerCapture = None
        self.stoneRemovedCount = 0

    def start(self):
        self.window.playerTwoTimer.setText("00:00:00")

    def rustReturn(self):
        self.turnTime.stop()
        last_move_ai = (self.x, self.y)  ##

        if self.window.gameManager.gameBoard.placeStone(self.x, self.y, self.color, True) is None:
            return

        self.playerCapture.setText(str(self.stoneRemovedCount) + "/10")
        self.window.gameManager.playerTurn = not self.window.gameManager.playerTurn

    def startTurn(self):
        self.turnTime.start()
        self.startTime = time()

        global last_move_ai
        global last_move_human

        self.thread = QThread()
        self.worker = Worker()
        self.worker.setup(self.window.algoPointer, self.window, self.color, last_move_human, last_move_ai, self)
        self.worker.moveToThread(self.thread)
        self.thread.started.connect(self.worker.run)
        self.worker.finished.connect(self.thread.quit)
        self.worker.finished.connect(self.worker.deleteLater)
        self.thread.finished.connect(self.thread.deleteLater)
        self.thread.start()

        self.thread.finished.connect(lambda: self.rustReturn())

    def finishTurn(self, x, y):
        if not y or not x:
            return

        self.turnTime.stop()
        last_move_ai = (x, y)  ##

        if self.window.gameManager.gameBoard.placeStone(x, y, self.color, True) is None:
            return

        self.playerCapture.setText(str(self.stoneRemovedCount) + "/10")
        self.window.gameManager.playerTurn = not self.window.gameManager.playerTurn

    def end(self):
        self.turnTime.stop()


class GameBoard():
    def __init__(self, window):
        self.window = window
        self.grid = np.zeros(shape=(19, 19), dtype=np.uint8)
        self.placedPoint = []
        self.placedHint = []

    def highLightWinningLine(self, x, y):
        # -------------------------------------------#
        #               Part fixed                  #
        ##widget = window_attr_lst['centralwidget']
        window_attr_lst = vars(self.window)
        painter = QtGui.QPainter()
        painter.setPen(QtCore.Qt.red)
        # painter.begin(QWidget or self) begin ?
        painter.drawLine(10, 10, 200, 200)
        # painter.end(QWidget or self) end ?
        #                                           #
        # -------------------------------------------#

        # -------------------------------------------#
        #                   Old part                #
        # widget = self.window.__getattr__("centralwidget")
        # widget.drawLine(10, 10, 200, 200)
        #                                           #
        # -------------------------------------------#

    def placeStone(self, x, y, color, computerMove):
        scaledX = 0
        scaledY = 0
        global last_move_human
        global last_move_ai

        if computerMove:
            scaledX = x
            scaledY = y
            last_move_ai = (scaledX, scaledY)  # update last ai move for rust side

        else:
            blockSize = (629 / 19)
            scaledX = x - self.window.layoutWidget.geometry().y()
            scaledX = int(scaledX / blockSize)
            scaledY = y - self.window.layoutWidget.geometry().x()
            scaledY = int(scaledY / blockSize)

            last_move_human = (scaledX, scaledY)  # update last human move for rust side
        if self.grid[scaledX, scaledY] != 0 or not self.isValidMove(scaledX, scaledY, color):
            tmp = self.window.layoutWidget.cursor()
            self.window.layoutWidget.setCursor(QtGui.QCursor(QtCore.Qt.ForbiddenCursor))
            QtCore.QTimer.singleShot(1000, lambda: unSetForbiddenCursor(tmp, self.window))
            return None

        self.window.gameManager.gameBoard.clearHint()
        dropPoint = self.window.boardGrid.itemAtPosition(scaledX, scaledY)
        if color == 1:
            self.grid[scaledX, scaledY] = 1
            dropPoint.widget().setPixmap(QtGui.QPixmap(str(pathlib.Path("ressources/pictures/blackStone.png"))))
        else:
            self.grid[scaledX, scaledY] = 2
            dropPoint.widget().setPixmap(QtGui.QPixmap(str(pathlib.Path("ressources/pictures/whiteStone.png"))))
        self.placedPoint.append(dropPoint)
        if 'Capture' in self.window.option.rulesSet:
            removedStone = self.window.gameManager.rules.captureRule(self.grid, scaledX, scaledY, color)
            for stone in removedStone:
                dropPoint = self.window.boardGrid.itemAtPosition(stone[0], stone[1])
                dropPoint.widget().clear()
                self.grid[stone[0]][stone[1]] = 0
                removedStonePlayer = self.window.gameManager.player1 if color == self.window.gameManager.player1.color else self.window.gameManager.player2
                removedStonePlayer.stoneRemovedCount += 1
        winStart, winEnd = self.isWinner()
        print("color : ", color)
        global continue_game
        if type(winStart) is tuple and type(winEnd) is tuple and (
                'Game-ending capture' in self.window.option.rulesSet or 'Capture fin de partie' in self.window.option.rulesSet):
            counterCapture = self.window.gameManager.rules.gameEndingCaptureRule(self.grid, winStart, winEnd, color)

            if len(counterCapture) == 0 and not continue_game:
                if ((color == self.window.gameManager.player1.color and self.window.gameManager.player2.stoneRemovedCount == 8)
                    or (color == self.window.gameManager.player2.color and self.window.gameManager.player1.stoneRemovedCount == 8)):
                    if self.window.gameManager.rules.checkPotentialCapture(self.grid, self.window.gameManager.player2.color if color ==
                                                                           self.window.gameManager.player1.color else self.window.gameManager.player1.color):
                        continue_game = True
                        return True
            elif len(counterCapture) > 0:
                return True

        if winStart:
            self.window.gameManager.gameBoard.clearHint()
            self.window.gameManager.end()
            self.window.layoutWidget.unsetCursor()
            windowBuilding.winDraw(self.window, 1, color)
            self.highLightWinningLine(x, y)
            return True
        self.window.update()
        return True

    def dropHint(self, x, y, color):
        if self.grid[x, y] != 0:
            return None
        dropPoint = self.window.boardGrid.itemAtPosition(x, y)
        img = None
        if color == 1:
            img = QtGui.QPixmap(str(pathlib.Path("ressources/pictures/blackStone.png")))
        else:
            img = QtGui.QPixmap(str(pathlib.Path("ressources/pictures/whiteStone.png")))
        p = QtGui.QPainter()
        p.begin(img)
        p.setCompositionMode(QtGui.QPainter.CompositionMode_DestinationIn)
        p.fillRect(img.rect(), QtGui.QColor(0, 0, 0, 100))
        p.end()
        dropPoint.widget().setPixmap(img)
        self.placedHint.append(dropPoint)
        self.window.update()

    def clear(self):
        self.grid = np.zeros(shape=(19, 19), dtype=np.uint8)
        for stone in self.placedPoint:
            stone.widget().clear()
        self.placedPoint = []

    def clearHint(self):
        for stone in self.placedHint:
            stone.widget().clear()
        self.placedHint = []

    def isValidMove(self, x, y, color):
        isDoubleThreeRule = True if (
                    "Double trois" in self.window.gameManager.rules.activeRules or "Double three" in self.window.gameManager.rules.activeRules) else False
        if isDoubleThreeRule and not self.window.gameManager.rules.doubleThreeRule(self.grid, x, y, color):
            return False
        if self.window.gameManager.rules.isWinner != 0:
            ret = self.window.gameManager.rules.getValidPoints(self.grid, color)
            for validX, validY in ret:
                if x == validX and y == validY:
                    self.window.gameManager.rules.isWinner = False
                    self.winStart = None
                    self.winEnd = None
                    return True
            return False
        return self.window.gameManager.rules.checkBasicRule(self.grid, x, y, color)

    def isWinner(self):
        if self.window.gameManager.player1.stoneRemovedCount >= 10:
            return self.window.gameManager.player1.color, self.window.gameManager.player1.color
        elif self.window.gameManager.player2.stoneRemovedCount >= 10:
            return self.window.gameManager.player2.color, self.window.gameManager.player2.color

        for x in range(19):
            for y in range(19):
                if self.grid[x][y] != 0:
                    toCompare = self.grid[x][y]
                    if x < 14:
                        for n in range(1, 5):
                            if self.grid[x + n][y] != toCompare:
                                break
                            if n + 1 == 5:
                                return (x, y), (x + n, y)
                    if y < 14:
                        for n in range(1, 5):
                            if self.grid[x][y + n] != toCompare:
                                break
                            if n + 1 == 5:
                                return (x, y), (x, y + n)
                    if x < 14 and y < 14:
                        for n in range(1, 5):
                            if self.grid[x + n][y + n] != toCompare:
                                break
                            if n + 1 == 5:
                                return (x, y), (x + n, y + n)
                    if x < 14 and y > 3:
                        for n in range(1, 5):
                            if self.grid[x + n][y - n] != toCompare:
                                break
                            if n + 1 == 5:
                                return (x, y), (x + n, y - n)
        return None, None


class GameManager():
    def __init__(self, window, option, hintButtonBool):
        self.isPlayer1Turn = True if randint(0, 1) == 0 else False
        self.player1 = HumanPlayer(window, 1 if self.isPlayer1Turn == True else 2)
        self.player1.timerText = window.playerOneTimer
        self.player1.colorLabel = window.playerOneLabel
        self.player1.playerCapture = window.player1Capture
        self.options = option
        if self.options.gameMode == "PVE":
            self.player2 = ComputerPlayer(window, 1 if self.isPlayer1Turn == False else 2)
        else:
            self.player2 = HumanPlayer(window, 1 if self.isPlayer1Turn == False else 2)
            self.player2.timerText = window.playerTwoTimer
            self.player2.colorLabel = window.playerTwoLabel
        self.player2.playerCapture = window.player2Capture
        self.hintButtonBool = hintButtonBool
        self.window = window
        self.window.playerOneTimer.setText("00:00:00")
        self.window.playerTwoTimer.setText("00:00:00")
        self.window.player1Capture.setText("0/10")
        self.window.player2Capture.setText("0/10")
        self.gameBoard = GameBoard(window)
        self.gameRuning = False
        self.globalTimer = QtCore.QTimer()
        self.globalTimer.setInterval(10)
        self.startGameTimer = 0.0
        self.globalTimer.timeout.connect(lambda: windowBuilding.updateTimerGame(window,
                                                                                self.globalTimer, self.startGameTimer,
                                                                                self.window.gameTimer))
        self._observers = [self.nextTurn]
        self.rules = rulesSet.Rules(self.options)
        self.turnCount = 0

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
        self.player1.start()
        self.player2.start()
        if self.isPlayer1Turn:
            self.player1.startTurn()
        else:
            self.player2.startTurn()

    def nextTurn(self, isPlayer1Turn):
        if not self.gameRuning:
            return
        self.turnCount += 1
        self.window.turnsValue.setText(str(int(self.turnCount / 2)))
        if isPlayer1Turn:
            self.player1.startTurn()
        else:
            self.player2.startTurn()

    def end(self):
        self.gameRuning = False
        self.globalTimer.stop()
        self.player1.end()
        self.player2.end()
