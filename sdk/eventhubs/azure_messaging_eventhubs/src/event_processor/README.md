# Azure Event Hubs Event Processor notes

The following are a set of collected notes on Event Hubs event processor functionality.

## Processor

The processor provides a mechanism interface which allows MULTIPLE Event Hubs clients to process the events from multiple partitions while attempting to ensure that events are never lost.

### Processor concepts

#### Checkpoint

A "Checkpoint" represents a fixed location within an Event Hubs partition. A Checkpoint contains two sets of information:

1. Information to uniquely identify the EventHubs partition (Event Hub instance FQDN, Event Hub name, consumer group, and partition ID)
1. Information to identify an event within that partition (sequence number or offset).

#### Ownership

Within the processor, each partition is "owned" by a processor client. Ownership is defined by an "Ownership" structure, which describes the partition being owned, which client "owns" the partition, and when was the last time that the ownership was updated (this allows the processor to claim ownership of clients which were previously owned by processor instances that are no longer running).

#### Checkpoint Store

A Checkpoint Store is used to persist the value of `Checkpoints` and `Ownerships`. Typically different processors will use the same checkpoint store (normally backed by Azure Blob Storage) to coordinate between the processors.

A Checkpoint store contains 4 methods:

- list_checkpoints - Returns the current checkpoints for a given EventHubs instance.
- list_ownership - Returns the owners for a given EventHubs instance.
- claim_ownership - Claim ownership for a set of partitions.
- update_checkpoint - Update a checkpoint with a new checkpoint value.

#### Partition Client

A processor partition client receives events from an eventhubs instance. It also can update the checkpoint for the partition from a received message.

### Processor configuration options

The processor has several configuration options available:

- Load Balancing Strategy - the strategy used by the load balancer (Greedy or Balanced). More on load balancing strategy in [Load Balancing Strategy]
- Update Interval - the duration that the processor should sleep between processor cycles. The default update interval is 30 seconds.
- Partition Expiration Duration - The time after which a partition is considered unowned. The default partition expiration duration is 2 minutes.
- Start Positions - the starting position for each partition (or a default starting position for all partitions).
- Prefetch - the number of items to prefetch for each partition client.
- Maximum number of partitions - The maximum number of partitions which the processor should manage.

### Processor operation

A processor client starts the processor running in the background (using a language specific mechanism to start the processor). It then calls into the processor to receive a "partition client".

The client can then use the partition client to receive messages, and after it has processed the message, it asks the processor to update the checkpoint for the client (indicating that the client has finished processing the message and that the processor should not hand this message to another processor client).

When the processor starts, it runs a dispatch cycle and then sleeps for the processor update interval. This continues until either an error occurs or the processor is stopped.

### Processor dispatch cycle

For each processor dispatch cycle, the processor first load balances among the partitions.
The load balancer will return a set of ownerships that the current processor instance now owns. The processor will then add a partition client to the set of partition clients which can be returned to the client.

### Load Balancing Cycle

The first thing that the load balancer does is to query the checkpoint store for the current state of the checkpoint store. It categories all ownerships into three buckets:

- Partitions owned by the current processor instance.
- Partitions that are unowned or are expired.
- Partitions whose clients own "too many" partitions.

The load balancer also determines the number of *active* processors (based on ownership).

Once the load balancer has determined the set of ownerships, the load balancer attempts to claim the "appropriate" number of partitions.

To do this, the processor performs the following operations:

- If the processor instance already has its fair share of partitions, exit.
- If there are unowned partitions, claim one at random.
- If there are no unowned partitions, pick one to "steal" at random.
- Update the checkpoint store with any changes. If the partition we are attempting to claim is currently owned by another processor, that will result in an HTTP 412 error. If we receive this, update local state to remove the ownership claim.

#### Raw notes on load balancing

- Renew Local Ownership
  - Based on local state
  - Each that the processor thinks it owns, call to SetMetadata.  May call Upload.
  - If no longer owned, results in HTTP 412, local state updated to remove ownership   (seen in the logs; indicates stolen between load balancing cycles)

- List All Ownership
  - Calls ListBlobs with metadata trait set
  - If this fails, the load balancing cycle cannot continue; local state is preserved  (seen in logs; processors will fight and Event Hubs will enforce single reader)

- Calculate Ownership
  - Update state for all ownership and all unowned partitions; expired ownership is unowned
  - Determine number of active processors by looking at ownership; an active processor must own at least one partition

- Claim Ownership
  - Determine if this instance has its fair share based on count of active processors
  - If fair share is owned, do nothing; assume unowned will be claimed by a processor without its fair share
  - If unowned partitions, pick one to claim at random
  - If no unowned partitions, pick one to steal at random
  - Update storage with any change, call to SetMetadata.  May call Upload.
  - If claimed by another, results in HTTP 412, local state updated to remove ownership   (seen in the logs; indicates stolen between load balancing cycles)

- Determine balance stability
  - If fair share of partitions are owned and no claim was needed, assume stable

- Ensure owned partitions are being processed
  - If no current processing task, initialize and start
  - If owned partition has a completed task, capture exceptions, initialize and restart

- Calculate next cycle time
  - If greedy strategy and not stable, run immediate
  - If elapsed time was more than the load balancing interval, run immediate
  - Delay for (load balancing interval - current cycle time), then run next

## Load Balancing Strategy

 CALLS PER CYCLE
    - Event Hubs
      - Query partitions (1)
      - Create receiver, start reading (varies by claimed/faulted, max of owned partitions)

    - Storage
      - List Blobs (1)
      - SetMetadata (varies by owned/claimed, max of partition count * 2)
      - Upload (varies by new partitions, max of partition count)
