# Azalea Pathfinder

Depended on by `azalea` for pathfinding.

## How it works

The pathfinder uses the [Moving Target D* Lite](http://idm-lab.org/bib/abstracts/papers/aamas10a.pdf) pathfinding algorithm to determine a path from a starting point to a goal area. MT-D* Lite has the advantage of being able to replan the path when anything changes very efficiently, which is very useful for following a moving target.


