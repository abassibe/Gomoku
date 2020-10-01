import sys
import PyQt5
from PyQt5.QtGui import *
from PyQt5 import uic, QtWidgets
import windowBuilding
import buttonEventHandler
import options

app = PyQt5.QtWidgets.QApplication(sys.argv)
window = uic.loadUi("GUI/mainwindow.ui")
option = options.Options()

window.optionsButton.clicked.connect(lambda x: buttonEventHandler.optionsEvent(window, option))
window.hintButton.clicked.connect(lambda x: buttonEventHandler.hintEvent(window.hintButton))
window.giveUpButton.clicked.connect(lambda x: buttonEventHandler.giveUpEvent(window.giveUpButton))
window.newGameButton.clicked.connect(lambda x: buttonEventHandler.newGameEvent(window))
windowBuilding.setFontShadow(window)
windowBuilding.setRulesList(window, option.rulesSet)

window.show()
app.exec()
