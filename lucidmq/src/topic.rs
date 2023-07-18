use log::debug;
use nolan::Commitlog;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str;
use std::{sync::atomic::AtomicU32, sync::Arc};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumerGroup {
    pub name: String,
    pub offset: AtomicU32,
}

impl ConsumerGroup {
    pub fn new(consumer_group_name: &str) -> ConsumerGroup {
        ConsumerGroup {
            name: consumer_group_name.to_string(),
            offset: 0.into(),
        }
    }

    pub fn _new_cg(consumer_group_name: &str, offset_in: AtomicU32) -> ConsumerGroup {
        ConsumerGroup {
            name: consumer_group_name.to_string(),
            offset: offset_in,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(from = "DeserTopic")]
pub struct Topic {
    pub name: String,
    pub directory: String,
    pub consumer_groups: Vec<Arc<ConsumerGroup>>,
    pub max_segment_size: u64,
    pub max_topic_size: u64,
    #[serde(skip_serializing)]
    pub commitlog: Commitlog,
}

#[derive(Deserialize)]
struct DeserTopic {
    name: String,
    directory: String,
    consumer_groups: Vec<Arc<ConsumerGroup>>,
    pub max_segment_size: u64,
    pub max_topic_size: u64,
}

impl From<DeserTopic> for Topic {
    fn from(tmp: DeserTopic) -> Self {
        let commitlog = nolan::Commitlog::new(
            &tmp.directory,
            tmp.max_segment_size,
            tmp.max_topic_size,
        )
        .expect("Unable to create commitlog for topic");
        Self {
            name: tmp.name,
            directory: tmp.directory,
            consumer_groups: tmp.consumer_groups,
            max_segment_size: tmp.max_segment_size,
            max_topic_size: tmp.max_topic_size,
            commitlog: commitlog,
        }
    }
}

impl Topic {
    pub fn new(
        topic_name: String,
        base_directory: String,
        max_segment_size: u64,
        max_topic_size: u64,
    ) -> Topic {
        debug!("Creating a new topic {}", topic_name);
        let path = Path::new(&base_directory);
        // Generate a random directory name
        let directory_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();
        let new_path = &path.join(directory_name);
        let new_consumer_groups = Vec::new();
        let new_commitlog = nolan::Commitlog::new(
            new_path
                .to_str()
                .expect("unable to convert to string"),
            max_segment_size,
            max_topic_size,
        )
        .expect("Unable to create commitlog for topic");
        Topic {
            name: topic_name,
            directory: new_path
                .to_str()
                .expect("unable to convert to string")
                .to_string(),
            consumer_groups: new_consumer_groups,
            commitlog: new_commitlog,
            max_segment_size: max_segment_size,
            max_topic_size: max_topic_size,
        }
    }

    pub fn load_consumer_group(&mut self, consumer_group_name: &str) -> Arc<ConsumerGroup> {
        for group in &self.consumer_groups {
            if group.name == consumer_group_name {
                return group.clone();
            }
        }
        let new_gc = Arc::new(ConsumerGroup::new(consumer_group_name));
        self.consumer_groups.push(new_gc.clone());
        new_gc
    }

    // pub fn _new_topic_from_ref(topic_ref: &Topic) -> Topic {
    //     let mut new_consumer_groups = Vec::new();
    //     for cg in &topic_ref.consumer_groups {
    //         new_consumer_groups.push(cg.clone());
    //     }
    //     let new_commitlog = nolan::Commitlog::new(topic_ref.directory.clone(), 1000, 100);
    //     Topic {
    //         name: topic_ref.name.clone(),
    //         directory: topic_ref.directory.clone(),
    //         consumer_groups: new_consumer_groups,
    //         commitlog: new_commitlog
    //     }
    // }

    pub fn get_consumer_groups(&self) -> Vec<String> {
        let cg_names = self
            .consumer_groups
            .iter()
            .map(|x| x.name.clone())
            .collect();
        return cg_names;
    }

    pub fn get_max_segment_size(&self) -> u64 {
        self.max_segment_size
    }
}


pub struct SimpleTopic {
    pub topic_name: String,
    pub consumer_groups: Vec<String>
}