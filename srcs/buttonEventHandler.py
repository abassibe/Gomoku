from time import time
from PyQt5 import uic, QtWidgets, QtGui, QtCore
from random import randint
import options
import windowBuilding

_hintButtonBool = False
_giveUpButtonBool = False
_newGameButtonBool = False
isGameRuning = False
player1TurnTime = 0.0
player2TurnTime = 0.0
isPlayer1Turn = True


def optionsEvent(window, option):
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(-5, -5)
    window.optionsButton.setGraphicsEffect(effect)
    window.optionsButton.setGeometry(1129, 927, 51, 51)

    dialog = uic.loadUi("GUI/dialog.ui")
    dialog.langageCombobox.setCurrentIndex(dialog.langageCombobox.findText(option.langage))
    if option.gameMode == "PVE":
        dialog.PVEButton.setChecked(True)
        dialog.PVPButton.setChecked(False)
    else:
        dialog.PVEButton.setChecked(False)
        dialog.PVPButton.setChecked(True)
    dialog.buttonBox.accepted.connect(lambda: option._onAccept(window, dialog))
    for rule in option.rulesSet:
        if rule == dialog.ruleCheckbox1.text():
            dialog.ruleCheckbox1.setChecked(True)
        if rule == dialog.ruleCheckbox2.text():
            dialog.ruleCheckbox2.setChecked(True)
        if rule == dialog.ruleCheckbox3.text():
            dialog.ruleCheckbox3.setChecked(True)
        if rule == dialog.ruleCheckbox4.text():
            dialog.ruleCheckbox4.setChecked(True)

    dialog.show()
    dialog.exec()

    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.optionsButton.setGraphicsEffect(effect)
    window.optionsButton.setGeometry(1129, 927, 61, 61)


def hintEvent(hintButton):
    global _hintButtonBool
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    if not _hintButtonBool:
        effect.setColor(QtGui.QColor(0, 0, 0, 120))
        effect.setOffset(-7, -7)
        hintButton.setGraphicsEffect(effect)
        hintButton.setGeometry(60, 980, 235, 55)
        _hintButtonBool = True

    else:
        effect.setColor(QtGui.QColor(0, 0, 0, 90))
        effect.setOffset(-10, -10)
        hintButton.setGraphicsEffect(effect)
        hintButton.setGeometry(60, 980, 241, 61)
        _hintButtonBool = False


def giveUpEvent(giveUpButton):
    global _giveUpButtonBool
    global isGameRuning
    isGameRuning = False
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    if not _giveUpButtonBool:
        effect.setColor(QtGui.QColor(0, 0, 0, 150))
        effect.setOffset(-7, -7)
        giveUpButton.setGraphicsEffect(effect)
        giveUpButton.setGeometry(400, 980, 235, 55)
        _giveUpButtonBool = True
    else:
        effect.setColor(QtGui.QColor(0, 0, 0, 90))
        effect.setOffset(-10, -10)
        giveUpButton.setGraphicsEffect(effect)
        giveUpButton.setGeometry(400, 980, 241, 61)
        _giveUpButtonBool = False


def releaseNGButton(window, effect):
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.newGameButton.setGraphicsEffect(effect)
    window.newGameButton.setGeometry(750, 980, 241, 61)
    _newGameButtonBool = False


def newGameEvent(window):
    global _newGameButtonBool
    global isGameRuning
    global player1TurnTime
    global player2TurnTime
    global isPlayer1Turn
    if isGameRuning:
        return
    if randint(0, 1) == 0:
        isPlayer1Turn = True
    else:
        isPlayer1Turn = False
    isGameRuning = True
    player1TurnTime = 0.0
    player2TurnTime = 0.0

    timer = QtCore.QTimer()
    startGameTimer = time()
    timer.setInterval(10)
    timer.timeout.connect(lambda: windowBuilding.updateTimerGame(window, timer, startGameTimer))
    timer.start()

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    QtCore.QTimer.singleShot(150, lambda: releaseNGButton(window, effect))
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(-7, -7)
    window.newGameButton.setGraphicsEffect(effect)
    window.newGameButton.setGeometry(750, 980, 235, 55)
    _newGameButtonBool = True
    windowBuilding.playerTurnEffect(window)