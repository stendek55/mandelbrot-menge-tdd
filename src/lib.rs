//#################################################################
//#################-----eigeneSTRUKTUREN-----######################
//#################################################################
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

//#################################################################
//###################-----eigeneMETHODEN-----######################
//#################################################################
impl Complex {
    pub fn neu(real_teil: f64, imaginaer_teil: f64) -> Self {
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
}
