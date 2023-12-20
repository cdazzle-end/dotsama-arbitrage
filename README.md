# Install
Need yarn v3+
install ts-node - yarn global add ts-node



# arb
Arb-handler is written in rust. It builds a graph from the data retrieved through /lps/ and /assets/
Builds a local registry of all assets, and correlates/tracks them through their asset multilocation.
Build a local registry of all liquidity pools, using the asset registry for the liquidity pool assets.
Build an 'adjacency' hash table from the liquidity pool registry. Every asset has an entry in the table. Each entry is a list of assets that are adjacent to the entry asset.
    Adjacency means that there is an immediate path from one asset to the other, because they share a liquidity pool.
Build a token graph from the adjacency table. Each asset is a node. Nodes are connected to the nodes of other assets that they are adjacent to.
    These connections are represented in each node through a list of pointers to other nodes
    These connections also contain relevant data like liqudity numbers
    Assets with the same multilocation also have their nodes connected, tracking cross chain assets
Algorithm to find arbitrage oppurtunities. very simplified rn. Could be much better
    Picks one asset and amount to start. Pathfinding algo to find a path to another node of the same asset multilocation. 
    Finds path that results in the largest token amount upon completion.
    Path values are calculated as the graph is traversed, according to the DEX algorithm used in the pool between the 2 assets, using the specified input at the beginning
    Traverses all possible routes that could possibly end in larger token A amount at somepoint in the graph
    token A -> token B -> token B on dif chain -> token c -> token c on dif chain -> token A. started with 1.00 token A, ends with 1.05 token A
    
For now, see search_ksm() function in lib.rs to examine execution flow


# assets

The purpose of assets is to be able to  build and update a registry of all the assets registered across the various parachains

# lps

Lps scripts query the chains to find out the current liquidity for all registered assets