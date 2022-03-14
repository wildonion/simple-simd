
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
INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-14T18:50:02.534409500
 INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-14T18:50:02.534899400
 INFO  simd  >  :) Doing some heavy operation on chunk [0]
 INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-14T18:50:02.535312700
 INFO  simd  >  :) Doing some heavy operation on chunk [60]
 INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-14T18:50:02.537517200
 INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-14T18:50:02.538859100
 INFO  simd  >  :) Doing some heavy operation on chunk [210]
 INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-14T18:50:02.539872900
 INFO  utils > collecting all chunks received from the receiver at time 2022-03-14T18:50:02.543530400
 INFO  simd  >  :) Doing some heavy operation on chunk [15]
 INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-14T18:50:02.544438600
 INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-14T18:50:02.547225400
 INFO  simd  > ::::: the result with native threads is [it might be different from the input] 3985935


 INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-14T18:50:02.553185400
 INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-14T18:50:02.554105200
 INFO  simd  >  :) Doing some heavy operation on chunk [0]
 INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-14T18:50:02.554954200
 INFO  simd  >  :) Doing some heavy operation on chunk [60]
 INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-14T18:50:02.555930300
 INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-14T18:50:02.556885700
 INFO  simd  >  :) Doing some heavy operation on chunk [210]
 INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-14T18:50:02.557993400
 INFO  utils > collecting all chunks received from the receiver at time 2022-03-14T18:50:02.560735400
 INFO  simd  >  :) Doing some heavy operation on chunk [15]
 INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-14T18:50:02.561585200
 INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-14T18:50:02.567014
 INFO  simd  > ::::: the result with tokio green threads is [it might be different from the input] 3985935
```


**NOTE** - Due to the time takes to send and receive each chunks inside threads through the `mpsc` channel asyncly the result of each method might be different on each run.