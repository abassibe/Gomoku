import windowBuilding

class Options():
    def __init__(self):
        self.langage = "English"
        self.gameMode = "PVE"
        self.rulesSet = ['']

    def _translate(self, window, dialog):
        if self.langage == "English" or self.langage == "Anglais":
            window.playersTitle.setText('Players infos')
            if self.gameMode == "PVE":
                window.playerOneLabel.setText('Player')
                window.playerTwoLabel.setText('Computer')
                window.playerTurnEffect.setText('Player turn')
            else:
                window.playerOneLabel.setText('Player 1')
                window.playerTwoLabel.setText('Player 2')
                window.playerTurnEffect.setText('Player 1 turn')
            window.rulesTitle.setText('Selected rules')
            window.hintButton.setText('Hint')
            window.giveUpButton.setText('Give Up')
            window.newGameButton.setText('New Game')
        else:
            window.playersTitle.setText('Infos joueurs')
            if self.gameMode == "PVE":
                window.playerOneLabel.setText('Joueur')
                window.playerTwoLabel.setText('Ordinateur')
                window.playerTurnEffect.setText('Tour joueur')
            else:
                window.playerOneLabel.setText('Joueur 1')
                window.playerTwoLabel.setText('Joueur 2')
                window.playerTurnEffect.setText('Tour joueur 1')
            window.rulesTitle.setText('Règles sélectionnée')
            window.hintButton.setText('Astuces')
            window.giveUpButton.setText('Abandon')
            window.newGameButton.setText('Nouvelle partie')


    def _onAccept(self, window, dialog):
        self.langage = dialog.langageCombobox.currentText()
        self._translate(window, dialog)
        if dialog.PVEButton.isChecked():
            self.gameMode = "PVE"
        else:
            self.gameMode = "PVP"
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
