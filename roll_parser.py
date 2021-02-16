"""
Take in a string, determine if it is valid, then compute the dice roll result
TODO: Check if parentheses are extraneous, then find the place to split at
"""
import re
from collections import deque
from random import randrange

OPERATOR_PRECEDENCE = {
    '+': 200,
    '-': 200,
    '*': 100,
    '/': 100
}


# Check to make sure parentheses in s are properly opening/closing
def parse_parens(s):
    stack = deque()
    for ch in s:
        if ch == ')':
            if len(stack) == 0:
                return False
            else:
                stack.pop()
        elif ch == '(':
            stack.append(ch)
    return len(stack) == 0


# Check to make sure the provided string is valid
def check_match(s):
    # Pattern matching valid dice roll commands
    pattern = r"^\(*(?:[0-9]+|[0-9]*(?:d[1-9][0-9]*))\)*(?:(?:\+|\*|-|/)\(*(?:[0-9]+|[0-9]*(?:d[1-9][0-9]*))\)*)*$"
    return bool(re.match(pattern, s))


# remove extraneous parens and create map from opening paren index to close paren index
def create_parens_map(s):
    d = deque()
    openings_to_closings = {}
    for i, ch in enumerate(s):
        if ch == '(':
            d.append(i)
        elif ch == ')':
            openings_to_closings[d.pop()] = i
    while openings_to_closings.get(0) == len(s) - 1:
        openings_to_closings.pop(0)
        openings_to_closings = {(k-1): (v-1)
                                for k, v in openings_to_closings.items()}
        s = s[1:-1]

    return s, openings_to_closings


# Find index to split expression at
def find_split_ind(s):
    s, openings_to_closings = create_parens_map(s)
    if openings_to_closings != {}:
        last_close_paren_ind = -1
        max_precedence_ind = -1
        for k, v in sorted(openings_to_closings.items()):
            # make sure that paren isn't enclosed in parens...
            if k > last_close_paren_ind:
                last_close_paren_ind = v
                if k != 0:
                    if OPERATOR_PRECEDENCE.get(s[k-1], 0) >= OPERATOR_PRECEDENCE.get(s[max_precedence_ind], 0):
                        max_precedence_ind = k-1
                if v != len(s) - 1:
                    if OPERATOR_PRECEDENCE.get(s[v+1], 0) >= OPERATOR_PRECEDENCE.get(s[max_precedence_ind], 0):
                        max_precedence_ind = v+1
        return s, max_precedence_ind
    else:
        max_precedence_ind = 0
        for i, ch in enumerate(s):
            if OPERATOR_PRECEDENCE.get(ch, 0) >= OPERATOR_PRECEDENCE.get(s[max_precedence_ind], 0):
                max_precedence_ind = i
        return s, max_precedence_ind


# Recursive method to parse the expression
def parse(s):
    s, split_ind = find_split_ind(s)
    # No split to do...
    if split_ind == len(s) - 1:
        if s[0] == 'd':
            return randrange(int(s[1:])) + 1
        elif 'd' in s:
            split = s.split('d')
            return sum([randrange(int(split[1])) + 1 for i in range(int(split[0]))])
        else:
            return int(s)
    else:
        operator = s[split_ind]
        lsplit = s[:split_ind]
        rsplit = s[split_ind+1:]
        # I should be using a language with pattern matching...
        if operator == '+':
            return parse(lsplit) + parse(rsplit)
        elif operator == '-':
            return parse(lsplit) - parse(rsplit)
        elif operator == '*':
            return parse(lsplit) * parse(rsplit)
        elif operator == '/':
            return parse(lsplit) / parse(rsplit)
        else:
            return "I don't pattern match incorrectly."


def compute(s):
    s = s.replace(" ", "")
    if not parse_parens(s):
        return "Parentheses inbalance detected"
    if not check_match(s):
        return "Not a valid roll expression"
    else:
        return parse(s)
