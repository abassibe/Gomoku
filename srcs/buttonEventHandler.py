from PyQt5 import uic, QtWidgets, QtGui

hintButtonBool = False

def optionsEvent(optionsButton):
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(5, -5)
    optionsButton.setGraphicsEffect(effect)
    optionsButton.setGeometry(1129, 927, 51, 51)

    dialog = uic.loadUi("GUI/dialog.ui")
    dialog.show()
    dialog.exec()

    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(10, -10)
    optionsButton.setGraphicsEffect(effect)
    optionsButton.setGeometry(1129, 927, 61, 61)


def hintEvent(hintButton):
    global hintButtonBool
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    if not hintButtonBool:
        effect.setColor(QtGui.QColor(0, 0, 0, 120))
        effect.setOffset(-7, -7)
        hintButton.setGraphicsEffect(effect)
        hintButton.setGeometry(60, 980, 235, 55)
        hintButtonBool = True

    else:
        effect.setColor(QtGui.QColor(0, 0, 0, 90))
        effect.setOffset(-10, -10)
        hintButton.setGraphicsEffect(effect)
        hintButton.setGeometry(60, 980, 241, 61)
        hintButtonBool = False


def giveUpEvent(giveUpButton):
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(5, -5)
    giveUpButton.setGraphicsEffect(effect)
    giveUpButton.setGeometry(1129, 927, 51, 51)

    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(10, -10)
    giveUpButton.setGraphicsEffect(effect)
    giveUpButton.setGeometry(1129, 927, 61, 61)


def newGameEvent(newGameButton):
    effect = QtWidgets.QGraphicsDropShadowEffect()
    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 150))
    effect.setOffset(5, -5)
    newGameButton.setGraphicsEffect(effect)
    newGameButton.setGeometry(1129, 927, 51, 51)

    effect.setBlurRadius(0)
    effect.setColor(QtGui.QColor(0, 0, 0, 90))
    effect.setOffset(10, -10)
    newGameButton.setGraphicsEffect(effect)
    newGameButton.setGeometry(1129, 927, 61, 61)
