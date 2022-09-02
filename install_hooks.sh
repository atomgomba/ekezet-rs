#!/bin/sh
mkdir -p ".git/hooks"
ln -s ../../tools/hooks/pre-commit .git/hooks/pre-commit
