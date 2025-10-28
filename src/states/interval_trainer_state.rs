#[derive(Default)]
struct IntervalTrainer
{
    root: String,
    intervals: Vec<u8> //data to contain intervals from root note in integer format. Ex: [1,3] will contain intervals 1 step and 3 steps above it.
}