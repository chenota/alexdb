<div align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="logo_horizontal.svg">
    <source media="(prefers-color-scheme: dark)" srcset="logo_horizontal_dark.svg">
    <img alt="AlexDB logo" src="logo_horizontal.svg" height="125">
  </picture>
</div>

##  AlexDB Vector Clock Demo

A vector clock is a type of logical clock widely used in distributed systems to determine causality between events. AlexDB can trivially implement a system that updates a process' local vector clock based on messages received.

### How Vector Clocks Work

In a distributed system with $n$ processes, a Vector clock is an $n$-tuple where each item $n_i$ is a logical clock corresponding to process $i$. Every proces stores their own local copy of the vector clock, and each time an event occurs in process $i$, it increments its local clock value $n_i$ by one. Every message sent by process $i$ within the system should be accompanied with $i$'s local vector clock, and when process $j$ receives $i$'s messages, $j$ updates its local clock as follows:

1. Process $j$ updates its own vector clock with the maximum of its own value and the received value for every value in the vector clock. 
2. Process $j$ increments its own value $n_j$ by one.

To determine if $a \prec b$, the following must be true:

1. $\forall i \in \{1..n\} ~ a_i \leq b_i$
2. $\exists i \in \{1..n\} ~ a_i < b_i$

If $\neg (a \prec b \lor b \prec a \lor a \equiv b)$, then $a\|b$, which means that you can't say anything about the order of $a$ and $b$.

### Implementing in AlexDB

First, we need to define a schema for our database to hold received messages. Let's keep the schema simple, so we'll only consider a distributed system with three processes (we're process zero) and only store messages and vector clock values in the database. It's likely that received vector clock values will be close to each other, so we can add XOR compression to those fields.

```
> CREATE TABLE messages (msg str, vc0 num xor, vc1 num xor, vc2 num xor)
```

We have a schema, so let's "receive" a couple of messages and see what the relation looks like with data in it.

```
> INSERT INTO messages VALUES ('Hello', 0, 1, 1)
> INSERT INTO messages VALUES ('Hello Again', 0, 1, 2)
```

```
> SELECT * FROM messages
msg             vc0   vc1   vc2
'Hello'         0     1     1
'Hello Again'   0     1     2
```

Now that we have some data, let's start to define our vector clock updating functionality. The vector clock algorithm makes heavy use of the `max` function, so we'll start by defining a global `max` function to make the rest of this process a little easier.

```
> CREATE CONST max = fun a, b -> if a > b then a else b
```

Should be pretty self-explanitory. Now that we have a `max` function, let's define an aggregate that updates our own vector clock whenever we receive new messages. Vector clocks are naturally represented as tuples, so we're going to store our aggregate value in a tuple.

```
> CREATE AGGREGATE my_vc = [max(current.0, vc0) + 1, max(current.1, vc1), max(current.2, vc2)] INIT [vc0 + 1, vc1, vc2] INTO messages
```

Effectively, this aggregate stores the maximum of its current value the the received vector clock value, and increments our process' value by one. If we did this correctly, we should get a vector clock value of `[2, 1, 2]`, so let's see how we did!

```
> SELECT AGGREGATE my_vc FROM messages
[2, 1, 2]
```

Perfect! Let's insert a couple more values just to make sure we're right.

```
> INSERT INTO messages VALUES ('Another message', 1, 3, 1)
> INSERT INTO messages VALUES ('Hello Again Again', 1, 2, 4)
> SELECT AGGREGATE my_vc FROM messages
[4, 3, 4]
```

All right! Seems like our AlexDB relation is correctly updaing our process' vector clock. We can take this even further and write a `SELECT` query that selects messages that come before a certain vector clock value. Of course, we need a way of knowing if two clocks are causally ordered, so let's implement that first.

```
> CREATE CONST is_causal_before = fun a, b -> (a.0 <= b.0 && a.1 <= b.1 && a.2 <= b.2) && (a.0 < b.0 || a.1 < b.1 || a.2 < b.2)
```

Now, we can easily use our is_causal function to select only messages from our first round of inserts.

```
> SELECT msg FROM messages WHERE is_causal_before([vc0, vc1, vc2], [2, 2, 2])
msg
'Hello'
'Hello Again'
```

### Conclusion

We used AlexDB to store received messages and create a vector clock tracker for a single process in a distributed system. This example demonstrates how AlexDB can be trivially adapted to a wide variety of circumstances that it wasn't necessarily designed for.