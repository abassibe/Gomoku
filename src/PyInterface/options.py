import windowBuilding

class Options():
    def __init__(self):
        self.langage = "English"
        self.gameMode = "PVE"
        self.rulesSet = ['Basic Rule', 'Capture', 'Double three', 'Game-ending capture']

    def _onAccept(self, window, dialog):
        self.langage = dialog.langageCombobox.currentText()
        if dialog.PVEButton.isChecked():
            self.gameMode = "PVE"
            self.rulesSet.append(dialog.ruleCheckbox1.text())
            self.rulesSet.append(dialog.ruleCheckbox2.text())
            self.rulesSet.append(dialog.ruleCheckbox3.text())
            self.rulesSet.append(dialog.ruleCheckbox4.text())
            windowBuilding.setRulesList(window, self.rulesSet)
            dialog.ruleCheckbox1.setEnabled(False)
            dialog.ruleCheckbox2.setEnabled(False)
            dialog.ruleCheckbox3.setEnabled(False)
            dialog.ruleCheckbox4.setEnabled(False)
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
