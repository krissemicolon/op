#!/bin/sh

PROJECTS_DIR="$HOME/Projects"
EDITOR="vim"

# Input
printf "Enter Project Name: "
read -r PROJECT_NAME

# Logic
tmux setenv PROJECTS_DIR $PROJECTS_DIR -t $PROJECT_NAME
tmux setenv PROJECT_NAME $PROJECT_NAME -t $PROJECT_NAME
tmux new-session -d -s $PROJECT_NAME 'cd $PROJECTS_DIR/$PROJECT_NAME; $EDITOR .'
# tmux split-window
# tmux send 'cd $PROJECTS_DIR/$PROJECT_NAME; vim .' ENTER
tmux a -t $PROJECT_NAME
