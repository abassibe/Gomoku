from time import time
from PyQt5 import QtWidgets, QtGui, QtCore


def setFontShadow(window):
    # window.playerTurnEffect.setVisible(False)

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
    window.ruleLabel1.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(60, 17, 3))
    effect.setOffset(3, -3)
    window.ruleLabel2.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
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
    effect.setOffset(-20, -10)
    window.infosBackground1.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 170))
    effect.setOffset(-20, 54)
    window.infosBackground2.setGraphicsEffect(effect)

    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(-10, -10)
    window.optionsButton.setGraphicsEffect(effect)
    
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 170))
    effect.setOffset(3, -3)
    window.gameTimer.setGraphicsEffect(effect)
    window.gameTimer.setGeometry(1270, 440, 151, 41)
    window.gameTimer.setText("00:00")


def setRulesList(window, ruleSet):
    window.ruleLabel1.setText('')
    window.ruleLabel2.setText('')
    window.ruleLabel3.setText('')
    window.ruleLabel4.setText('')
    for i, rule in enumerate(ruleSet):
        if i == 0:
            window.ruleLabel1.setText(rule)
        if i == 1:
            window.ruleLabel2.setText(rule)
        if i == 2:
            window.ruleLabel3.setText(rule)
        if i == 3:
            window.ruleLabel4.setText(rule)


def updateTimerGame(window, timer, startGameTimer):
    miliSeconds = time() - startGameTimer
    minutes = int(miliSeconds / 60)
    seconds = int(miliSeconds)
    miliSeconds = (miliSeconds - seconds) * 100
    window.gameTimer.setText("%02d:%02d:%02d" % (minutes, seconds, miliSeconds))


def playerTurnEffect(window):
    # window.playerTurnEffect.setVisible(True)
    xIncrease = int((460 - 260) / 5)
    yIncrease = int((490 - 440) / 5)
    widthIncrease = int((541 - 141) / 5)
    heightIncrease = int((151 - 51) / 5)
    i = 1
    x = 460
    y = 490
    width = 141
    height = 51
    newfont = QtGui.QFont("SF Wasabi", 20) 
    QtCore.QTimer.singleShot(i * 100, lambda: window.playerTurnEffect.setFont(newfont))
    QtCore.QTimer.singleShot(i * 100, lambda: window.playerTurnEffect.setGeometry(460, 490, 141, 51))
    newfont2 = QtGui.QFont("SF Wasabi", 32) 
    QtCore.QTimer.singleShot(i * 100, lambda: window.playerTurnEffect.setFont(newfont2))
    QtCore.QTimer.singleShot(i * 200, lambda: window.playerTurnEffect.setGeometry(420, 480, 221, 71))
    newfont3 = QtGui.QFont("SF Wasabi", 44) 
    QtCore.QTimer.singleShot(i * 100, lambda: window.playerTurnEffect.setFont(newfont3))
    QtCore.QTimer.singleShot(i * 300, lambda: window.playerTurnEffect.setGeometry(380, 470, 301, 91))
    newfont4 = QtGui.QFont("SF Wasabi", 56) 
    QtCore.QTimer.singleShot(i * 100, lambda: window.playerTurnEffect.setFont(newfont4))
    QtCore.QTimer.singleShot(i * 400, lambda: window.playerTurnEffect.setGeometry(340, 460, 381, 111))
    newfont5 = QtGui.QFont("SF Wasabi", 68) 
    QtCore.QTimer.singleShot(i * 100, lambda: window.playerTurnEffect.setFont(newfont5))
    QtCore.QTimer.singleShot(i * 500, lambda: window.playerTurnEffect.setGeometry(300, 450, 461, 131))
    newfont5 = QtGui.QFont("SF Wasabi", 80) 
    QtCore.QTimer.singleShot(i * 100, lambda: window.playerTurnEffect.setFont(newfont5))
    QtCore.QTimer.singleShot(i * 500, lambda: window.playerTurnEffect.setGeometry(260, 440, 541, 151))
    # while i < 6:
    #     QtCore.QTimer.singleShot(i * 200, lambda: window.playerTurnEffect.setGeometry(x, y, width, height))
    #     print("x = " + str(x) + " y = " + str(y) + " width = " + str(width) + " height = " + str(height))
    #     x -= xIncrease
    #     y -= yIncrease
    #     width += widthIncrease
    #     height += heightIncrease
    #     i += 1
    # window.playerTurnEffect.setVisible(False)
