import pathlib

from PyQt5 import QtWidgets, QtGui, QtCore
from time import time

import buttonEventHandler

mainWindow_en_EN = {}
dialog_en_EN = {}
howtoplay_en_EN = {}
mainWindow_fr_FR = {}
dialog_fr_FR = {}
howtoplay_fr_FR = {}
p1Turn = "Player turn"
p2Turn = ""
p1Win = "Black Win"
p2Win = "White Win"
draw = "Draw"

def setFontShadow(window):
    #fontDB = QtGui.QFontDatabase()
    window.playerTurnEffect.hide()
    window.playerTurnEffect.setStyleSheet("background-color:rgba(0, 0, 0, 0)")
    window.winOrDrawLabel.hide()
    window.winOrDrawLabel.setStyleSheet("background-color: rgba(255, 255, 255, 0);color:rgb(255, 255, 255);")

    window.layoutWidget.setStyleSheet("background-color: rgba(0, 0, 0, 0);")

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(209, 168, 101))
    effect.setOffset(3, -3)
    window.playersTitle.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(209, 168, 101))
    effect.setOffset(3, -3)
    window.rulesTitle.setGraphicsEffect(effect)
    
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.playerOneLabel.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.playerOneTimer.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.player1Capture.setGraphicsEffect(effect)
    window.player1Capture.hide()
    
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.playerTwoLabel.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.playerTwoTimer.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.player2Capture.setGraphicsEffect(effect)
    window.player2Capture.hide()

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.ruleLabel1.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.ruleLabel2.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(1)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.ruleLabel3.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.ruleLabel4.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.hintButton.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.giveUpButton.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.newGameButton.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 170))
    effect.setOffset(-30, -10)
    window.infosBackground1.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 170))
    effect.setOffset(-30, 41)
    window.infosBackground2.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.optionsButton.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 255))
    effect.setOffset(-5, -5)
    window.winOrDrawLabel.setGraphicsEffect(effect)
    
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 170))
    effect.setOffset(3, -3)
    window.gameTimer.setGraphicsEffect(effect)
    window.gameTimer.setGeometry(960, 330, 151, 41)
    window.gameTimer.setText("00:00")
    
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.turnsLabel.setGraphicsEffect(effect)
    
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.turnsValue.setGraphicsEffect(effect)


def setRulesList(window, ruleSet):
    window.ruleLabel1.setText('')
    window.ruleLabel2.setText('')
    window.ruleLabel3.setText('')
    window.ruleLabel4.setText('')
    isCaptureRule = False
    for i, rule in enumerate(ruleSet):
        if i == 0:
            window.ruleLabel1.setText(rule)
        elif i == 1:
            window.ruleLabel2.setText(rule)
        elif i == 2:
            window.ruleLabel3.setText(rule)
        elif i == 3:
            window.ruleLabel4.setText(rule)
        if rule == 'Capture':
            isCaptureRule = True
    if isCaptureRule:
        window.player1Capture.show()
        window.player2Capture.show()
    else:
        window.player1Capture.hide()
        window.player2Capture.hide()


def updateTimerGame(startGameTimer, toUpdate):
    miliSeconds = time() - startGameTimer
    minutes = int(miliSeconds / 60)
    seconds = int(miliSeconds - (minutes * 60))
    miliSeconds = (miliSeconds - int(miliSeconds)) * 100
    toUpdate.setText("%02d:%02d:%02d" % (minutes, seconds, miliSeconds))


