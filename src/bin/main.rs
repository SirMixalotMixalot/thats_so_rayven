use thats_so_rayven::run;
fn main() {
    //Image info
    const ASPECT_RATIO : f64 = 16. / 9. ;
    const IM_WIDTH : i32 = 400;
    const IM_HEIGHT : i32 = (IM_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL : usize = 10usize;
    const RECURSE_DEPTH: u32 = 50;
    run(IM_HEIGHT,IM_WIDTH,SAMPLES_PER_PIXEL, RECURSE_DEPTH);
}
