# Overview

Rules define bags and what they can contain

    light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.

LR -> 1 BW, 2 MY
DO -> 3 BW, 4 MY
BW -> 1 SG
MY -> 2 SG, 9 FB
SG -> 1 DO, 2 VP
DO -> 3 FB, 4 DB,
VP -> 5 FB, 6 DB,
FB -> 
DB -> 

So each bag ultimately has to contain a finite number of other bags. After
building an initial lookup of the above datastructure, I should be able to
iterate over each bag type and recursively build a set of all of the other bag
types it contains.  I don't think I even need to build the first datastructure,
simply using the list of rules should be enough b/c N is small.

