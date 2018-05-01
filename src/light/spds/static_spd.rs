use light::spds::data::*;
use light::spds::RegularSPD;
use light::spectrum_wavelengths::*;

const CIE_SCALE: f32 =
    SCALE_W_TO_LM + (WAVELENGTH_END - WAVELENGTH_START) / WAVELENGTH_SAMPLES as f32;

lazy_static! {
    static ref spd_ciex: RegularSPD = RegularSPD::new(&CIE_X, CIE_START as _, CIE_END as _, CIE_SCALE);
}
