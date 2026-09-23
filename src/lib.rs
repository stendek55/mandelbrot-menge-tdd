//#################################################################
//#################-----eigeneSTRUKTUREN-----######################
//#################################################################
#[derive(Debug, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

pub struct Grenzen {
    breite: usize,
    hoehe: usize,
    min_real: f64,
    max_real: f64,
    min_imag: f64,
    max_imag: f64,
}

//#################################################################
//###################-----eigeneMETHODEN-----######################
//#################################################################
impl Complex {
    pub fn neu(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    // kurze hinweise zur verwendeten mathematik im testbereich

    pub fn quadrierte_norm(&self) -> f64 {
        (self.real * self.real) + (self.imag * self.imag)
    }

    pub fn addition(&self, andere: &Complex) -> Complex {
        Complex::neu(self.real + andere.real, self.imag + andere.imag)
    }

    pub fn quadrieren(&self) -> Complex {
        let real_neu = (self.real * self.real) - (self.imag * self.imag);
        let imag_neu = 2f64 * (self.real * self.imag);
        Complex::neu(real_neu, imag_neu)
    }
}

impl Grenzen {
    pub fn pixel_in_complex(&self, x: usize, y: usize) -> Complex {
        // lineare skalierung für realteil - Xachse
        let real =
            self.min_real + (x as f64 / self.breite as f64) * (self.max_real - self.min_real);

        // lineare skalierung für imaginärteil - Yachse
        let imag = self.min_imag + (y as f64 / self.hoehe as f64) * (self.max_imag - self.min_imag);
        Complex::neu(real, imag)
    }
}

//#################################################################
//####################-----TDDbereich-----#########################
//#################################################################
#[cfg(test)]
mod tests {
    use super::*;

    //##############-----komplexeARITHMETIK-----#######################
    #[test]
    fn test_neue_complexe_struktuer_erstellen() {
        let cpx = Complex::neu(1.1, -2.2);
        assert_eq!(cpx.real, 1.1, "REALteil nicht korrekt gesetzt");
        assert_eq!(cpx.imag, -2.2, "IMAGINÄRrteil nicht korrekt gesetzt");
    }

    #[test]
    fn test_betrags_quadrat_einer_complexen_zahl() {
        //quadrierte norm
        //für z = a + bi -> betragsquadrat |z|² = a² + b²
        //verzicht auf wurzel um performance zu steigern
        let cpx = Complex::neu(4.0, 2.0);
        assert_eq!(
            cpx.quadrierte_norm(),
            20f64,
            "BETRAGSQUADRAT nicht korrekt errechnet"
        );
        //negative werte testen
        let cpx = Complex::neu(-2.0, -3.0);
        assert_eq!(
            cpx.quadrierte_norm(),
            13f64,
            "BETRAGSQUADRAT bei negativen input nicht korrekt errechnet"
        );
    }

    #[test]
    fn test_addition_einer_complexen_zahl() {
        //man addiert realteile miteinander und imaginärteile miteinander
        //(a + bi) + (c + di) = (a + c) + (b + d)i
        let cpx_1 = Complex::neu(1.5, 2.0);
        let cpx_2 = Complex::neu(3.0, 4.5);
        let ergebnis = cpx_1.addition(&cpx_2);
        assert_eq!(
            ergebnis,
            Complex::neu(4.5, 6.5),
            "ADDITION complexerZahl nicht korrekt errechnet"
        );
        //negative werte testen
        let cpx_1 = Complex::neu(-1.5, 2.0);
        let cpx_2 = Complex::neu(3.0, -4.5);
        let ergebnis = cpx_1.addition(&cpx_2);
        assert_eq!(
            ergebnis,
            Complex::neu(1.5, -2.5),
            "ADDITION negativer complexerZahl nicht korrekt errechnet"
        );
    }

    #[test]
    fn test_quadrieren_einer_complexen_zahl() {
        //binomische formel
        //(a + bi)² = a² + 2abi + (bi)²
        //es gilt -> i² = -1
        //daraus folgt -> a² + 2abi - b²
        // Sortiert nach Real- und Imaginärteil:
        // Neuer Realteil      = a² - b²
        // Neuer Imaginärteil = 2ab
        let cpx = Complex::neu(2.0, 3.0);
        let ergebnis = cpx.quadrieren();
        assert_eq!(
            ergebnis,
            Complex::neu(-5.0, 12.0),
            "QUADRIEREN einer complexenZahl nich5t korrekt errechnet"
        );
        //negative zahl testen
        let cpx = Complex::neu(-3.0, 2.0);
        let ergebnis = cpx.quadrieren();
        assert_eq!(
            ergebnis,
            Complex::neu(5.0, -12.0),
            "QUADRIEREN einer negativen complexenZahl nich5t korrekt errechnet"
        );
    }

    //#########-----KOORDINATENumrechnung (PIXEL -> KOMPLEX)-----#################
    #[test]
    fn test_wandle_complexe_ebene_in_pixel_bereich() {
        //X-Achse für den Realteil
        //Y-Achse für den Imaginärteil
        let grenzen = Grenzen {
            breite: 100,
            hoehe: 100,
            min_real: -2.0,
            max_real: 2.0,
            min_imag: -1.0,
            max_imag: 1.0,
        };

        // Obere linke Ecke (Pixel 0,0) muss exakt den minimalen Werten entsprechen
        let oben_links = grenzen.pixel_in_complex(0, 0);
        assert_eq!(
            oben_links.real, -2.0,
            "komplexer wert nicht korrekt auf pixelbereich gelegt - oben links - Xreal"
        );
        assert_eq!(
            oben_links.imag, -1.0,
            "komplexer wert nicht korrekt auf pixelbereich gelegt - oben links - Yimaginär"
        );

        // Untere rechte Ecke (Pixel 100,100) muss den maximalen Werten entsprechen
        let unten_rechts = grenzen.pixel_in_complex(100, 100);
        assert_eq!(
            unten_rechts.real, 2.0,
            "komplexer wert nicht korrekt auf pixelbereich gelegt - unten rechts - Xreal"
        );
        assert_eq!(
            unten_rechts.imag, 1.0,
            "komplexer wert nicht korrekt auf pixelbereich gelegt - unten rechts - Yimaginär"
        );
    }
}
