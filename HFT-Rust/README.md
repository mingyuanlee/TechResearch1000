Three main operations:
(1) add O(1)
(2) cancel O(1)
(3) execute O(1)

"What are the best bid and offer?"
"How much volume is there between prices A and B?"
"What is order X's current position in the book?"

Volume: the number of shares
Sparsity: average distance in cents between limits that have volume
Slippage: the difference between the expected price of a trade and the actual executed price.

HFT system depends on high volume to work efficiently. Market-making. What's market-making? 


Thoughts:
Can we make it a distributed orderbook using Kademlia?