pub trait Config {

}

struct MLPLayerConfiguration {

    activation: Option<Activation::RELU>
}

struct MLPLayerConfigurationBuilder {}

impl MLPLayerConfigurationBuilder {

    pub fn builder() -> Self {
        MLPLayerConfigurationBuilder {}
    }

    pub fn weight_init() -> Self {
        MLPLayerConfigurationBuilder {}
    }

    pub fn build() -> MLPLayerConfiguration {
        MLPLayerConfiguration {}
    } 
}
