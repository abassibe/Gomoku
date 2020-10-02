from time import time
from PyQt5 import uic, QtWidgets, QtGui, QtCore
from random import randint
import options
import windowBuilding

_hintButtonBool = False
isGameRuning = False
player1TurnTime = 0.0
player2TurnTime = 0.0
isPlayer1Turn = True
globalTimer = None


def translateOptionDialog(dialog, option):
    if option.langage == "English" or option.langage == "Anglais":
        dialog.langageLabel.setText('Langage:')
        dialog.langageCombobox.setCurrentText("French")
        dialog.langageCombobox.setCurrentText("English")
        dialog.gameModeLabel.setText('Game mode:')
        dialog.PVEButton.setText('Player VS Engine')
        dialog.PVPButton.setText('Player VS Player')
        dialog.rulesSelectionLabel.setText('Rules set:')
        dialog.ruleCheckbox1.setText('Rule 1')
        dialog.ruleCheckbox2.setText('Rule 2')
        dialog.ruleCheckbox3.setText('Rule 3')
        dialog.ruleCheckbox4.setText('Rule 4')
    else:
        dialog.langageLabel.setText('Langue :')
        dialog.langageCombobox.setCurrentText("Français")
        dialog.langageCombobox.setCurrentText("Anglais")
        dialog.gameModeLabel.setText('Mode de jeu :')
        dialog.PVEButton.setText('Joueur VS Ordinateur')
        dialog.PVPButton.setText('Joueur VS Joueur')
        dialog.rulesSelectionLabel.setText('Set de règles :')
        dialog.ruleCheckbox1.setText('Règle 1')
        dialog.ruleCheckbox2.setText('Règle 2')
        dialog.ruleCheckbox3.setText('Règle 3')
        dialog.ruleCheckbox4.setText('Règle 4')


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
    dialog.langageCombobox.setCurrentText(option.langage)
    translateOptionDialog(dialog, option)
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


def newGameEvent(window):
    global isGameRuning
    global player1TurnTime
    global player2TurnTime
    global isPlayer1Turn
    global globalTimer
    if isGameRuning:
        return
    if randint(0, 1) == 0:
        isPlayer1Turn = True
    else:
        isPlayer1Turn = False
    if isPlayer1Turn:
        window.gameBoard.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/blackStone.png")))
    else:
        window.gameBoard.setCursor(QtGui.QCursor(QtGui.QPixmap("ressources/pictures/whiteStone.png")))
    isGameRuning = True
    player1TurnTime = 0.0
    player2TurnTime = 0.0

    globalTimer = QtCore.QTimer()
    startGameTimer = time()
    globalTimer.setInterval(10)
    globalTimer.timeout.connect(lambda: windowBuilding.updateTimerGame(window, globalTimer, startGameTimer))
    globalTimer.start()

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    QtCore.QTimer.singleShot(150, lambda: releaseNGButton(window, effect))
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(-7, -7)
    window.newGameButton.setGraphicsEffect(effect)
    window.newGameButton.setGeometry(750, 980, 235, 55)
    windowBuilding.playerTurnEffect(window)
