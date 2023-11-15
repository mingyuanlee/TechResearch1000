When a user views their timeline, it’s too expensive to iterate over all
the people they are following to get those users’ tweets. Instead,
Twitter must compute a user’s timeline ahead of time, and cache it
so that it’s fast to read when a user looks at it. To do that, the system
needs a process that translates from the write-optimized event (a
single tweet) to the read-optimized aggregate (a timeline).

the idea of event sourcing; that is, representing the changes to a database as a log of immutable events. 

From the Twitter and Facebook examples we can see a certain pat‐
tern: the input events, corresponding to the buttons in the user
interface, are quite simple. They are immutable facts, we can simply
store them all, and we can treat them as the source of truth

The database you read from is just a cached
view of the event log.

event streams

Contemporary distributed stream processing frameworks (Kafka
Streams, Samza, Storm, Spark Streaming, Flink) are mostly con‐
cerned with low-level matters: how to scale processing across multi‐
ple machines, how to deploy a job to a cluster, how to handle faults
(crashes, machine failures, network outages), and how to achieve
reliable performance in a multitenant environment.

a stream should be implemented as a log;  that is, an append-only
sequence of events in a fixed order.

we must assume
that with dual writes the application needs to deal with partial fail‐
ure, which is difficult.

if you
write down the order in which you make your writes, it becomes
much easier to recover from partial failures

append-only ordered sequence

Replication is a feature that you find in many databases: it allows
you to keep a copy of the same data on several different nodes. That
can be useful for spreading the load, and it also means that if one
node dies, you can failover to another one.

you might require all your database nodes to agree on
which node is the leader for a particular partition (shard) of the
database

The thing that makes Kafka interestingly different from other mes‐
sage brokers is that it is structured as a log. In fact, it somewhat
How Logs Are Used in Practice | 67
resembles a log file in the sense of Log4j or Syslog: when a producer
sends a message to Kafka, it is literally appended to the end of a file
on disk. 

rather than having the application write
directly to the various datastores, the application only appends the
data to a log (such as Kafka). All the different representations of this
data—your databases, your caches,21 your indexes—are constructed
by consuming the log in sequential order

Change Data Capture (CDC) effectively means replicating data from
one storage technology to another.

We said that Unix tools are composable because they all implement
the same interface of stdin, stdout, and stderr, and each of these
is a file descriptor; that is, a stream of bytes that you can read or write
like a file

- As mentioned, Unix pipes provide a byte stream, whereas Kafka
provides a stream of messages.
- Unix pipes are just a small in-memory buffer, whereas Kafka
durably writes all messages to disk.
- A data stream in Kafka is called a topic, and you can refer to it
by name (which makes it more like a Unix named pipe20)
- In Unix, if the consuming process of a pipe is slow to read the
data, the buffer fills up and the sending process is blocked from
writing to the pipe. This is a kind of backpressure. In Kafka, the
producer and consumer are more decoupled: a slow consumer
has its input buffered, so it doesn’t slow down the producer or
other consumers.
- Kafka keeps messages in a fixed order
