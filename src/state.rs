use std::{cell::RefCell, rc::Rc};

use futures::{stream, StreamExt};
use reqwest::Client;
use tokio::task::spawn_local;
use yew::suspense::Suspension;

use crate::provider::Provider;

const PARALLEL_REQUESTS: usize = 3;

pub struct State {
    pub susp: Suspension,
    pub value: Rc<RefCell<Option<Vec<String>>>>,
}

impl State {
    pub fn new(search: &str) -> Self {
        let (susp, handle) = Suspension::new();
        let value: Rc<RefCell<Option<Vec<String>>>> = Rc::default();

        {
            let value = value.clone();
            // we need to own to spawn local
            let s = search.to_owned();
            // we use tokio spawn local here.
            spawn_local(async move {
                let res = Self::fetch(&s);
                {
                    let mut value = value.borrow_mut();
                    *value = Some(res.await);
                }

                handle.resume();
            });
        }

        Self { susp, value }
    }

    // we need an owned string because we're sending it to other threads
    async fn fetch(search: &str) -> Vec<String> {
        let client = Client::new();

        let providers = vec![Provider::BIKE_DISCOUNT, Provider::ALLTRICKS, Provider::STARBIKE];
        let providers_len = providers.len();

        let results = stream::iter(providers)
            .map(|p| {
                // should be safe to clone since backed by an rc (to check)
                let client = client.clone();
                // corouting moves
                let s = search.to_owned();
                tokio::spawn(async move {
                    p.crawl(&client, &s).await.unwrap_or("not found".to_owned())
                })
            })
            .buffer_unordered(PARALLEL_REQUESTS)
            .take(providers_len)
            .collect::<Vec<_>>()
            .await;

        results
            .into_iter()
            .map(|r| match r {
                Ok(s) => Some(s),
                Err(e) => {
                    println!("join error: {e}");
                    None
                },
            })
            .flatten()
            .collect()
    }
}