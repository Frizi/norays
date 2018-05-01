/*
Some facts and ideas:

    - fundamentally, we cannot only care only about 3 colors, because of metamerism
        - shader should then output a spectrum
        - full spectrum tracing is too slow to be practical
        - tracing single wavelength introduces lots of variance
        - something in-between: trace multiple cleverly selected wavelengths, different set per sample
            - http://graphicsinterface.org/wp-content/uploads/gi1999-7.pdf

    - to achieve refraction, we need separation of wavelengths
        - single wavelength tracing necessary when light path is wavelength dependant
        - not all light paths have to be separated, in fact most of them will probably not
        - light path may collapse to single wavelength only when necessary

    - every ray trace could be modelled as a job to do
        - jobs can spawn more jobs and depend on their outcome
        - jobs have expected outcome, modelled as Future
        - jobs are run by coordinated scheduler in multithreaded fashion
        - borrowing is complex in multithreading. Two approaches:
            - explicit lifetimes (probably way to go)
                - job must not outlive a scheduler it was spawned for
                - all data referenced by job MUST outlive a scheduler
            - passed context
                - job is handed a immutable context reference on execution
                - context is prepared once and handed to scheduler at run
                Q: how to add context during work, like local sample context?
*/

mod bounce_quota;
mod bounce_type;
mod spectrum;
mod spectrum_wavelengths;

pub mod spds;

pub use self::bounce_quota::*;
pub use self::bounce_type::*;
pub use self::spectrum::*;
pub use self::spectrum_wavelengths::*;
