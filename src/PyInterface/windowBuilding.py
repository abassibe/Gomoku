import pathlib
from time import time
from PyQt5 import QtWidgets, QtGui, QtCore
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


def setRulesList(window, rule_set):
    window.ruleLabel1.setText('')
    window.ruleLabel2.setText('')
    window.ruleLabel3.setText('')
    window.ruleLabel4.setText('')
    is_capture_rule = False
    for i, rule in enumerate(rule_set):
        if i == 0:
            window.ruleLabel1.setText(rule)
        elif i == 1:
            window.ruleLabel2.setText(rule)
        elif i == 2:
            window.ruleLabel3.setText(rule)
        elif i == 3:
            window.ruleLabel4.setText(rule)
        if rule == 'Capture':
            is_capture_rule = True
    if is_capture_rule:
        window.player1Capture.show()
        window.player2Capture.show()
    else:
        window.player1Capture.hide()
        window.player2Capture.hide()


def updateTimerGame(start_game_timer, to_update):
    milli_seconds = time() - start_game_timer
    minutes = int(milli_seconds / 60)
    seconds = int(milli_seconds - (minutes * 60))
    milli_seconds = (milli_seconds - int(milli_seconds)) * 100
    to_update.setText("%02d:%02d:%02d" % (minutes, seconds, milli_seconds))


def playerTurnEffect(window, player_turn):
    global p1Turn
    global p2Turn

    if player_turn == 1:
        window.playerTurnEffect.setText(p1Turn)
    else:
        window.playerTurnEffect.setText(p2Turn)
    new_font1 = QtGui.QFont("", 5)
    window.playerTurnEffect.setFont(new_font1)
    QtCore.QTimer.singleShot(50, lambda: window.playerTurnEffect.show())
    QtCore.QTimer.singleShot(50, lambda: window.playerTurnEffect.setGeometry(320, 0, 160, 30))
    new_font2 = QtGui.QFont("", 10)
    QtCore.QTimer.singleShot(100, lambda: window.playerTurnEffect.setFont(new_font2))
    QtCore.QTimer.singleShot(100, lambda: window.playerTurnEffect.setGeometry(295, 0, 216, 46))
    new_font3 = QtGui.QFont("", 15)
    QtCore.QTimer.singleShot(150, lambda: window.playerTurnEffect.setFont(new_font3))
    QtCore.QTimer.singleShot(150, lambda: window.playerTurnEffect.setGeometry(270, 0, 272, 62))
    new_font4 = QtGui.QFont("", 20)
    QtCore.QTimer.singleShot(200, lambda: window.playerTurnEffect.setFont(new_font4))
    QtCore.QTimer.singleShot(200, lambda: window.playerTurnEffect.setGeometry(245, 0, 328, 78))
    new_font5 = QtGui.QFont("", 35)
    QtCore.QTimer.singleShot(250, lambda: window.playerTurnEffect.setFont(new_font5))
    QtCore.QTimer.singleShot(250, lambda: window.playerTurnEffect.setGeometry(220, 0, 384, 94))
    new_font5 = QtGui.QFont("", 40)
    QtCore.QTimer.singleShot(300, lambda: window.playerTurnEffect.setFont(new_font5))
    QtCore.QTimer.singleShot(300, lambda: window.playerTurnEffect.setGeometry(190, 0, 440, 110))
    QtCore.QTimer.singleShot(1000, lambda: window.playerTurnEffect.hide())


