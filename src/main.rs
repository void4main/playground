use std::arch::asm;

use kuva::plot::SankeyPlot;
use kuva::backend::svg::SvgBackend;
use kuva::render::render::render_multiple;
use kuva::render::layout::Layout;
use kuva::render::plots::Plot;


fn main() -> ! {
    println!("Hello, world!");

    let sankey = SankeyPlot::new()
        .with_node_color("Budget",    "#e41a1c")
        .with_node_color("R&D",       "#377eb8")
        .with_node_color("Marketing", "#4daf4a")
        .with_node_color("Ops",       "#ff7f00")
        .with_node_color("Product A", "#984ea3")
        .with_node_color("Product B", "#a65628")
        .with_link("Budget",    "R&D",       40.0)
        .with_link("Budget",    "Marketing", 25.0)
        .with_link("Budget",    "Ops",       35.0)
        .with_link("R&D",       "Product A", 25.0)
        .with_link("R&D",       "Product B", 15.0)
        .with_link("Marketing", "Product A", 15.0)
        .with_link("Marketing", "Product B", 10.0)
        .with_link("Ops",       "Product A", 20.0)
        .with_link("Ops",       "Product B", 15.0)
        .with_gradient_links()
        .with_link_opacity(0.6);

    let plots = vec![Plot::Sankey(sankey)];
    let layout = Layout::auto_from_plots(&plots)
        .with_title("Budget Allocation — Gradient Ribbons");

    let svg = SvgBackend.render_scene(&render_multiple(plots, layout));

    loop {
        unsafe {
            asm!("nop")
        }
    }
}



