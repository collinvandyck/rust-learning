# Readme

Example blueprint:

    Blueprint 1:
      Each ore robot costs 4 ore.
      Each clay robot costs 2 ore.
      Each obsidian robot costs 3 ore and 14 clay.
      Each geode robot costs 2 ore and 7 obsidian.

Each move we make should be targeted towards the solution, which is to maximize
the amount of geodes collected. This means we should prioritize the fastest path
to creating geode robots, because each minute, the existing robots will produce
one resource they are designed to mine.

For the above blueprint, we can map out the costs:

    Geode Robot
        2 ore
            option 1:
                do nothing. wait a turn for another ore to be produced.
                cost = 1
                result = 2 ore.
            option 2:
                wait 3 turns. this produces 4 ore.
                then build another ore robot.
                then we have two robots.
                wait a turn.
                this produces 2 ore.

        7 obsidian
            3 ore -> 21 ore
                1 ore robot for 2 moves, OR
                2 ore robots for 1 move
            14 clay
                2 ore -> 28 ore
                    1 ore robot for 2 moves, OR
                    2 ore robots for 1 move

Mapping out the possibilities seems a little mind bending.
We could brute force it.

Each state has a number of other states that could follow from it. We would do a
depth or breadth first approach to see what the highest number of obsidian is.
Each state could maintain its own set of moves that produce a final result. Will
probably go with that to start and see how it does.
