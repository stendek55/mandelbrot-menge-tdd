use mandelbrot_menge_tdd::*;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let max_iterationen = 155;
    let breite = 555;
    let hoehe = 444;

    //ganzer apfel
    let grenzen = Grenzen {
        breite,
        hoehe,
        min_real: -2.1,
        max_real: 1.0,
        min_imag: -1.3,
        max_imag: 1.3,
    };
    let _seepferdchen = Grenzen {
        breite,
        hoehe,
        min_real: -0.75,
        max_real: -0.73,
        min_imag: 0.09,
        max_imag: 0.11,
    };
    let _sonnensystem = Grenzen {
        breite,
        hoehe,
        min_real: -1.8,
        max_real: -1.7,
        min_imag: -0.05,
        max_imag: 0.05,
    };
    let _dreifach_spiral_tal = Grenzen {
        breite,
        hoehe,
        min_real: -0.165,
        max_real: -0.150,
        min_imag: 1.025,
        max_imag: 1.040,
    };
    let _elefanten_tal = Grenzen {
        breite,
        hoehe,
        min_real: 0.265,
        max_real: 0.285,
        min_imag: -0.010,
        max_imag: 0.010,
    };
    //bei 1000 iterationen
    let _herz_medusa = Grenzen {
        breite,
        hoehe,
        min_real: -0.74384,
        max_real: -0.74344,
        min_imag: 0.13167,
        max_imag: 0.13197,
    };

    let mut buffer: Vec<u32> = vec![0; breite * hoehe];
    let daten = render_mandelbrot(&grenzen, max_iterationen);

    for y in 0..hoehe {
        for x in 0..breite {
            let index = y * breite + x;
            let ausbruch = daten[index];
            let farbe = match ausbruch {
                //minimal variante zum einfärben
                //i if i < (max_iterationen / 8) => 0x00000000,
                //5..=55 => 0xffffffff,
                //_ => 0x004a6b53,

                // feiner abgestufte variante
                i if i == max_iterationen => 0x00000000,
                _ => {
                    // relativer ausbruch wird errechnet damit zooms bei größenunterschieden
                    // von maximum iterationen noch farbmuster ergeben
                    let t = ausbruch as f32 / max_iterationen as f32;
                    match t {
                        i if i < 0.02 => 0x004A6B53,
                        i if i < 0.05 => 0x00FFA500,
                        i if i < 0.15 => 0x00FDFFB6,
                        i if i < 0.35 => 0x00CAFFBF,
                        i if i < 0.65 => 0x009BF6FF,
                        _ => 0x00333333,
                    }
                }
            };
            buffer[index] = farbe;
        }
    }
    // das eigentliche fenster erstellen
    let mut window = Window::new(
        "MANDELBROT-stendek55",
        breite,
        hoehe,
        WindowOptions::default(),
    )
    .unwrap();

    // fenster offen halten -> bis ESC gedrückt wird
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, breite, hoehe).unwrap();
    }
}
