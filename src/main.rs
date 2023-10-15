fn get_size() -> String {
    use si_scale::prelude::{Base, Constraint, Value};
    use systemstat::{Platform, System};

    let sys = System::new();

    let memory_bits = sys
        .memory()
        .expect("Failed to query memory information!")
        .total
        .as_u64()
        * 8;

    let mem_size = Value::new_with(memory_bits as f64, Base::B1024, Constraint::UnitAndAbove);
    let prefix_string = format!("{:?}", mem_size.prefix).to_uppercase();

    format!("{:.0} {}", mem_size.mantissa, prefix_string)
}

fn get_os() -> String {
    os_info::get().os_type().to_string().to_uppercase()
}

fn main() {
    use figlet_rs::FIGfont;
    use terminal_size::{terminal_size, Width};
    use textflow::align;
    use textflow::{Alignment, Options as TextflowOptions};

    let textflow_options = match terminal_size() {
        Some((Width(width), _)) => TextflowOptions::new(width.into()),
        _ => TextflowOptions::new(80),
    };

    let standard_font = FIGfont::standard().unwrap();

    let banner = "GEOFETCH";

    let figure = match standard_font.convert(banner) {
        Some(banner) => format!("{}", banner),
        _ => banner.to_string(),
    };

    println!(
        "{}",
        align(&figure, Alignment::CENTER, textflow_options.clone())
    );

    let spec = format!("MAX {}", get_size());

    println!(
        "{}",
        align(&spec, Alignment::CENTER, textflow_options.clone())
    );

    println!();

    println!(
        "{}",
        align("PRO-GEAR SPEC", Alignment::CENTER, textflow_options.clone())
    );

    let os_type = get_os();

    println!();

    println!(
        "{}",
        align(&os_type, Alignment::CENTER, textflow_options.clone())
    );

    println!();
}
