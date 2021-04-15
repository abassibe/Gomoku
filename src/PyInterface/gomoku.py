import pathlib
import sys
import PyQt5
from PyQt5.QtGui import *
from PyQt5 import uic, QtWidgets
import windowBuilding
import buttonEventHandler
import options
import gameManager
import rust_ext as rst
from random import randint
import time
import rulesSet

window = None


class MainWindow(QtWidgets.QMainWindow):
    def __init__(self):
        super(MainWindow, self).__init__()

        self.isBlackTurn = True
        self.local = "en_EN"
        uic.loadUi(str(pathlib.Path("GUI/mainwindow.ui")), self)
        self.option = options.Options()
        windowBuilding.parseTranslationFile()
        self.setWindowTitle("Gomoku")
        self.gameManager = None
        self.setFixedSize(self.geometry().width(), self.geometry().height())

        self.optionsButton.clicked.connect(lambda x: buttonEventHandler.optionsEvent(self, self.option))
        self.hintButton.clicked.connect(lambda x: buttonEventHandler.hintEvent(self.hintButton, window))
        self.giveUpButton.clicked.connect(lambda x: buttonEventHandler.giveUpEvent(self))
        self.newGameButton.clicked.connect(lambda x: buttonEventHandler.newGameEvent(self, self.option))
        self.algoPointer = None

        windowBuilding.setFontShadow(self)
        windowBuilding.setRulesList(self, self.option.rulesSet)

    def mousePressEvent(self, event):
        if self.gameManager is None or self.gameManager.gameRuning is False or (self.option.gameMode == "PVE" and not
        self.gameManager.isPlayer1Turn):
            return
        if event.button() == 1:
            y = event.x()
            x = event.y()
            if (x < 87 or x > 716) or (y < 100 or y > 729):
                return
            if self.gameManager.playerTurn:
                self.gameManager.player1.endTurn(x, y)
            else:
                self.gameManager.player2.endTurn(x, y)


def getOptionsSet(targeted_option=[]):
    """
        Return list of options.

        If targetedOption=None return a list of all options.

        Otherwise, specify wich option you want by sending a list of string. ex getOptionsSet(['langage', 'gameMode']).
        
        Available options: langage, gameMode, rulesSet.
    """
    if targeted_option == []:
        return window.option.langage, window.option.gameMode, window.option.rulesSet
    else:
        to_return = []
        for item in targeted_option:
            try:
                to_return.append(window.option.__getattribute__(item))
            except:
                exit("Unknown option: " + item)
        return to_return


def algoSubscribe(func):
    """
        Function used to connect the algo and the GUI.

        Param "func" must be the entrance of algorithm with following signature : func(board, playerColor, hint)

        Where "board" is a matrix of the actual state of the board, "playerColor" tell you if he's black(1) or white(2)
        and "hint" is a boolean that tells you if it's the algorithm's turn or just a hint you're looking for.

        And the return value must be two integer "x" and "y", representing the position of the move. (0 <= xy <= 19)
    """
    global window
    window.algoPointer = func


app = PyQt5.QtWidgets.QApplication(sys.argv)
window = MainWindow()
algoSubscribe(rst.get_next_move)
window.show()
app.exec()
