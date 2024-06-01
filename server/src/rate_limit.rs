use std::{convert::Infallible, hash::RandomState, net::{IpAddr, SocketAddr}, time::{Duration, Instant}};

use anyhow::Result;
use moka::future::Cache;
use warp::{filters::BoxedFilter, reject::{custom, Reject, Rejection}, Filter};

#[derive(Copy, Clone)]
pub struct Rate {
    num: u32,
    per: Duration,
}

impl Rate {
    pub fn new(num: u32, per: Duration) -> Self {
        assert!(num > 0);
        Self { num, per }
    }
}

#[derive(Copy, Clone)]
pub struct State {
    until: Instant,
    rem: u32,
}

impl State {
    pub fn new(rate: Rate) -> Self {
        Self {
            until: Instant::now() + rate.per,
            rem: rate.num,
        }
    }
}

#[derive(Debug)]
pub struct RateLimited {
    pub remaining_duration: Duration,
}

impl Reject for RateLimited {}

pub fn rate_limit(rate: Rate) -> BoxedFilter<((),)> {
    let cache: Cache<Option<IpAddr>, State, RandomState> =
        Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(3600))
            .build();

    warp::any()
        .and(warp::addr::remote())
        .and(with_rate_limit_cache(cache))
        .and_then(move |
            addr: Option<SocketAddr>,
            cache: Cache<Option<IpAddr>, State, RandomState>,
        | {
            compute_rate_limit(cache, addr, rate)
        })
        .boxed()
}

pub fn with_rate_limit_cache(
    cache: Cache<Option<IpAddr>, State, RandomState>,
) -> impl Filter<
    Extract = (Cache<Option<IpAddr>, State, RandomState>,),
    Error = Infallible
> + Clone {
    warp::any().map(move || cache.clone())
}

async fn compute_rate_limit(
    cache: Cache<Option<IpAddr>, State, RandomState>,
    addr: Option<SocketAddr>,
    rate: Rate,
) -> Result<(), Rejection> {
    let now = Instant::now();

    let ip = addr.map(|addr| addr.ip());

    let mut state = cache.get(&ip).await.unwrap_or(State::new(rate));

    if now >= state.until {
        state.until = now + rate.per;
        state.rem = rate.num;
    }

    let res = if state.rem > 1 {
        state.rem -= 1;
        Ok(())
    } else {
        Err(custom(RateLimited { remaining_duration: state.until - now } ))
    };

    cache.insert(ip, state).await;

    res
}