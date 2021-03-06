use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Taskset generator with rt-app compatible json output.")]
pub struct Opt {
    #[structopt(flatten)]
    pub taskgen_options: TaskgenOpt,
    #[structopt(flatten)]
    pub rtapp_options: RTAppOpt,
    #[structopt(
        long = "from-file",
        short = "f",
        help = "Load the tasksets from file which was previously generated by taskgen.py. \
                If this option is provided all other taskgen option are ignored.",
        parse(from_os_str),
        conflicts_with = "tasksets"
    )]
    pub from_file: Option<PathBuf>,
    #[structopt(
        long = "deserialize",
        help = "Deserialize json tasksets and produce back the taskgen output. \
                If this option is provided all other taskgen option are ignored.",
        parse(from_os_str)
    )]
    pub tasksets: Option<Vec<PathBuf>>,
}

#[derive(StructOpt, Debug, Clone)]
pub struct TaskgenOpt {
    // Configuration for taskgen
    #[structopt(
        long = "taskset-utilization",
        short = "u",
        help = "Set total taskset utilization to UTIL",
        default_value = "0.75"
    )]
    pub util: f32,
    #[structopt(
        long = "num-tasks",
        short = "n",
        help = "Produce tasksets of size N",
        default_value = "5"
    )]
    pub n: usize,
    #[structopt(
        long = "num-sets",
        short = "s",
        help = "Produce SETS tasksets",
        default_value = "3"
    )]
    pub nsets: usize,
    #[structopt(
        long = "seed",
        short = "S",
        help = "Set the random number generator seed",
        default_value = "0"
    )]
    pub seed: isize,
    #[structopt(
        possible_values = &["unif", "logunif"],
        long = "period-distribution",
        short = "d",
        help = "Choose period distribution.",
        default_value = "logunif"
    )]
    pub perdist: String,
    #[structopt(
        long = "period-min",
        short = "p",
        help = "Set minimum period value to PMIN",
        default_value = "1000"
    )]
    pub permin: isize,
    #[structopt(
        long = "period-max",
        short = "q",
        help = "Set maximum period value to PMAX [PMIN]",
        //default_value = None,
    )]
    pub permax: Option<isize>,
    #[structopt(
        long = "period-gran",
        short = "g",
        help = "Set period granularity to PGRAN [PMIN]"
    )]
    pub pergran: Option<isize>,
    #[structopt(long = "round-C", help = "Round execution times to nearest integer")]
    pub round_c: bool,
    /*#[structopt(
        long = "output-format",
        short = "f",
        help = "Specify output format as a Python template string.  The following variables are available: Ugen - the task utilisation value generated by Stafford's randfixedsum algorithm, T - the generated task period value, C - the generated task execution time, U - the actual utilisation equal to C/T which will differ from Ugen if the --round-C option is used.  See below for further examples.  A new line is always inserted between tasksets.",
        default_value = "%(Ugen)f %(U)f %(C).2f %(T)d"
    )]
    pub format: String,*/
}
#[derive(StructOpt, Debug, Serialize, Deserialize, Clone)]
pub struct RTAppOpt {
    #[structopt(
        long,
        help = "rt-app default scheduling policy of threads.",
        default_value = "SCHED_OTHER",
        possible_values = &["SCHED_OTHER", "SCHED_IDLE", "SCHED_RR", "SCHED_FIFO", "SCHED_DEADLINE"],
    )]
    default_policy: String,
    #[structopt(
        long,
        help = "A integer skips the calibration step and uses the integer value as ns per loop.",
        default_value = "51"
    )]
    calibration: usize,
    //#[structopt(long, help = "Enable the prority inheritance of mutex")]
    #[structopt(skip)]
    pi_enabled: bool,
    //#[structopt(long, help = "Lock the mem page in RAM")]
    #[structopt(skip = true)]
    lock_pages: bool,
    #[structopt(
        long,
        parse(from_os_str),
        default_value = "./rt-app-log",
        help = "Path to store the various log files"
    )]
    logdir: PathBuf,
    /*#[structopt(
        long,
        help = "Prefix used for all log files of the use case",
        default_value = "rt-app"
    )]*/
    #[structopt(skip = "rt-app")]
    log_basename: String,
    /*#[structopt(
        long,
        help = "A string is used to set a predifined behavior",
        possible_values = &["file", "Disable", "Auto", "Size"],
        default_value = "file"
    )]*/
    //#[structopt(skip = "file")]
    //log_mode: String,
    /*#[structopt(
        long,
        help = "A Integer defines a fix size in MB of the temporary buffer, required if log_mode set to `Size`.",
        required_if("log_mode", "Size")
    )]*/
    #[structopt(skip = "auto")]
    log_size: String,
    /*#[structopt(
        long,
        help = "rt-app logs in ftrace the events corresponding to the requested categories",
        possible_values = &["main", "task", "run", "loop", "stats"],
        use_delimiter = true
    )]
    ftrace: Option<Vec<String>>,*/
    #[structopt(long, help = "Create a gnu plot compatible file for each threads")]
    //#[structopt(skip)]
    gnuplot: bool,
    /*#[structopt(
        long,
        help = "Path to the file which will be written to create IO-bounded busy loop",
        default_value = "/dev/null"
    )]
    io_device: String,
    #[structopt(
        long,
        help = "The size of per-thread memory buffer in byte being used to create IO-bounded and memory-bounded busy loop.",
        default_value = "4194304"
    )]
    mem_buffer_size: usize,
    #[structopt(
        long,
        help = "Accumulate slack (see below) measured during successive timer events in a phase"
    )]
    cumulative_slack: bool,*/
}
