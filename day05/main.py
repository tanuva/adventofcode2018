import string

def readFile(name):
    with open(name, "r") as f:
        return f.read().rstrip("\n")

def findReactives(polymer):
    reactives = []

    for i in range(0, len(polymer) - 1):
        a = polymer[i]
        b = polymer[i + 1]
        if a != b and a.lower() == b.lower():
            if len(reactives) == 0 or abs(i - reactives[-1]) > 1:
                reactives.append(i)
                #print("found:", polymer[i:i+2])

    return reactives

def removeReactives(polymer, reactives):
    # Apply in reverse order to not mess up the indices for reactives later in
    # the string.
    for i in reversed(reactives):
        #print("l:", polymer[:i])
        #print("r:", polymer[i + 2:])
        polymer = polymer[:i] + polymer[i + 2:]
    return polymer

def react(polymer):
    #print("Reacting:", polymer[:100])

    while True:
        #print("poly:", polymer)
        reactives = findReactives(polymer)
        #print("reactives:", reactives)
        if len(reactives) == 0:
            break
        polymer = removeReactives(polymer, reactives)
        #print("---")

    return polymer

def shortenAggressively(polymer):
    shortest = 50000 # The original polymer is 50k chars, must be shorter than that

    for c in string.ascii_lowercase:
        cur = polymer
        cur = cur.replace(c, "").replace(c.upper(), "")
        reacted = react(cur)
        if len(reacted) < shortest:
            shortest = len(reacted)

    return shortest

def main():
    print("Reduced:", react(readFile("example")))
    print("Reduced:", len(react(readFile("input1"))))
    print("Shortest:", shortenAggressively(readFile("example")))
    print("Shortest:", shortenAggressively(readFile("input1")))

if __name__ == "__main__":
    main()
