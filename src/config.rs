pub trait Config {

}

enum Activation {
    RELU
}

struct MLPLayerConfiguration {
    activation: Option<Activation>,
    weight_init: bool
}

pub struct MLPLayerConfigurationBuilder {
    activation: Option<Activation>,
    weight_init: bool
}

impl MLPLayerConfigurationBuilder {

    pub fn builder() -> Self {
        MLPLayerConfigurationBuilder {
            activation: None,
            weight_init: false
        }
    }

    pub fn weight_init(&mut self) -> &mut MLPLayerConfigurationBuilder {
        self.weight_init = true;
        self
    }

    pub fn activation(&mut self) -> &mut MLPLayerConfigurationBuilder {
        self.activation = Some(Activation::RELU);
        self
    }

    pub fn build(&mut self) -> MLPLayerConfiguration {
        MLPLayerConfiguration {
            activation: self.activation,
            weight_init: self.weight_init
        }
    }
}
