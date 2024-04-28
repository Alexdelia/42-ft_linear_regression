# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: adelille <adelille@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2020/11/30 19:21:49 by adelille          #+#    #+#              #
#    Updated: 2024/04/28 18:00:47 by adelille         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME_TRAIN =	train
NAME_PREDICT =	predict
CC =			cargo
RM = 			rm -rf

CCFLAGS = --release

TARGET_DIR = target/release/

# **************************************************************************** #
#	MAKEFILE	#

MAKEFLAGS += --silent

SHELL := bash

# *************************************************************************** #
#	RULES	#

all:		$(NAME_TRAIN) $(NAME_PREDICT)

$(NAME_TRAIN):		build
	@cp -f $(TARGET_DIR)$(NAME_TRAIN) . || true

$(NAME_PREDICT):	build
	@cp -f $(TARGET_DIR)$(NAME_PREDICT) . || true

build:
	$(CC) build $(CCFLAGS)

check:
	$(CC) check
	
clean:
	$(CC) clean

fclean:		clean
	@$(RM) $(NAME_TRAIN) $(NAME_PREDICT)

re:			fclean all

.PHONY: all clean fclean re build check

# **************************************************************************** #
