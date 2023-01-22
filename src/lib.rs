//! This crate provides a client for interacting with the Civo API. 

/// Module containing the client definition and related helper function 
pub mod client;

/// Module containing errors that could be returned by the API
pub mod errors;

/// Module containing instance definitions and related functions
pub mod instance;

/// Module for retreiving instance sizes on civo 
pub mod instance_size;

/// Module containing network related structs and functions 
pub mod network;

/// Module containing Civo region related objects 
pub mod region;

/// Module for managing ssh keys 
pub mod ssh_key;

/// Module for managing disk images,  useful when working with instances 
pub mod disk_image;

/// Module for managing Kubernetes clusters 
pub mod kubernetes;

/// Module cotaining misc helper fucntions including the heroku style name generator 
pub mod utils;

/// Module for managing Objectstores 
pub mod objectstore;
