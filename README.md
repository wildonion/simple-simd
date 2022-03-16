
# Run 

```console 
$ cargo run
```

[Question](https://quera.org/problemset/113613/)


## Inputs

* An operation function
* u32 bits number

## Output

* u32 bits number


## Sample Input

* _heavy_func_
* _3985935_

## Sample Output

```console
INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-16T15:50:58.905877800
INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-16T15:50:58.907882800
INFO  simd  >  --------Doing some heavy operation on chunk [0]
INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-16T15:50:58.907882800
INFO  simd  >  --------Doing some heavy operation on chunk [60]
INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-16T15:50:58.909883
INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-16T15:50:58.909883
INFO  simd  >  --------Doing some heavy operation on chunk [210]
INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-16T15:50:58.910878200
INFO  utils > collecting all chunks received from the receiver at time 2022-03-16T15:50:58.912874800
INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-16T15:50:58.913878
INFO  simd  >  --------Doing some heavy operation on chunk [15]
INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-16T15:50:58.915875
INFO  utils > collected bytes -> [0, 60, 210, 15] at time 2022-03-16T15:50:58.916875900
INFO  simd  > ::::: the result with native threads is 3985935 - [it might be different from the input] - | cost : 11.5148


INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-16T15:50:58.917873700
INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-16T15:50:58.919876400
INFO  simd  >  --------Doing some heavy operation on chunk [0]
INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-16T15:50:58.921869700
INFO  simd  >  --------Doing some heavy operation on chunk [60]
INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-16T15:50:58.922874700
INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-16T15:50:58.923879
INFO  simd  >  --------Doing some heavy operation on chunk [210]
INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-16T15:50:58.924882
INFO  utils > collecting all chunks received from the receiver at time 2022-03-16T15:50:58.925877100
INFO  simd  >  --------Doing some heavy operation on chunk [15]
INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-16T15:50:58.926876400
INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-16T15:50:58.927877
INFO  utils > collected bytes -> [0, 60, 210, 15] at time 2022-03-16T15:50:58.929877100
INFO  simd  > ::::: the result with tokio green threads is 3985935 - [it might be different from the input] - | cost : 11.9192
```

## Concurrency Beauty!

**NOTE** - Due to the time which takes to send and receive each chunks inside threads through the `mpsc` channel asyncly, the result of each method might be different on each run and it depends on the system, but here at first run both input and result got into an equality condition.