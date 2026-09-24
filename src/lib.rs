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

//####################################################################
//####################-----eigeneFUNKTIONEN-----######################
//####################################################################
/// Berechnet, wie lange eine komplexe Zahl benötigt, um gegen Unendlich zu entweichen.
/// Die Mandelbrot-Menge ist die Menge aller komplexen Zahlen c, für die die Folge
/// z_{n+1} = z_n² + c (mit dem Startwert z_0 = 0) beschränkt bleibt. Das bedeutet,
/// der Wert von z darf nicht ins Unendliche wachsen, sondern muss "gefangen" bleiben.
pub fn ausbruch_zeit(cpx_c: &Complex, max_iterationen: usize) -> Option<usize> {
    // Jede komplexe Zahl startet bei der Berechnung im Ursprung: z_0 = 0 + 0i.
    // Das ist der mathematische Ankerpunkt für die gesamte Fraktal-Berechnung.
    //startpunkt
    let mut cmplx_z = Complex::neu(0.0, 0.0);

    // Jeder Durchlauf repräsentiert den Schritt von z_n zu z_{n+1}.
    //durchgänge
    for i in 0..max_iterationen {
        // DIE FORMEL: z_{n+1} = z_n² + c
        // Hier passiert das mathematische "Chaos" und die Rückkopplung:
        // 1. z.square(): Wir nehmen das aktuelle z und quadrieren es.
        //    (Mathematisch: (re + im*i)² = re² - im² + 2*re*im*i)
        // 2. .add(&c): Wir addieren die feste Koordinate c (den Punkt, den wir gerade prüfen).
        // Das Ergebnis wird das neue z für den nächsten Schleifendurchlauf.
        //formel anwenden -> z_{n+1} = z_n² + c
        cmplx_z = cmplx_z.quadrieren().addition(cpx_c); //-> parameter ohne &
        // hinweis zur parameterübergabe an .addition()
        // |-> es erwartet einen zeiger auf Complex
        // da in diese funktion der parameter schon als zeiger ankommt
        // wird er hier ohne das & weitergegeben

        // MATHEMATISCHER HINTERGRUND DES ESCAPES (Flucht-Bedingung):
        // Es ist mathematisch bewiesen: Sobald der Abstand (die Norm) einer Zahl z
        // zum Nullpunkt größer als 2 wird, bricht die Folge unaufhaltsam gegen Unendlich aus.
        // Um die langsame und rechenintensive Quadratwurzel (für den echten Abstand) zu vermeiden,
        // nutzen wir das Betragsquadrat.
        //escape-bedingung prüfen
        if cmplx_z.quadrierte_norm() > 4.0 {
            // Der Punkt ist "entwichen" (escaped).
            // Wir geben Some(i) zurück. Die Zahl i sagt uns, *wie schnell* (in welchem Schritt)
            // der Punkt ausgebrochen ist. Das wird später genutzt, um das Fraktal bunt einzufärben!
            return Some(i);
        }
    }
    // DIE BEDEUTUNG VON None:
    // Wenn z selbst nach z. B. 150 Durchläufen immer noch einen Abstand kleiner oder gleich 2 hat,
    // gehen wir davon aus, dass diese Folge stabil ist und niemals gegen Unendlich entweichen wird.
    // Das bedeutet: Dieser Punkt c GEHÖRT ZUR MANDELBROT-MENGE.
    // In Grafiken ist das der berühmte "schwarze Kern" (Apfelmännchen).
    None
}

pub fn render_mandelbrot(_grenzen: &Grenzen, _max_iterationen: usize) -> Vec<usize> {
    unimplemented!()
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

    #[test]
    fn test_lege_pixel_in_zentrum_complexer_ebene() {
        let grenzen = Grenzen {
            breite: 200,
            hoehe: 200,
            min_real: -2.0,
            max_real: 2.0,
            min_imag: -2.0,
            max_imag: 2.0,
        };

        // die exakte mitte muss bei diesem symetrischen grid (0.0, 0.0) sein!
        let zentrum = grenzen.pixel_in_complex(100, 100);
        assert!(zentrum.real == 0.0);
        assert!(zentrum.imag == 0.0);
    }

    //####################-----MANDELBROTkernlogik -> escapeZEIT-----############################
    #[test]
    fn test_escape_zeit_von_nullpunkt_in_menge() {
        //der nullpunkt (0,0) gehört garantiert zur mandelbrot-menge
        // |--> darf niemals ausbrechen
        let cmplx_innen = Complex::neu(0.0, 0.0);
        assert_eq!(
            ausbruch_zeit(&cmplx_innen, 155),
            None,
            "der nullpunkt ist nicht stabil geblieben und hat kein None geliefert"
        );
    }

    #[test]
    fn test_escape_zeit_sofort_entweichen_von_wert_ausserhalb_der_menge() {
        //der punkt (3,3) ist weit ausserhalb der mandelbrot-menge
        // |-> muss sofort ausbrechen
        let cmplx_aussen = Complex::neu(3.0, 3.0);
        let ergebnis_ausbruch = ausbruch_zeit(&cmplx_aussen, 10);
        assert!(ergebnis_ausbruch.is_some(), "punkt (3,3) muss entweichen");
        assert_eq!(
            ergebnis_ausbruch,
            Some(0),
            "muss sofort ausbrechen bei iteration 0 -> denn startwert > 2"
        );
    }

    #[test]
    fn test_escape_zeit_langsam_entweichen_von_wert_knapp_ausserhalb_der_menge() {
        //der punkt (1,0) ist am rand ausserhalb der mandelbrot-menge
        // |-> muss nach wenigen schritten ausbrechen
        let cmplx_aussen = Complex::neu(1.0, 0.0);
        let ergebnis_ausbruch = ausbruch_zeit(&cmplx_aussen, 10);
        assert!(
            ergebnis_ausbruch.unwrap() < 4,
            "muss langsam ausbrechen bei iteration kleiner 4"
        );
    }

    #[test]
    fn test_escape_zeit_nicht_entweichen_da_wert_knapp_innerhal_menge() {
        //der punkt (-1,0) ist am rand innerhalb der mandelbrot-menge
        // |-> oszilliert stabil -> darf nicht entweichen
        let cmplx_innen = Complex::neu(-1.0, 0.0);
        assert_eq!(
            ausbruch_zeit(&cmplx_innen, 155),
            None,
            "der punkt (-1,0) bleibt nicht stabil in menge"
        );
    }

    //#################-----RENDERER-----#################################################
    #[test]
    fn test_hat_vector_korrekte_groesse_beim_rendern() {
        let grenzen = Grenzen {
            breite: 11,
            hoehe: 5,
            min_real: -2.0,
            max_real: 1.0,
            min_imag: -1.0,
            max_imag: 1.0,
        };

        // der gerenderte vector muss breite * hoehe elemnte haben
        let grid = render_mandelbrot(&grenzen, 10);
        assert_eq!(
            grid.len(),
            grenzen.breite * grenzen.hoehe,
            "der vector hat falsche anzahl an elementen"
        );
    }
}
