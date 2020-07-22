#[allow(dead_code)]
mod futhark;

// use std::convert::TryInto;
use std::time::Instant;

macro_rules! timed {
    ($d:ident <- $code:stmt) => {
        let first_time = Instant::now();
        $code
        let second_time = Instant::now();
        let $d = second_time - first_time;
    }
}

use futhark::{F64Array1DRef, Context, ContextConfig};

fn main() -> Result<(), i32> {
    let cfg = ContextConfig::new();
    let ctx = local_context!(MyCtx: &cfg);
    timed! { bld_time <- let mut data = (0..100_000_000).into_iter().map(|x| x as f64).collect::<Vec<_>>() }
    timed! { seq_time <- let seq_avg = data.iter().fold(0f64, |a, x| a + x) / data.len() as f64 }
    timed! { fut_time <- let futhark_data = F64Array1DRef::new(&ctx, &mut data) }
    timed! { gpu_time <- let gpu_avg = ctx.avg(futhark_data)? }

    println!("Seq Avg: {}", seq_avg);
    println!("GPU Avg: {}", gpu_avg);
    println!("Bld Time: {} secs {} nanosecs", bld_time.as_secs(), bld_time.subsec_nanos());
    println!("Seq Time: {} secs {} nanosecs", seq_time.as_secs(), seq_time.subsec_nanos());
    println!("Fut Time: {} secs {} nanosecs", fut_time.as_secs(), fut_time.subsec_nanos());
    println!("GPU Time: {} secs {} nanosecs", gpu_time.as_secs(), gpu_time.subsec_nanos());

    Ok(())
}