def playerTurnEffect(window, playerTurn):
    global p1Turn
    global p2Turn

    if playerTurn == 1:
        window.playerTurnEffect.setText(p1Turn)
    else:
        window.playerTurnEffect.setText(p2Turn)
    newfont1 = QtGui.QFont("", 5)
    window.playerTurnEffect.setFont(newfont1)
    QtCore.QTimer.singleShot(50, lambda: window.playerTurnEffect.show())
    QtCore.QTimer.singleShot(50, lambda: window.playerTurnEffect.setGeometry(320, 0, 160, 30))
    newfont2 = QtGui.QFont("", 10)
    QtCore.QTimer.singleShot(100, lambda: window.playerTurnEffect.setFont(newfont2))
    QtCore.QTimer.singleShot(100, lambda: window.playerTurnEffect.setGeometry(295, 0, 216, 46))
    newfont3 = QtGui.QFont("", 15)
    QtCore.QTimer.singleShot(150, lambda: window.playerTurnEffect.setFont(newfont3))
    QtCore.QTimer.singleShot(150, lambda: window.playerTurnEffect.setGeometry(270, 0, 272, 62))
    newfont4 = QtGui.QFont("", 20)
    QtCore.QTimer.singleShot(200, lambda: window.playerTurnEffect.setFont(newfont4))
    QtCore.QTimer.singleShot(200, lambda: window.playerTurnEffect.setGeometry(245, 0, 328, 78))
    newfont5 = QtGui.QFont("", 35)
    QtCore.QTimer.singleShot(250, lambda: window.playerTurnEffect.setFont(newfont5))
    QtCore.QTimer.singleShot(250, lambda: window.playerTurnEffect.setGeometry(220, 0, 384, 94))
    newfont5 = QtGui.QFont("", 40)
    QtCore.QTimer.singleShot(300, lambda: window.playerTurnEffect.setFont(newfont5))
    QtCore.QTimer.singleShot(300, lambda: window.playerTurnEffect.setGeometry(190, 0, 440, 110))
    QtCore.QTimer.singleShot(1000, lambda: window.playerTurnEffect.hide())


def winDraw(window, isWin, player):
    if isWin == 0:
        window.winOrDrawLabel.setText(draw)
    elif player == 1:
        window.winOrDrawLabel.setText(p1Win)
    else:
        window.winOrDrawLabel.setText(p2Win)
    newfont1 = QtGui.QFont("", 12)
    window.winOrDrawLabel.setFont(newfont1)
    QtCore.QTimer.singleShot(50, lambda: window.winOrDrawLabel.show())
    QtCore.QTimer.singleShot(50, lambda: window.winOrDrawLabel.setGeometry(290, 320, 250, 100))
    newfont2 = QtGui.QFont("", 24)
    QtCore.QTimer.singleShot(100, lambda: window.winOrDrawLabel.setFont(newfont2))
    QtCore.QTimer.singleShot(100, lambda: window.winOrDrawLabel.setGeometry(258, 280, 313, 187))
    newfont3 = QtGui.QFont("", 36)
    QtCore.QTimer.singleShot(150, lambda: window.winOrDrawLabel.setFont(newfont3))
    QtCore.QTimer.singleShot(150, lambda: window.winOrDrawLabel.setGeometry(226, 240, 376, 274))
    newfont4 = QtGui.QFont("", 52)
    QtCore.QTimer.singleShot(200, lambda: window.winOrDrawLabel.setFont(newfont4))
    QtCore.QTimer.singleShot(200, lambda: window.winOrDrawLabel.setGeometry(194, 200, 439, 361))
    newfont5 = QtGui.QFont("", 60)
    QtCore.QTimer.singleShot(250, lambda: window.winOrDrawLabel.setFont(newfont5))
    QtCore.QTimer.singleShot(250, lambda: window.winOrDrawLabel.setGeometry(162, 160, 502, 448))
    newfont5 = QtGui.QFont("", 72)
    QtCore.QTimer.singleShot(300, lambda: window.winOrDrawLabel.setFont(newfont5))
    QtCore.QTimer.singleShot(300, lambda: window.winOrDrawLabel.setGeometry(130, 120, 565, 535))
    newfont5 = QtGui.QFont("", 72)
    QtCore.QTimer.singleShot(300, lambda: window.winOrDrawLabel.setFont(newfont5))
    QtCore.QTimer.singleShot(300, lambda: window.winOrDrawLabel.setGeometry(95, 85, 630, 624))


