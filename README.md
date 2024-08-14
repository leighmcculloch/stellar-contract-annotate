# stellar-contract-annotate
Stellar contracts demonstrating how to annotate invocations.

## `annotate-memo`

An example of an annotation contract which has the responsibility of publishing
events that annotate SEP-41 token transfer destinations.

The contract publishes an event with:
- Topic:
  ```
  symbol:"annotate"`
  ```
- Data:
  ```
  map:{
    symbol:"auther" => address:..., // The party authorizing the send.
    symbol:"address" => symbol:..., // The party receiving the asset.
    symbol:"memo" => string...,     // A string that should be interpreted as
                                    // the off-chain memo associated with the
                                    // destination address
  }
  ```

The contract requires auth (i.e. `require_auth`) by the `auther` over the map
included in the event data. The transfer invocation must be a direct or indirect
sub-invocation and the sub-invocatoin must be captured in the same authorization
entry such that the signature over the annotation is also over the transfer
operation.

Sytems that ingest the two events need to ensure that both events were authorized ... **TODO: How?**
