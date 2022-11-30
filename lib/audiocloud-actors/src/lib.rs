/*
 * Copyright (c) Audio Cloud, 2022. This code is licensed under MIT license (see LICENSE for details)
 */
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::time::Duration;

use coerce::actor::message::{Handler, Message};
use coerce::actor::{Actor, ActorRef};
use tokio::spawn;
use tokio::sync::broadcast::{Receiver as BroadcastReceiver, Sender as BroadcastSender};
use tokio::task::JoinHandle;
use tracing::trace;

pub fn schedule_interval<A: Actor, M: Message + Copy>(actor: impl Into<ActorRef<A>>, interval: Duration, msg: M) -> JoinHandle<()>
    where A: Handler<M>
{
    let actor = actor.into();
    spawn(async move {
        let mut interval = tokio::time::interval(interval);
        loop {
            interval.tick().await;
            if let Err(error) = actor.send(msg).await {
                trace!(%error, "Sending to actor failed");
                break;
            }
        }
    })
}

pub fn schedule_interval_counted<A: Actor, M: Message, G, R>(actor: R, interval: Duration, generator: G) -> JoinHandle<()>
    where A: Handler<M>,
          G: Fn(u64) -> M + Send + Sync + 'static,
          R: Into<ActorRef<A>>
{
    let actor = actor.into();
    spawn(async move {
        let mut counter = 0;
        let mut interval = tokio::time::interval(interval);
        loop {
            interval.tick().await;
            if let Err(error) = actor.send(generator(counter)).await {
                trace!(%error, "Sending to actor failed");
                break;
            }
            counter += 1;
        }
    })
}

pub fn subscribe<A: Actor, M: Message + Clone>(actor: impl Into<ActorRef<A>>, mut receiver: BroadcastReceiver<M>) -> JoinHandle<()>
    where A: Handler<M>
{
    let actor = actor.into();

    spawn(async move {
        loop {
            match receiver.recv().await {
                Ok(message) => match actor.send(message).await {
                    Ok(_) => {}
                    Err(error) => {
                        trace!(%error, "Sending to actor failed");
                        break;
                    }
                },
                Err(error) => {
                    trace!(%error, "Subscription to broadcast channel closed");
                    break;
                }
            }
        }
    })
}

pub fn subscribe_to_sender<A: Actor, M: Message + Clone>(actor: impl Into<ActorRef<A>>, sender: &BroadcastSender<M>) -> JoinHandle<()>
    where A: Handler<M>
{
    subscribe(actor, sender.subscribe())
}
