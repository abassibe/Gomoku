from time import time
from PyQt5 import uic, QtWidgets, QtGui, QtCore
import options
import windowBuilding
import gameManager
import numpy as np

_hintButtonBool = False
isGameRuning = False
globalTimer = None


def optionsEvent(window, option):
    if isGameRuning:
        return
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(-5, -5)
    window.optionsButton.setGraphicsEffect(effect)
    window.optionsButton.setGeometry(1129, 927, 51, 51)

    dialog = uic.loadUi("GUI/dialog.ui")
    windowBuilding.dialogTranslate(dialog, option.langage)
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


def releaseGUButton(window, effect):
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.giveUpButton.setGraphicsEffect(effect)
    window.giveUpButton.setGeometry(400, 980, 241, 61)


def giveUpEvent(window):
    global isGameRuning
    isGameRuning = False
    if globalTimer != None:
        globalTimer.stop()

    gameManager.player1TurnTime.stop()
    gameManager.player2TurnTime.stop()
    window.layoutWidget.unsetCursor()
    for point in gameManager.savedPlacedPoint:
        point.widget().clear()
    gameManager.grid = np.zeros(shape=(19, 19))
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    QtCore.QTimer.singleShot(150, lambda: releaseGUButton(window, effect))
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(-7, -7)
    window.giveUpButton.setGraphicsEffect(effect)
    window.giveUpButton.setGeometry(400, 980, 235, 55)


def releaseNGButton(window, effect):
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.newGameButton.setGraphicsEffect(effect)
    window.newGameButton.setGeometry(750, 980, 241, 61)


def newGameEvent(window, option):
    global isGameRuning
    global globalTimer
    global _hintButtonBool
    if isGameRuning:
        return
    isGameRuning = True

    globalTimer = QtCore.QTimer()
    window.playerTwoTimer.setText("00:00:00")
    window.playerOneTimer.setText("00:00:00")
    startGameTimer = time()
    globalTimer.setInterval(10)
    globalTimer.timeout.connect(lambda: windowBuilding.updateTimerGame(window, globalTimer, startGameTimer, window.gameTimer))
    globalTimer.start()

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    QtCore.QTimer.singleShot(150, lambda: releaseNGButton(window, effect))
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(-7, -7)
    window.newGameButton.setGraphicsEffect(effect)
    window.newGameButton.setGeometry(750, 980, 235, 55)
    gameManager.gameManager(window, option, _hintButtonBool)
