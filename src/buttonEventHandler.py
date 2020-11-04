import pathlib
from time import time
from PyQt5 import uic, QtWidgets, QtGui, QtCore
import options
import windowBuilding
import gameManager
import numpy as np


_hintButtonBool = False


def optionsEvent(window, option):
    if window.gameManager != None and window.gameManager.gameRuning:
        return
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(-5, -5)
    window.optionsButton.setGraphicsEffect(effect)
    window.optionsButton.setGeometry(1129, 927, 51, 51)

    dialog = uic.loadUi(str(pathlib.Path("GUI/dialog.ui")))
    dialog.ruleCheckbox1.setEnabled(False)
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


def hintEvent(hintButton, window):
    global _hintButtonBool
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    if not _hintButtonBool:
        effect.setColor(QtGui.QColor(0, 0, 0, 120))
        effect.setOffset(-7, -7)
        hintButton.setGraphicsEffect(effect)
        hintButton.setGeometry(60, 980, 235, 55)
        _hintButtonBool = True
        color = None
        if window.gameManager and window.gameManager.gameRuning == True:
            if window.gameManager.isPlayer1Turn:
                color = window.gameManager.player1.color
            else:
                color = window.gameManager.player2.color
            x, y = window.algoPointer(window.gameManager.gameBoard.grid, color, True)
            window.gameManager.gameBoard.dropHint(x, y, color)
    else:
        effect.setColor(QtGui.QColor(0, 0, 0, 90))
        effect.setOffset(-10, -10)
        hintButton.setGraphicsEffect(effect)
        hintButton.setGeometry(60, 980, 241, 61)
        _hintButtonBool = False
        if window.gameManager:
            window.gameManager.gameBoard.clearHint()
    if window.gameManager != None:
        window.gameManager.hintButtonBool = _hintButtonBool


def releaseGUButton(window, effect):
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.giveUpButton.setGraphicsEffect(effect)
    window.giveUpButton.setGeometry(400, 980, 241, 61)


def giveUpEvent(window):
    if window.gameManager == None or window.gameManager.gameRuning == False:
        return

    window.layoutWidget.unsetCursor()
    window.gameManager.end()
    window.gameManager.gameBoard.clearHint()
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
    global _hintButtonBool
    if window.gameManager != None and window.gameManager.gameRuning == True:
        return

    if window.gameManager:
        window.gameManager.gameBoard.clear()
    window.gameManager = gameManager.GameManager(window, option, _hintButtonBool)
    window.winOrDrawLabel.hide()

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    QtCore.QTimer.singleShot(150, lambda: releaseNGButton(window, effect))
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(-7, -7)
    window.newGameButton.setGraphicsEffect(effect)
    window.newGameButton.setGeometry(750, 980, 235, 55)
    window.gameManager.start()