def winDraw(window, is_win, player):
    if is_win == 0:
        window.winOrDrawLabel.setText(draw)
    elif player == 1:
        window.winOrDrawLabel.setText(p1Win)
    else:
        window.winOrDrawLabel.setText(p2Win)
    new_font1 = QtGui.QFont("", 12)
    window.winOrDrawLabel.setFont(new_font1)
    QtCore.QTimer.singleShot(50, lambda: window.winOrDrawLabel.show())
    QtCore.QTimer.singleShot(50, lambda: window.winOrDrawLabel.setGeometry(290, 320, 250, 100))
    new_font2 = QtGui.QFont("", 24)
    QtCore.QTimer.singleShot(100, lambda: window.winOrDrawLabel.setFont(new_font2))
    QtCore.QTimer.singleShot(100, lambda: window.winOrDrawLabel.setGeometry(258, 280, 313, 187))
    new_font3 = QtGui.QFont("", 36)
    QtCore.QTimer.singleShot(150, lambda: window.winOrDrawLabel.setFont(new_font3))
    QtCore.QTimer.singleShot(150, lambda: window.winOrDrawLabel.setGeometry(226, 240, 376, 274))
    new_font4 = QtGui.QFont("", 52)
    QtCore.QTimer.singleShot(200, lambda: window.winOrDrawLabel.setFont(new_font4))
    QtCore.QTimer.singleShot(200, lambda: window.winOrDrawLabel.setGeometry(194, 200, 439, 361))
    new_font5 = QtGui.QFont("", 60)
    QtCore.QTimer.singleShot(250, lambda: window.winOrDrawLabel.setFont(new_font5))
    QtCore.QTimer.singleShot(250, lambda: window.winOrDrawLabel.setGeometry(162, 160, 502, 448))
    new_font5 = QtGui.QFont("", 72)
    QtCore.QTimer.singleShot(300, lambda: window.winOrDrawLabel.setFont(new_font5))
    QtCore.QTimer.singleShot(300, lambda: window.winOrDrawLabel.setGeometry(130, 120, 565, 535))
    new_font5 = QtGui.QFont("", 72)
    QtCore.QTimer.singleShot(300, lambda: window.winOrDrawLabel.setFont(new_font5))
    QtCore.QTimer.singleShot(300, lambda: window.winOrDrawLabel.setGeometry(95, 85, 630, 624))


def parseTranslationFile():
    f = open(str(pathlib.Path("local/en_EN")))
    tmp = []
    to_fill = {}
    for line in f:
        line = line[:-1]
        if line == "#mainwindow.ui":
            to_fill = mainWindow_en_EN
            continue
        elif line == "#dialog.ui":
            to_fill = dialog_en_EN
            continue
        elif line == "#howtoplay.ui":
            to_fill = howtoplay_en_EN
            continue
        splited = line.split('=')
        for arg in splited[1:]:
            tmp.append(arg)
        to_fill[splited[0]] = tmp
        tmp = []
    f.close()
    f = open(str(pathlib.Path("local/fr_FR")))
    tmp = []
    to_fill = {}
    for line in f:
        line = line.replace("\\n", "\n")
        line = line[:-1]
        if line == "#mainwindow.ui":
            to_fill = mainWindow_fr_FR
            continue
        elif line == "#dialog.ui":
            to_fill = dialog_fr_FR
            continue
        elif line == "#howtoplay.ui":
            to_fill = howtoplay_fr_FR
            continue
        splited = line.split('=')
        for arg in splited[1:]:
            tmp.append(arg)
        to_fill[splited[0]] = tmp
        tmp = []
    f.close()


def howtoplayTranslate(option):
    window = buttonEventHandler.helpDialog

    translation_set = {}
    if option.langage == "English" or option.langage == "Anglais":
        translation_set = howtoplay_en_EN
    else:
        translation_set = howtoplay_fr_FR
    for key, value in translation_set.items():
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
    translation_set = {}
    if option.langage == "English" or option.langage == "Anglais":
        translation_set = mainWindow_en_EN
    else:
        translation_set = mainWindow_fr_FR
    for key, value in translation_set.items():
        try:
            to_translate = window.__getattribute__(key)
            if ";" in value[0]:
                tmp = value[0].split(";")
                if len(tmp) == 2:
                    if option.gameMode == "PVE":
                        to_translate.setText(tmp[0])
                    else:
                        to_translate.setText(tmp[1])
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
                to_translate.setText(value[0])
        except:
            exit("Translation error")


def dialogTranslate(dialog, language):
    translation_set = {}
    if language == "English" or language == "Anglais":
        translation_set = dialog_en_EN
    else:
        translation_set = dialog_fr_FR
    for key, value in translation_set.items():
        try:
            to_translate = dialog.__getattribute__(key)
            if "Combobox" in key:
                language = value[0].split(";")
                to_translate.setItemText(0, language[0])
                to_translate.setItemText(1, language[1])
                if language == "English" or language == "Anglais":
                    to_translate.setCurrentIndex(0)
                    dialog.buttonBox.button(QtWidgets.QDialogButtonBox.Cancel).setText("Cancel")
                else:
                    to_translate.setCurrentIndex(1)
                    dialog.buttonBox.button(QtWidgets.QDialogButtonBox.Cancel).setText("Annuler")
            else:
                to_translate.setText(value[0])
        except:
            exit("Translation error")
