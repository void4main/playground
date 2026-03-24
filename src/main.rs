use std::arch::asm;
use kuva::plot::SankeyPlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;


fn main() {
    println!("Hello, world!");


    let sankey = SankeyPlot::new()
        .with_link("Input", "Process A", 50.0)
        .with_link("Input", "Process B", 30.0)
        .with_link("Process A", "Output X", 40.0)
        .with_link("Process A", "Output Y", 10.0)
        .with_link("Process B", "Output X", 10.0)
        .with_link("Process B", "Output Y", 20.0);

    unsafe {
        asm!("nop");
    }

    let plots = vec![Plot::Sankey(sankey)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Energy Flow");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));
    std::fs::write("sankey.svg", svg).unwrap();
    println!("Done!");
    loop {
        unsafe {
            asm!("nop");
        }
    }
}
