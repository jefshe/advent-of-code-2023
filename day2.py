games = [
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
]

import re;

colorConstraint = {
    "red": 12,
    "green": 13, 
    "blue": 14
}

def fulfillsConstraint(gameSet: dict[str, int]):
    return all([colorConstraint[color] >= gameSet[color] for color in gameSet.keys()])

def parsePair(numAndcolor: str):
    pair = numAndcolor.split(" ")
    return (pair[1], int(pair[0]))

def parseSet(setStr: str):
    colors = re.split("\s*,\s*", setStr)
    colorSet = dict([parsePair(c) for c in colors])
    return colorSet

def parseGame(gameStr: str):
    gameSets = gameStr.split(": ")[-1]
    sets = re.split("\s*;\s*", gameSets)
    return [parseSet(s) for s in sets]

games = [parseGame(line) for line in games]

possibleGames = [i+1 for i, game in enumerate(games) if all([fulfillsConstraint(set) for set in game])]
print(sum(possibleGames))
 