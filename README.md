
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

## Sample Output on Equal Condition

```console
INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-16T18:19:47.883156
INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-16T18:19:47.885159800
INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-16T18:19:47.885159800
INFO  simd  >  --------Doing some heavy operation on chunk [0]
INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-16T18:19:47.885159800
INFO  simd  >  --------Doing some heavy operation on chunk [60]
INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-16T18:19:47.885159800
INFO  simd  >  --------Doing some heavy operation on chunk [210]
INFO  utils > collecting all chunks received from the receiver at time 2022-03-16T18:19:47.886155
INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-16T18:19:47.886155
INFO  simd  >  --------Doing some heavy operation on chunk [15]
INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-16T18:19:47.886155
INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-16T18:19:47.887157100
INFO  utils > collected bytes -> [0, 60, 210, 15] at time 2022-03-16T18:19:47.887157100
INFO  simd  > ::::: the result is 3985935 - [it might be different from the input] - | cost : 4.0779
```

## Sample Output on Unequal Condition

```console
INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-16T18:20:57.775299
INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-16T18:20:57.776326200
INFO  simd  >  --------Doing some heavy operation on chunk [0]
INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-16T18:20:57.779358200
INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-16T18:20:57.780341
INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-16T18:20:57.780341
INFO  simd  >  --------Doing some heavy operation on chunk [60]
INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-16T18:20:57.783330100
INFO  utils > collecting all chunks received from the receiver at time 2022-03-16T18:20:57.782328700
INFO  simd  >  --------Doing some heavy operation on chunk [15]
INFO  simd  >  --------Doing some heavy operation on chunk [210]
INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-16T18:20:57.787324900
INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-16T18:20:57.788324300
INFO  utils > collected bytes -> [0, 60, 15, 210] at time 2022-03-16T18:20:57.790324800
INFO  simd  > ::::: the result is 3936210 - [it might be different from the input] - | cost : 15.9839
```

## Concurrency Beauty!

**NOTE** - Due to the time which takes to send and receive each chunks inside threads through the `mpsc` channel asyncly, the result might be different on each run and it depends on the system, but here at first run both input and the result got into an equality condition.