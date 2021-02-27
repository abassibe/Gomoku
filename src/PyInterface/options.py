import windowBuilding

class Options():
    def __init__(self):
        self.langage = "English"
        self.gameMode = "PVE"
        self.rulesSet = ['Basic Rule']


    def _onAccept(self, window, dialog):
        self.langage = dialog.langageCombobox.currentText()
        if dialog.PVEButton.isChecked():
            self.gameMode = "PVE"
        else:
            self.gameMode = "PVP"
        windowBuilding.mainWindowTranslate(window, self)
        windowBuilding.dialogTranslate(dialog, self.langage)
        windowBuilding.howtoplayTranslate(self)
        self.rulesSet = []
        if dialog.ruleCheckbox1.isChecked():
            self.rulesSet.append(dialog.ruleCheckbox1.text())
        if dialog.ruleCheckbox2.isChecked():
            self.rulesSet.append(dialog.ruleCheckbox2.text())
        if dialog.ruleCheckbox3.isChecked():
            self.rulesSet.append(dialog.ruleCheckbox3.text())
        if dialog.ruleCheckbox4.isChecked():
            self.rulesSet.append(dialog.ruleCheckbox4.text())
        windowBuilding.setRulesList(window, self.rulesSet)
