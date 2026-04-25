//! The engine is split in two sides, that work on two main threads :
//! - the processing thread - initiated by the cpal audio card binding, pulling data through the pipeline graph.
//! - the topology thread - which allows to modify the topology of the pipeline graph in real time without concurrency.
//! 
//! Besides, more threads can run individually to push data in the pipeline, in a push-pull fashion, like decoder
//! which benefits from running on their own threads as decoding can be quite expansive, which might create tension
//! with the cpal thread expecting a steady data flow.
pub mod process;
pub mod topology;