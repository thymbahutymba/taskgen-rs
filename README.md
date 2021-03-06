Taskset generator with rt-app compatible json output.

```
taskgen-rs 0.1.0
Taskset generator with rt-app compatible json output.

USAGE:
    taskgen-rs [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
        --round-C    Round execution times to nearest integer
    -V, --version    Prints version information

OPTIONS:
        --default-policy <default-policy>    rt-app default scheduling policy of threads. [default: SCHED_OTHER]
                                             [possible values: SCHED_OTHER, SCHED_IDLE, SCHED_RR, SCHED_FIFO,
                                             SCHED_DEADLINE]
    -f, --from-file <from-file>              Load the tasksets from file which was previously generated by taskgen.py.
                                             If this option is provided all other taskgen option are ignored.
    -n, --num-tasks <n>                      Produce tasksets of size N [default: 5]
    -s, --num-sets <nsets>                   Produce SETS tasksets [default: 3]
    -d, --period-distribution <perdist>      Choose period distribution. [default: logunif]  [possible values: unif,
                                             logunif]
    -g, --period-gran <pergran>              Set period granularity to PGRAN [PMIN]
    -q, --period-max <permax>                Set maximum period value to PMAX [PMIN]
    -p, --period-min <permin>                Set minimum period value to PMIN [default: 1000]
    -S, --seed <seed>                        Set the random number generator seed [default: 0]
    -u, --taskset-utilisation <util>         Set total taskset utilisation to UTIL [default: 0.75]
```