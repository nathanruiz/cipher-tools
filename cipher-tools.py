import itertools
import json

def achr(n):
    return chr((n % 26) + ord('A'))

def aord(c):
    return ord(c) - ord('A')

def add_trans(mapping, source, target):
    for i, char in enumerate(source):
        mapping[char] = target[i]

def substitute(mapping, chars):
    return "".join([mapping.get(n, '-') for n in chars])

def num2alpha(numbers):
    return " ".join([
        "".join([
            achr(int(n) - 1)
            for n in word.split("-")
        ])
        for word in numbers.split(" ")
    ])

def rot(n, chars):
    return "".join(achr(aord(c) + n) if c != " " else " " for c in chars)

def atbash(chars):
    return "".join(achr(25 - aord(c)) if c != " " else " " for c in chars)

def vigenere(key, chars, encrypt=False):
    i = 0
    result = ""
    for c in chars:
        if c == " ":
            result += " "
        else:
            key_c = aord(key[i % len(key)])
            if encrypt:
                result += achr(aord(c) + key_c)
            else:
                result += achr(aord(c) - key_c)
            i += 1

    return result

def bruteforce(length, fn):
    chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    for n in range(length):
        for key in itertools.permutations(chars, n + 1):
            if fn("".join(key)):
                return key

with open("dictionary.json") as f:
    WORDS = {word.upper() for word in json.load(f)}

def is_word(word):
    return word in WORDS

def test_achr():
    values = {
        0: "A",
        1: "B",
        2: "C",
        3: "D",
        4: "E",
        5: "F",
        6: "G",
        7: "H",
        8: "I",
        9: "J",
        10: "K",
        11: "L",
        12: "M",
        13: "N",
        14: "O",
        15: "P",
        16: "Q",
        17: "R",
        18: "S",
        19: "T",
        20: "U",
        21: "V",
        22: "W",
        23: "X",
        24: "Y",
        25: "Z",
    }
    for n, c in values.items():
        assert c == achr(n)
        assert n == aord(c)

def test_vigenere():
    assert vigenere("KEY", "TWOWORDS", encrypt=True) == "DAMGSPNW"
    assert vigenere("KEY", "TWO WORDS", encrypt=True) == "DAM GSPNW"
    assert vigenere("KEY", "DAMGSPNW") == "TWOWORDS"
    assert vigenere("KEY", "DAM GSPNW") == "TWO WORDS"

def test_num2alpha():
    assert num2alpha("1-2-3 24-25-26") == "ABC XYZ"

test_achr()
test_vigenere()
test_num2alpha()
