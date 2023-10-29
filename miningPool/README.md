
Let's think of some questions. What subproblems do we need to solve?
1. Communication between Miners and the Pool
2. Job Distribution. Distribute mining jobs to miners.
3. Miner Submission. Successfully mined block, send back to the mining pool.
4. Reward Distribution.



What data structures do we like?

1. BlockTemplate
  - block header: 
  - transaction list
  - coinbase transaction
  - Target Difficulty


What do we need to store?


luck window:  a specific timeframe during which a mining pool's performance, in terms of successfully mining blocks, is assessed




structure:
- api
- payouts
- proxy
- rpc
- storage
- stratum


## Miner

```
type Miner struct {
	lastBeat      int64 // last time see it's active
	startedAt     int64 // ?
	validShares   int64 // 
	invalidShares int64
	staleShares   int64
	accepts       int64 // 
	rejects       int64 // 
	shares        map[int64]int64
	sync.RWMutex
	id string
	ip string

	maxConcurrency int
}
```

```
type Job struct {
	height int64 
	sync.RWMutex // Lock for concurrency
	id          string 
	extraNonce  uint32
	submissions map[string]struct{}
}
```

## Stratum Server

