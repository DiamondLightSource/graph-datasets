use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};

use models::data_collection;

#[derive(Debug, SimpleObject)]
pub struct DataCollection {
    /// An opaque unique identifier for the data collection
    pub data_collection_id: u32,
    /// An opaque unique identifier for the session
    pub sessionid: Option<u32>,
    /// The date time and which data collection began
    pub start_time: Option<DateTime<Utc>>,
    /// The date time and which data collection ended
    pub end_time: Option<DateTime<Utc>>,
    /// Number of images taken during data collection
    pub number_of_images: Option<u32>,
    /// Sample exposure time on the beamline
    pub exposure_time: Option<f32>,
    /// Wavelength of the beamline
    pub wavelength: Option<f32>,
    /// The distance of the detector from the sample
    pub detector_distance: Option<f32>,
    /// Beam X position
    pub x_beam: Option<f32>,
    /// Beam Y position
    pub y_beam: Option<f32>,
    /// Comments during data collection
    pub comments: Option<String>,
    /// Beam size at sample at X position
    pub beam_size_at_sample_x: Option<f32>,
    /// Beam size at sample at Y position
    pub beam_size_at_sample_y: Option<f32>,
    /// An opaque unique identifier for the data collection group
    pub data_collection_group_id: i32,
    /// An opaque unique identifier for the detector
    pub detector_id: Option<i32>,
    /// Location of the image stored 
    pub image_directory: Option<String>, 
    /// Image file name without extension
    pub image_suffix: Option<String>,
    /// Image file extension
    pub image_prefix: Option<String>, 
}

impl From<data_collection::Model> for DataCollection {
    fn from(values: data_collection::Model) -> Self {
        Self {
            data_collection_id: values.data_collection_id,
            sessionid: values.sessionid,
            start_time: values.start_time.map(|time| time.and_utc()),
            end_time: values.end_time.map(|time| time.and_utc()),
            number_of_images: values.number_of_images,
            exposure_time: values.exposure_time,
            wavelength: values.wavelength,
            detector_distance: values.detector_distance,
            x_beam: values.x_beam,
            y_beam: values.y_beam,
            comments: values.comments,
            beam_size_at_sample_x: values.beam_size_at_sample_x,
            beam_size_at_sample_y: values.beam_size_at_sample_y,
            data_collection_group_id: values.data_collection_group_id,
            detector_id: values.detector_id,
            image_directory: values.image_directory,
            image_suffix: values.image_suffix, 
            image_prefix: values.image_prefix,
        }
    }
}
