use mandelbrot_menge_tdd::*;
use minifb::{Key, Window, WindowOptions};
use std::io::{self, Write};

fn main() {
    let max_iterationen = 222;
    let breite = 1111;
    let hoehe = 777;

    let grenzen = Grenzen {
        breite,
        hoehe,
        min_real: -2.1,
        max_real: 1.0,
        min_imag: -1.3,
        max_imag: 1.3,
    };
    let seepferdchen = Grenzen {
        breite,
        hoehe,
        min_real: -0.75,
        max_real: -0.73,
        min_imag: 0.09,
        max_imag: 0.11,
    };
    let sonnensystem = Grenzen {
        breite,
        hoehe,
        min_real: -1.8,
        max_real: -1.7,
        min_imag: -0.05,
        max_imag: 0.05,
    };
    let dreifach_spiral_tal = Grenzen {
        breite,
        hoehe,
        min_real: -0.165,
        max_real: -0.150,
        min_imag: 1.025,
        max_imag: 1.040,
    };
    let elefanten_tal = Grenzen {
        breite,
        hoehe,
        min_real: 0.265,
        max_real: 0.285,
        min_imag: -0.010,
        max_imag: 0.010,
    };
    let herz_medusa = Grenzen {
        breite,
        hoehe,
        min_real: -0.74384,
        max_real: -0.74344,
        min_imag: 0.13167,
        max_imag: 0.13197,
    };
    let antennen_tal = Grenzen {
        breite,
        hoehe,
        min_real: -1.485,
        max_real: -1.465,
        min_imag: -0.010,
        max_imag: 0.010,
    };
    let mini_apfel = Grenzen {
        breite,
        hoehe,
        min_real: -1.775,
        max_real: -1.745,
        min_imag: -0.015,
        max_imag: 0.015,
    };
    let sternen_tal = Grenzen {
        breite,
        hoehe,
        min_real: -0.160,
        max_real: -0.130,
        min_imag: 0.635,
        max_imag: 0.665,
    };
    let doppel_spirale = Grenzen {
        breite,
        hoehe,
        min_real: -0.715,
        max_real: -0.695,
        min_imag: 0.225,
        max_imag: 0.245,
    };
    let herz_einkerbung = Grenzen {
        breite,
        hoehe,
        min_real: 0.250,
        max_real: 0.270,
        min_imag: 0.485,
        max_imag: 0.515,
    };

    loop {
        println!("was möchtest du sehen???");
        println!("[0] -> ganzer apfel");
        println!("[1] -> seepferdchen");
        println!("[2] -> sonnensystem");
        println!("[3] -> dreifaches spiral tal");
        println!("[4] -> elefanten tal");
        println!("[5] -> herz der medusa");
        println!("[6] -> antennen tal");
        println!("[7] -> mini apfel");
        println!("[8] -> sternen tal");
        println!("[9] -> doppelte spirale");
        println!("[10] -> herz einkerbung");
        println!("[q] -> beenden");

        io::stdout().flush().unwrap();
        let mut eingabe = String::new();
        io::stdin()
            .read_line(&mut eingabe)
            .expect("Fehler beim Lesen der Zeile");
        let auswahl = eingabe.trim();

        let daten = match auswahl {
            "0" => render_mandelbrot(&grenzen, max_iterationen),
            "1" => render_mandelbrot(&seepferdchen, max_iterationen),
            "2" => render_mandelbrot(&sonnensystem, max_iterationen),
            "3" => render_mandelbrot(&dreifach_spiral_tal, max_iterationen),
            "4" => render_mandelbrot(&elefanten_tal, max_iterationen),
            "5" => render_mandelbrot(&herz_medusa, max_iterationen),
            "6" => render_mandelbrot(&antennen_tal, max_iterationen),
            "7" => render_mandelbrot(&mini_apfel, max_iterationen),
            "8" => render_mandelbrot(&sternen_tal, max_iterationen),
            "9" => render_mandelbrot(&doppel_spirale, max_iterationen),
            "10" => render_mandelbrot(&herz_einkerbung, max_iterationen),
            "q" | "Q" => {
                println!("tschüss");
                break;
            }
            _ => {
                println!("Ungültige Auswahl! Bitte nutze 0, 1, 2, 3, 4, 5");
                println!("standarwert ganze apfel");
                render_mandelbrot(&grenzen, max_iterationen)
            }
        };

        let mut buffer: Vec<u32> = vec![0; breite * hoehe];

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
                    i if i == max_iterationen => 0x00001312,
                    _ => {
                        // relativer ausbruch wird errechnet damit zooms bei größenunterschieden
                        // von maximum iterationen noch farbmuster ergeben
                        let t = ausbruch as f32 / max_iterationen as f32;
                        match t {
                            i if i < 0.02 => 0x00003300,
                            i if i < 0.05 => 0x00FAEDCD, // Starkes Pastell-Sandgelb
                            i if i < 0.10 => 0x00D90429, // Massiver Kontrast: Knalliges Dunkelrot/Pink
                            i if i < 0.15 => 0x00B5E2FA, // Starkes Pastell-Babyblau
                            i if i < 0.20 => 0x005A189A, // Massiver Kontrast: Tiefes Retro-Lila
                            i if i < 0.25 => 0x00E8AEB7, // Starkes Pastell-Rosa
                            i if i < 0.30 => 0x00007200, // Massiver Kontrast: Dunkles Nadelgrün
                            i if i < 0.35 => 0x00EDF2F4, // Starkes Pastell-Kreideweiß
                            i if i < 0.40 => 0x00EF5350, // Massiver Kontrast: Intensives Korallenrot
                            i if i < 0.48 => 0x00F1C0E8, // Starkes Pastell-Zuckerwatte-Lila
                            i if i < 0.56 => 0x001D3557, // Massiver Kontrast: Deep Ocean Navy
                            i if i < 0.64 => 0x00C1FBA4, // Starkes Pastell-Limettengrün
                            i if i < 0.72 => 0x007209B7, // Massiver Kontrast: Elektrisches Dunkelviolett
                            i if i < 0.80 => 0x00FFC6FF, // Starkes Pastell-Orchidee
                            i if i < 0.88 => 0x000F4C5C, // Massiver Kontrast: Dunkles Vintage-Teal
                            i if i < 0.95 => 0x00EAF2D8, // Starkes Pastell-Salbei
                            _ => 0x00FF85A1, // Äußerster Rand: Leuchtendes Retro-Bubblegum
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
}
