#!/usr/bin/env bash
set -e
git init
git branch -M main
git remote add origin https://github.com/riscvthai/TranTheHao-MipsVisualizedNewTechnologyLowlevelRISC-RUST.git 2>/dev/null || true
git remote set-url origin https://github.com/riscvthai/TranTheHao-MipsVisualizedNewTechnologyLowlevelRISC-RUST.git
git add .
git commit -m "Add fixed full visual system model"
git push -u origin main
