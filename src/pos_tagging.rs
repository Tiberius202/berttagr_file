// Copyright 2019-present, the HuggingFace Inc. team, The Google AI Language Team and Facebook, Inc.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Part Of Speech pipeline
//! Extracts Part of Speech tags (Noun, Verb, Adjective...) from text.
//! Mostly copied off of a lightweight pretrained model using MobileBERT 
//! from the rust-bert library

use rust_bert::RustBertError;
use rust_bert::mobilebert::{
    MobileBertConfigResources, MobileBertModelResources, MobileBertVocabResources,
};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::token_classification::{
    LabelAggregationOption, TokenClassificationConfig, TokenClassificationModel,
};
use rust_bert::resources::{RemoteResource, Resource};
use tch::Device;

#[derive(Debug)]
/// # Part of Speech tag
pub struct POSTag {
    /// String representation of the word
    pub word: String,
    /// Part-of-speech label (e.g. NN, VB...)
    pub label: String,
}

//type alias for some backward compatibility
pub struct POSConfig {
    token_classification_config: TokenClassificationConfig,
}

impl Default for POSConfig {
    /// Provides a Part of speech tagging model (English)
    fn default() -> POSConfig {
        POSConfig {
            token_classification_config: TokenClassificationConfig {
                model_type: ModelType::MobileBert,
                model_resource: Resource::Remote(RemoteResource::from_pretrained(
                    MobileBertModelResources::MOBILEBERT_ENGLISH_POS,
                )),
                config_resource: Resource::Remote(RemoteResource::from_pretrained(
                    MobileBertConfigResources::MOBILEBERT_ENGLISH_POS,
                )),
                vocab_resource: Resource::Remote(RemoteResource::from_pretrained(
                    MobileBertVocabResources::MOBILEBERT_ENGLISH_POS,
                )),
                merges_resource: None,
                lower_case: true,
                strip_accents: Some(true),
                add_prefix_space: None,
                device: Device::cuda_if_available(),
                label_aggregation_function: LabelAggregationOption::First,
            },
        }
    }
}

impl From<POSConfig> for TokenClassificationConfig {
    fn from(pos_config: POSConfig) -> Self {
        pos_config.token_classification_config
    }
}

/// # POSModel to extract Part of Speech tags
pub struct POSModel {
    token_classification_model: TokenClassificationModel,
}

impl POSModel {
    /// Build a new `POSModel`
    ///
    /// # Arguments
    ///
    /// * `pos_config` - `POSConfig` object containing the resource references (model, vocabulary, configuration) and device placement (CPU/GPU)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() -> anyhow::Result<()> {
    /// use rust_bert::pipelines::pos_tagging::POSModel;
    ///
    /// let pos_model = POSModel::new(Default::default())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(pos_config: POSConfig) -> Result<POSModel, RustBertError> {
        let model = TokenClassificationModel::new(pos_config.into())?;
        Ok(POSModel {
            token_classification_model: model,
        })
    }

    /// Extract entities from a text
    ///
    /// # Arguments
    ///
    /// * `input` - `&[&str]` Array of texts to extract entities from.
    ///
    /// # Returns
    ///
    /// * `Vec<Vec<POSTag>>` containing Part of Speech tags for the inputs provided
    ///
    /// # Example
    ///
    /// ```no_run
    /// # fn main() -> anyhow::Result<()> {
    /// # use rust_bert::pipelines::pos_tagging::POSModel;
    ///
    /// let pos_model = POSModel::new(Default::default())?;
    /// let input = [
    ///     "My name is Amy. I live in Paris.",
    ///     "Paris is a city in France.",
    /// ];
    /// let output = pos_model.predict(&input);
    /// # Ok(())
    /// # }
    /// ```
    pub fn predict<'a, S>(&self, input: S) -> Vec<Vec<POSTag>>
    where
        S: AsRef<[&'a str]>,
    {
        self.token_classification_model
            .predict(input, true, false)
            .into_iter()
            .map(|sequence_tokens| {
                sequence_tokens
                    .into_iter()
                    .map(|mut token| {
                        if (Self::is_punctuation(token.text.as_str()))
                            & ((token.score < 0.5) | token.score.is_nan())
                        {
                            token.label = String::from(".");
                            token.score = 1f64;
                        };
                        token
                    })
                    .map(|token| POSTag {
                        word: token.text,
                        label: token.label,
                    })
                    .collect::<Vec<POSTag>>()
            })
            .collect::<Vec<Vec<POSTag>>>()
    }

    fn is_punctuation(string: &str) -> bool {
        string.chars().all(|c| c.is_ascii_punctuation())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore] // no need to run, compilation is enough to verify it is Send
    fn test() {
        let config = POSConfig::default();
        let _: Box<dyn Send> = Box::new(POSModel::new(config));
    }
}
