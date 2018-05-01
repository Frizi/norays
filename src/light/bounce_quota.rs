use light::BounceType;

#[derive(Clone, Copy)]
pub struct BounceQuota {
    diffuse: u16,
    glossy: u16,
    transmission: u16,
    volume: u16,
}

macro_rules! impl_bounce_attempt {
    ($self:ident, $field:ident) => {
        if $self.$field > 0 {
            Some(Self {
                $field: $self.$field - 1,
                ..*$self
            })
        } else {
            None
        }
    };
}

impl BounceQuota {
    pub fn new(diffuse: u16, glossy: u16, transmission: u16, volume: u16) -> Self {
        Self {
            diffuse,
            glossy,
            transmission,
            volume,
        }
    }

    pub fn attempt(&self, bounce: BounceType) -> Option<Self> {
        match bounce {
            BounceType::Diffuse => impl_bounce_attempt!(self, diffuse),
            BounceType::Glossy => impl_bounce_attempt!(self, glossy),
            BounceType::Transmission => impl_bounce_attempt!(self, transmission),
            BounceType::Volume => impl_bounce_attempt!(self, volume),
        }
    }
}
