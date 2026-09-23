//#################################################################
//#################-----eigeneSTRUKTUREN-----######################
//#################################################################
#[derive(Debug, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

//#################################################################
//###################-----eigeneMETHODEN-----######################
//#################################################################
impl Complex {
    pub fn neu(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    pub fn quadrierte_norm(&self) -> f64 {
        unimplemented!()
    }

    pub fn addition(&self, _andere: &Complex) -> Complex {
        unimplemented!()
    }

    pub fn quadrieren(&self) -> Complex {
        unimplemented!()
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
            Complex::neu(3.5, 1.5),
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
}
