#!/bin/bash
# Generate Russian voice clips using Milena voice

# 4 Random clicks phrases
say -v Milena -o click1.wav --file-format=WAVE --data-format=LEI16@44100 "Пепе"
say -v Milena -o click2.wav --file-format=WAVE --data-format=LEI16@44100 "Шнейне"
say -v Milena -o click3.wav --file-format=WAVE --data-format=LEI16@44100 "Втфа"
say -v Milena -o click4.wav --file-format=WAVE --data-format=LEI16@44100 "Фа"

# Specific sounds for Enter and Cmd
say -v Milena -o enter.wav --file-format=WAVE --data-format=LEI16@44100 "Энтер"
say -v Milena -o cmd.wav --file-format=WAVE --data-format=LEI16@44100 "Команд"

echo "Generated voice files from Gunwest phrases (simulated)."
