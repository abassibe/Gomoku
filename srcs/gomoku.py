import sys
import PyQt5
from PyQt5.QtGui import *
from PyQt5 import uic, QtWidgets
import windowBuilding
import buttonEventHandler
import options

isBlackTurn = True
app = PyQt5.QtWidgets.QApplication(sys.argv)
window = uic.loadUi("GUI/mainwindow.ui")
option = options.Options()

window.optionsButton.clicked.connect(lambda x: buttonEventHandler.optionsEvent(window, option))
window.hintButton.clicked.connect(lambda x: buttonEventHandler.hintEvent(window.hintButton))
window.giveUpButton.clicked.connect(lambda x: buttonEventHandler.giveUpEvent(window))
window.newGameButton.clicked.connect(lambda x: buttonEventHandler.newGameEvent(window))
window.gameBoard
windowBuilding.setFontShadow(window)
windowBuilding.setRulesList(window, option.rulesSet)


def getOptionsSet(targetedOption=[]):
    """
        Return list of options.

        If targetedOption=None return a list of all options.

        Otherwise, specify wich option you want by sending a list of string. ex getOptionsSet(['langage', 'gameMode']).
        
        Available options: langage, gameMode, rulesSet.
    """
    if targetedOption == []:
        return option.langage, option.gameMode, option.rulesSet
    else:
        toReturn = []
        for item in targetedOption:
            try:
                toReturn.append(option.__getattribute__(item))
            except:
                exit("Unknown option: " + item)
        return toReturn


def algoSubscribe(func):
    x, y = func("board", buttonEventHandler._hintButtonBool)
    return x, y


window.show()
app.exec()