def parseTranslationFile():
    f = open(str(pathlib.Path("local/en_EN")))
    tmp = []
    toFill = {}
    for line in f:
        line = line[:-1]
        if line == "#mainwindow.ui":
            toFill = mainWindow_en_EN
            continue        
        elif line == "#dialog.ui":
            toFill = dialog_en_EN
            continue
        elif line == "#howtoplay.ui":
            toFill = howtoplay_en_EN
            continue
        splited = line.split('=')
        for arg in splited[1:]:
            tmp.append(arg)
        toFill[splited[0]] = tmp
        tmp = []
    f = open(str(pathlib.Path("local/fr_FR")))
    tmp = []
    toFill = {}
    for line in f:
        line = line.replace("\\n", "\n")
        line = line[:-1]
        if line == "#mainwindow.ui":
            toFill = mainWindow_fr_FR
            continue        
        elif line == "#dialog.ui":
            toFill = dialog_fr_FR
            continue
        elif line == "#howtoplay.ui":
            toFill = howtoplay_fr_FR
            continue
        splited = line.split('=')
        for arg in splited[1:]:
            tmp.append(arg)
        toFill[splited[0]] = tmp
        tmp = []


def howtoplayTranslate(option):
    window = buttonEventHandler.helpDialog

    translationSet = {}
    if option.langage == "English" or option.langage == "Anglais":
        translationSet = howtoplay_en_EN
    else:
        translationSet = howtoplay_fr_FR
    for key, value in translationSet.items():
        try:
            window.__getattribute__(key).setText(value[0])
        except:
            exit("Translation error")


def mainWindowTranslate(window, option):
    global p1Turn
    global p2Turn
    global p1Win
    global p2Win
    global draw

    p1Turn = ""
    p2Turn = ""
    translationSet = {}
    if option.langage == "English" or option.langage == "Anglais":
        translationSet = mainWindow_en_EN
    else:
        translationSet = mainWindow_fr_FR
    for key, value in translationSet.items():
        try:
            toTranslate = window.__getattribute__(key)
            if ";" in value[0]:
                tmp = value[0].split(";")
                if len(tmp) == 2:
                    if option.gameMode == "PVE":
                        toTranslate.setText(tmp[0])
                    else:
                        toTranslate.setText(tmp[1])
                if key == "playerTurnEffect" and option.gameMode == "PVP":
                    p1Turn = tmp[1]
                    p2Turn = tmp[2]
                elif key == "playerTurnEffect":
                    p1Turn = tmp[0]
                elif key == "winOrDrawLabel":
                    p1Win = tmp[0]
                    p2Win = tmp[1]
                    draw = tmp[2]
            else:
                toTranslate.setText(value[0])
        except:
            exit("Translation error")


def dialogTranslate(dialog, actualLangage):
    translationSet = {}
    if actualLangage == "English" or actualLangage == "Anglais":
        translationSet = dialog_en_EN
    else:
        translationSet = dialog_fr_FR
    for key, value in translationSet.items():
        try:
            toTranslate = dialog.__getattribute__(key)
            if "Combobox" in key:
                langage = value[0].split(";")
                toTranslate.setItemText(0, langage[0])
                toTranslate.setItemText(1, langage[1])
                if actualLangage == "English" or actualLangage == "Anglais":
                    toTranslate.setCurrentIndex(0)
                    dialog.buttonBox.button(QtWidgets.QDialogButtonBox.Cancel).setText("Cancel")
                else:
                    toTranslate.setCurrentIndex(1)
                    dialog.buttonBox.button(QtWidgets.QDialogButtonBox.Cancel).setText("Annuler")
            else:
                toTranslate.setText(value[0])
        except:
            exit("Translation error")
